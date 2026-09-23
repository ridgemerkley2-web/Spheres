"""CLAUDE-C01-10: Brazil's presidents, 1990-2026, keep election, diplomação, posse, suspension, the Vice-President's
exercise, resignation, removal and disqualification apart, and state a start or an end only where a source does."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256). Every new source is reproducible.
RESPONSES = {
    'br_planalto_decreto_98797_19900105':
        (4920, 'd87cf71bfe8c916ae8cc1cf428a5eb66e9ed312dd76d05a018684de5ae8d62d8'),
    'br_planalto_decreto_99167_19900313':
        (14155, '28fe230405f4c5405e3b0bc7a757b6b4708458128160f2d0161caa33a02ec69f'),
    'br_bibpr_sarney_page_2016':
        (45127, '4f1f6d9613741b59871f922d66f63f9f51e5728443e5bf3aa8a48d97f7b0b34d'),
    'br_tse_diploma_collor_19891230':
        (3529917, '9db4c926dd93fd9d27535de2287c2ebfac084fafc77d4031aa5ca52f73a75427'),
    'br_tse_catalogue_diplomacao_1989':
        (5194, 'ade970724841f69ef18cd39797bc191e8e83adfbf7f0d98a826103fed9eda4b0'),
    'br_dcn_08_19900316':
        (1255764, '838abf03a2ef59781aabc66121dd9fd938ed78ebd15df36a6f0c02c70c7b6645'),
    'br_dcd_161_19920930':
        (11721640, '20b17a8baf815b7a6e343e9ca88f71657fe71426ed80d88ed84bfb0ac763aed6'),
    'br_dsf_163_19921001':
        (3889678, 'd1d0273def8b3b568de0861f0c5fd8ecc62698d46cff17e9b52232223e8fb281'),
    'br_dsf_164_19921002':
        (4417869, '3df141471b58bda113e1cbe251cba681bb133b1a2a0c2378c274fb51e63d89c7'),
    'br_planalto_decreto_663_19921001':
        (14428, '9da119b4fa18a10924cd7a85be1d03202c582d50fef0ec74be3132586846147e'),
    'br_senado_autos_impeachment_vol1':
        (47475991, '3e4ad563f4bfe236aaa4a506915c5ede3403b142b891b736a77527f1447d83be'),
    'br_planalto_lei_8471_19921007':
        (5010, '39e80d34cdc243323b61047c29b3ce1406405db6e8a7f6d126c2a94f3d2928b1'),
    'br_dcn_70_19921230':
        (397945, 'dfe8e27ab3045795d36504ed15aa4ef85a914cebe42682756e1a0cd2ba020d33'),
    'br_dsf_223_suplemento_19921230':
        (24080741, '563c6a98cdcc50a3b746c028d269603040588bf0599edd01152e81f5deffdf74'),
    'br_dsf_224_19921231':
        (1087012, 'f6d9477c216bb40e4a3c80f3961c173ddc8fd0324b9cfb8bd9d9d4da43c07147'),
    'br_bibpr_collor_page_2016':
        (42045, 'e0ad5fdb5c40149d18b7ea5237c20e3cc1aa60790b6f59beec75cf03d3cbf49d'),
    'br_bibpr_itamar_page_2016':
        (43437, '9f67a1a48b8cee7f8d6025a350d8c4969e1122baa6d6da44d96596e752e2ac22'),
    'br_dcn_1_1995_posse_19950101':
        (845143, 'fb7ff24416c6bcf6eb0d66ed6d6316fb6ce7ca1ed278eaacea1a0f92f3abc5c5'),
    'br_dcn_1_1999_posse_19990101':
        (939787, 'b7c0283e5691a42520189f974b743c548c2dd8db56d6a9a090b8e20e4a2f2abb'),
    'br_dcn_1_2003_posse_20030101':
        (1212635, '86f2d3923f091eda17f7eada3e081b00f5a12cdbfb385774ba7265bc6cbca32d'),
    'br_dcn_1_2007_posse_20070101':
        (1829092, '395e155d2c05f83c2e51380e9181769dd48b73e7f1ca7e698d865f9094d28822'),
    'br_dcn_1_2011_posse_20110101':
        (506362, 'b75d4e990a0cc00e27e570bf7ed8658194c4c78a241382e4b27ccb75c0baccbb'),
    'br_planalto_library_dilma_parlatorio_20110101':
        (37635, '3476ba0719dfa0e8a2e19ed4dd0dcc3221a258ba0825fa62f8b7be90bd94272a'),
    'br_dcn_1_2015_posse_20150101':
        (8131269, '8f588186c81de17bd8c7009f6ad37322edfc295eda788e33b4afd579fb861787'),
    'br_planalto_dilma_parlatorio_20150101':
        (63136, '2cbd5e75e8d55355bacb34dc5b4fec182cb326e64ac050be82e8fafbbf25947a'),
    'br_camara_dcd_20160418':
        (2515996, '266a43a0b416c4504b2a87f2672f36c22c5bb5a718546add200a9b0d554a37c2'),
    'br_senado_dsf64_20160512_p166_171':
        (27973809, 'c98dafbbdf9a07852d01da0c7f933275a12b6a1758f33020532d2771967b8c5d'),
    'br_agsen_20160512_senado_abre_processo':
        (121960, '61689374fc4b3aab66b0f6541fed5ce0d4c481b2bb34f75b6d3674a10a13b0be'),
    'br_senado_rousseff_contrafe_20160512':
        (311449, '41f9c9d8791458ec54b6f5488d13c6dfcc0a242ee9155a9053743910f8a41b97'),
    'br_radio_senado_notifications_20160512':
        (103027, '8fa440b7292db4514393d7ad28e262200beb140265185f86ff0dec9a7d0f86dd'),
    'br_senado_den1_2016_materia_20260508':
        (1097044, '0880869ddf624f43d3079079184506bfe0b34a293445bf7639410b8d025ee7c3'),
    'br_planalto_mpv726_20160512':
        (144547, '32c843c5ff8fa6dbe0351a2331cddf4381f254afc7854c8b700678f802876c84'),
    'br_senado_den1_2016_vote_loss_of_office':
        (156974, '7c0e3097345a4db13e08c066ff0c2c9fbfffd7eccb1b1246e12dc3278aa33e24'),
    'br_senado_den1_2016_vote_disqualification':
        (151651, 'a5538be78243d774c701826c3281c0814f7a27599bfa93516e70ad127d0e1152'),
    'br_senado_den1_2016_resolucao35':
        (37070, '6c0e65e98f21c5546897b6e7686178206f5d786f5cb2e2097e8394604a2c03fe'),
    'br_senado_den1_2016_dou_20160831_extra':
        (192778, '0096f80f529fbb4906a36ad5736ed569fce02663be7c421094a30a2815a24dd8'),
    'br_senado_den1_2016_mensagem144':
        (61472, 'ad1913ece01049dde93ceff3d584d1c53af99ae1cb0985f181a654b07d3f0535'),
    'br_cn_dcn15_20160901':
        (8585228, '60e2c85c00bf0215c934de795ca0c98a107ab9fc6bfe8ec29128032241824195'),
    'br_cn_dcn1_20190102_p5_11':
        (49257639, '2d8a63d500ff5ffc82d690a1dc27c3239e25ece96511550e5a82398fc90b24eb'),
    'br_cn_dcn1_20190102_full':
        (49257639, '2d8a63d500ff5ffc82d690a1dc27c3239e25ece96511550e5a82398fc90b24eb'),
    'br_planalto_mpv870_20190101':
        (241409, '98d89a0849b1226fc440e1629a6d3cfd4ef8226edf679a29ca63ef42ab00103e'),
    'br_planalto_bolsonaro_sash_speech_20190101':
        (192253, '7e6995acfc8bcc5968b85a59b13aafa07581057f26a7996a5ab700c43b2fa19d'),
    'br_planalto_d11322_20221230':
        (16059, '582e464c71f64c0891f8d6c04fa6bd6162feb9b2802c374d6e6bd0d0d6f44e34'),
    'br_planalto_d11324_20221231':
        (15704, 'b791adf5b078361eb6e2aab7fdf042382d4f07f05229134e3b1d3e21c9f3bd16'),
    'br_cn_dcn1_20230102_p1_8':
        (24950218, 'd6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee'),
    'br_cn_dcn1_20230102_p18_19':
        (24950218, 'd6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee'),
    'br_planalto_mpv1154_20230101':
        (314530, '61b3b64182304109f5941e493ec1ff44843f8ebd8e049ab542bca90410ea1315'),
    'br_planalto_mpv1388_20260824':
        (45429, '8b9eeda35e902ba5e558d4c1834f533d41e01041a0ff2243bcf9f704916ed3f5'),
}
ARCHIVED = {
    'br_planalto_decreto_98797_19900105': '20120624014148',
    'br_planalto_decreto_99167_19900313': '20150221092510',
    'br_bibpr_sarney_page_2016': '20160814001316',
    'br_planalto_decreto_663_19921001': '20081026122235',
    'br_planalto_lei_8471_19921007': '20081014095512',
    'br_bibpr_collor_page_2016': '20160812222130',
    'br_bibpr_itamar_page_2016': '20160813192944',
    'br_planalto_library_dilma_parlatorio_20110101': '20260123082728',
    'br_planalto_dilma_parlatorio_20150101': '20150111173546',
    'br_agsen_20160512_senado_abre_processo': '20160513123843',
    'br_radio_senado_notifications_20160512': '20160513105044',
    'br_senado_den1_2016_materia_20260508': '20260508123130',
    'br_planalto_mpv726_20160512': '20160517063546',
    'br_planalto_mpv870_20190101': '20190103091608',
    'br_planalto_bolsonaro_sash_speech_20190101': '20220305142422',
    'br_planalto_d11322_20221230': '20221231142723',
    'br_planalto_d11324_20221231': '20230101180620',
    'br_planalto_mpv1154_20230101': '20230102110823',
    'br_planalto_mpv1388_20260824': '20260826144228',
}
PDF_PAGES = {
    'br_dcn_08_19900316': [1, 3, 4],
    'br_dcd_161_19920930': [1, 57],
    'br_dsf_163_19921001': [1, 3],
    'br_dsf_164_19921002': [15],
    'br_senado_autos_impeachment_vol1': [1, 786, 787, 788, 791],
    'br_dcn_70_19921230': [3, 6],
    'br_dsf_223_suplemento_19921230': [1, 12, 235, 236],
    'br_dsf_224_19921231': [1, 2, 3, 4],
    'br_dcn_1_1995_posse_19950101': [1, 5, 6, 7],
    'br_dcn_1_1999_posse_19990101': [3, 4, 6, 9, 13],
    'br_dcn_1_2003_posse_20030101': [3, 4, 6, 7, 13],
    'br_dcn_1_2007_posse_20070101': [4, 8, 9],
    'br_dcn_1_2011_posse_20110101': [1, 4, 8, 9],
    'br_dcn_1_2015_posse_20150101': [1, 4, 6, 7, 10, 11],
    'br_camara_dcd_20160418': [1, 120],
    'br_senado_dsf64_20160512_p166_171': [167, 168],
    'br_senado_den1_2016_vote_loss_of_office': [1, 3],
    'br_senado_den1_2016_vote_disqualification': [1, 3],
    'br_senado_den1_2016_resolucao35': [1],
    'br_senado_den1_2016_dou_20160831_extra': [1, 2],
    'br_senado_den1_2016_mensagem144': [1, 2],
    'br_cn_dcn15_20160901': [4, 6],
    'br_cn_dcn1_20190102_p5_11': [6, 7],
    'br_cn_dcn1_20190102_full': [14, 20],
    'br_cn_dcn1_20230102_p1_8': [1, 6, 7],
    'br_cn_dcn1_20230102_p18_19': [18, 19],
}
EVENTS = {
    'br_sarney_signs_decreto_98797_19900105': ('1990-01-05', 'in_office_attestation'),
    'br_sarney_signs_decreto_99167_19900313': ('1990-03-13', 'in_office_attestation'),
    'br_bibpr_sarney_government_period_19850315_19900315': (None, 'retrospective_term_span'),
    'br_collor_tse_diploma_issued_19891230': ('1989-12-30', 'diplomacao'),
    'br_collor_elected_ballots_19891217': ('1989-12-17', 'popular_election'),
    'br_tse_diplomacao_ceremony_catalogued_19891230': ('1989-12-30', 'diplomacao_ceremony'),
    'br_collor_posse_declared_congress_19900315': ('1990-03-15', 'posse_declared'),
    'br_collor_termo_de_posse_19900315': ('1990-03-15', 'posse_record'),
    'br_collor_assumes_presidency_statement_19900315': ('1990-03-15', 'assumption_statement'),
    'br_collor_diplomacao_recited_19891230': ('1989-12-30', 'diplomacao'),
    'br_collor_second_round_recited_19891217': ('1989-12-17', 'popular_election'),
    'br_chamber_authorizes_impeachment_process_19920929': ('1992-09-29', 'chamber_authorization'),
    'br_chamber_authorization_read_in_senate_19920930': ('1992-09-30', 'chamber_authorization_received'),
    'br_senate_approves_process_opening_opinion_19921001': ('1992-10-01', 'senate_trial_admission'),
    'br_collor_signs_decreto_663_19921001': ('1992-10-01', 'in_office_attestation'),
    'br_senate_mesa_formalizes_summons_19921001': ('1992-10-01', 'summons_issued'),
    'br_senate_summons_mandado_suspension_terms_19921001': ('1992-10-01', 'summons_issued'),
    'br_vice_president_instructed_to_assume_19921001': ('1992-10-01', 'vice_president_notified_to_exercise'),
    'br_collor_receives_summons_suspended_19921002': ('1992-10-02', 'suspension'),
    'br_itamar_exercising_signs_lei_8471_19921007': ('1992-10-07', 'vice_president_exercising_office'),
    'br_collor_resignation_letter_read_congress_19921229': ('1992-12-29', 'resignation_letter_read'),
    'br_presidency_vacancy_declared_19921229': ('1992-12-29', 'vacancy_declared'),
    'br_itamar_posse_declared_congress_19921229': ('1992-12-29', 'posse_declared'),
    'br_itamar_termo_de_posse_19921229': ('1992-12-29', 'posse_record'),
    'br_collor_resignation_occurred_19921229': ('1992-12-29', 'resignation'),
    'br_itamar_states_exercise_from_19921002': ('1992-10-02', 'vice_president_exercise_start'),
    'br_senate_resolution_101_first_printed_19921230': ('1992-12-30', 'resolution_published'),
    'br_collor_resignation_letter_read_senate_19921229': ('1992-12-29', 'resignation_letter_read'),
    'br_senate_votes_to_continue_trial_19921229': ('1992-12-29', 'senate_trial_continuation_vote'),
    'br_senate_disqualification_vote_19921230': ('1992-12-30', 'disqualification_vote'),
    'br_collor_resignation_letter_printed_19921229': ('1992-12-29', 'resignation'),
    'br_senate_impeachment_sentence_19921230': ('1992-12-30', 'senate_judgment'),
    'br_senate_resolution_101_disqualification_19921230': ('1992-12-30', 'disqualification_from_public_function'),
    'br_bibpr_collor_first_phase_19900315_19921002': (None, 'retrospective_term_span'),
    'br_bibpr_itamar_phase_19921002_period_19921229': (None, 'retrospective_term_span'),
    'br_fhc_elected_first_round_19941003': ('1994-10-03', 'popular_election'),
    'br_fhc_diplomado_tse_19941217': ('1994-12-17', 'diplomacao'),
    'br_fhc_oath_before_congress_19950101': ('1995-01-01', 'oath_of_office'),
    'br_fhc_posse_declared_19950101': ('1995-01-01', 'posse_declared'),
    'br_fhc_termo_de_posse_19950101': ('1995-01-01', 'posse_record'),
    'br_fhc_assumes_position_today_19950101': ('1995-01-01', 'assumption_statement'),
    'br_itamar_leaves_government_fhc_address_19950101': ('1995-01-01', 'predecessor_departure_statement'),
    'br_fhc_reelected_first_round_19981004': ('1998-10-04', 'popular_election'),
    'br_fhc_diplomado_tse_19981212': ('1998-12-12', 'diplomacao'),
    'br_fhc_oath_before_congress_19990101': ('1999-01-01', 'oath_of_office'),
    'br_fhc_posse_declared_19990101': ('1999-01-01', 'posse_declared'),
    'br_fhc_termo_de_posse_19990101': ('1999-01-01', 'posse_record'),
    'br_fhc_second_mandate_assumed_acm_address_19990101': ('1999-01-01', 'assumption_statement'),
    'br_lula_elected_20021027': ('2002-10-27', 'popular_election'),
    'br_lula_diplomado_tse_20021214': ('2002-12-14', 'diplomacao'),
    'br_lula_oath_before_congress_20030101': ('2003-01-01', 'oath_of_office'),
    'br_lula_posse_declared_20030101': ('2003-01-01', 'posse_declared'),
    'br_lula_termo_de_posse_20030101': ('2003-01-01', 'posse_record'),
    'br_lula_assumes_presidency_first_secretary_20030101': ('2003-01-01', 'assumption_statement'),
    'br_lula_reelected_20061029': ('2006-10-29', 'popular_election'),
    'br_lula_diplomado_tse_20061214': ('2006-12-14', 'diplomacao'),
    'br_lula_oath_before_congress_20070101': ('2007-01-01', 'oath_of_office'),
    'br_lula_posse_declared_20070101': ('2007-01-01', 'posse_declared'),
    'br_lula_termo_de_posse_20070101': ('2007-01-01', 'posse_record'),
    'br_lula_new_mandate_inaugural_day_20070101': ('2007-01-01', 'assumption_statement'),
    'br_lula_assumption_recalled_20070101': ('2003-01-01', 'assumption_statement_retrospective'),
    'br_dilma_elected_20101031': ('2010-10-31', 'popular_election'),
    'br_dilma_diplomada_tse_20101217': ('2010-12-17', 'diplomacao'),
    'br_dilma_oath_before_congress_20110101': ('2011-01-01', 'oath_of_office'),
    'br_dilma_posse_declared_20110101': ('2011-01-01', 'posse_declared'),
    'br_dilma_termo_de_posse_20110101': ('2011-01-01', 'posse_record'),
    'br_dilma_sash_announced_congress_address_20110101': ('2011-01-01', 'sash_transfer_prospective'),
    'br_dilma_assumes_responsibility_sarney_address_20110101': ('2011-01-01', 'assumption_statement'),
    'br_dilma_assumes_government_parlatorio_20110101': ('2011-01-01', 'assumption_statement'),
    'br_lula_leaves_government_dilma_parlatorio_20110101': ('2011-01-01', 'predecessor_departure_statement'),
    'br_dilma_reelected_20141026': ('2014-10-26', 'popular_election'),
    'br_dilma_diplomada_tse_20141218': ('2014-12-18', 'diplomacao'),
    'br_dilma_oath_before_congress_20150101': ('2015-01-01', 'oath_of_office'),
    'br_dilma_posse_declared_20150101': ('2015-01-01', 'posse_declared'),
    'br_dilma_termo_de_posse_20150101': ('2015-01-01', 'posse_record'),
    'br_dilma_assumes_second_mandate_parlatorio_20150101': ('2015-01-01', 'assumption_statement'),
    'br_camara_authorizes_senate_trial_rousseff_20160417': ('2016-04-17', 'chamber_authorization'),
    'br_senate_admits_denuncia_rousseff_20160512': ('2016-05-12', 'senate_trial_admission'),
    'br_senate_intimation_order_suspension_terms_20160512': ('2016-05-12', 'summons_issued'),
    'br_agsen_senate_opens_process_0634_20160512': ('2016-05-12', 'senate_trial_admission'),
    'br_rousseff_receives_intimation_suspended_20160512': ('2016-05-12', 'suspension'),
    'br_radio_senado_notifications_served_20160512': ('2016-05-12', 'summons_served_report'),
    'br_senate_record_intimation_receipt_signed_20160512': ('2016-05-12', 'summons_receipt_filed'),
    'br_senate_record_vp_notified_20160512': ('2016-05-12', 'vice_president_notified_to_exercise'),
    'br_temer_vp_in_exercise_mpv726_20160512': ('2016-05-12', 'vice_president_exercising_office'),
    'br_senate_vote_loss_of_office_20160831': ('2016-08-31', 'senate_judgment_vote'),
    'br_senate_vote_disqualification_20160831': ('2016-08-31', 'disqualification_vote'),
    'br_senate_resolution35_removal_rousseff_20160831': ('2016-08-31', 'removal_resolution'),
    'br_dou_publishes_resolution35_20160831': ('2016-08-31', 'removal_in_force'),
    'br_sentence_records_suspension_20160512': ('2016-05-12', 'suspension_recorded_retrospective'),
    'br_sentence_loss_of_office_61_20_20160831': ('2016-08-31', 'senate_judgment'),
    'br_sentence_disqualification_not_imposed_20160831': ('2016-08-31', 'disqualification_not_imposed'),
    'br_dou_masthead_temer_vp_in_exercise_20160831': ('2016-08-31', 'vice_president_exercising_office'),
    'br_mensagem144_to_vp_in_exercise_20160831': ('2016-08-31', 'vice_president_exercising_office'),
    'br_temer_posse_declared_20160831': ('2016-08-31', 'posse_declared'),
    'br_temer_termo_de_posse_20160831': ('2016-08-31', 'posse_record'),
    'br_dcn_vp_in_exercise_at_posse_opening_20160831': ('2016-08-31', 'vice_president_exercising_office'),
    'br_bolsonaro_posse_declared_20190101': ('2019-01-01', 'posse_declared'),
    'br_bolsonaro_elected_20181028': ('2018-10-28', 'popular_election'),
    'br_temer_styled_ex_president_20190101': ('2019-01-01', 'styled_former_president'),
    'br_bolsonaro_diplomado_tse_20181210': ('2018-12-10', 'diplomacao'),
    'br_bolsonaro_signed_termo_de_posse_20190101': ('2019-01-01', 'posse_record'),
    'br_bolsonaro_signs_as_president_mpv870_20190101': ('2019-01-01', 'in_office_attestation'),
    'br_bolsonaro_sash_ceremony_speech_20190101': ('2019-01-01', 'sash_ceremony'),
    'br_mourao_vp_in_exercise_d11322_20221230': ('2022-12-30', 'vice_president_exercising_office'),
    'br_mourao_vp_in_exercise_d11324_20221231': ('2022-12-31', 'vice_president_exercising_office'),
    'br_lula_posse_declared_20230101': ('2023-01-01', 'posse_declared'),
    'br_lula_elected_20221030': ('2022-10-30', 'popular_election'),
    'br_lula_diplomacao_recited_20221212': ('2022-12-12', 'diplomacao'),
    'br_lula_diplomado_tse_20221212': ('2022-12-12', 'diplomacao'),
    'br_lula_signs_as_president_mpv1154_20230101': ('2023-01-01', 'in_office_attestation'),
    'br_lula_signs_as_president_mpv1388_20260824': ('2026-08-24', 'in_office_attestation'),
}
NEW_SOURCES = list(RESPONSES)
ORIGINAL_SOURCES = ('br_tse_fefc_2024', 'br_tse_fefc_announcement_20240617', 'br_tse_missao_20251104',
                    'br_tse_pmb_rename_20251202', 'br_trerj_pmb_name_notices')
PR = 'br_president'
T_PR = 'President of the Federative Republic of Brazil'
T_VP = 'Vice-President of the Republic in exercise of the office of President'

# Exact holder observations of br_president: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('José Sarney', '1990-01-05', None, None),
    ('Fernando Collor de Mello', None, '1990-03-15', '1992-12-29'),
    ('Itamar Franco', None, '1992-12-29', None),
    ('Fernando Henrique Cardoso', None, '1995-01-01', None),
    ('Fernando Henrique Cardoso', None, '1999-01-01', None),
    ('Luiz Inácio Lula da Silva', None, '2003-01-01', None),
    ('Luiz Inácio Lula da Silva', None, '2007-01-01', None),
    ('Dilma Rousseff', None, '2011-01-01', None),
    ('Dilma Rousseff', None, '2015-01-01', '2016-08-31'),
    ('Michel Temer', None, '2016-08-31', None),
    ('Jair Bolsonaro', None, '2019-01-01', None),
    ('Luiz Inácio Lula da Silva', None, '2023-01-01', None),
]
HOLDER_CLAIMS = [
    ['br_sarney_signs_decreto_98797_19900105'],
    ['br_collor_posse_declared_congress_19900315', 'br_collor_termo_de_posse_19900315',
     'br_collor_assumes_presidency_statement_19900315', 'br_collor_resignation_letter_read_senate_19921229',
     'br_collor_resignation_letter_read_congress_19921229', 'br_presidency_vacancy_declared_19921229',
     'br_collor_resignation_occurred_19921229', 'br_collor_resignation_letter_printed_19921229'],
    ['br_itamar_posse_declared_congress_19921229', 'br_itamar_termo_de_posse_19921229'],
    ['br_fhc_posse_declared_19950101', 'br_fhc_termo_de_posse_19950101', 'br_fhc_assumes_position_today_19950101'],
    ['br_fhc_posse_declared_19990101', 'br_fhc_termo_de_posse_19990101',
     'br_fhc_second_mandate_assumed_acm_address_19990101'],
    ['br_lula_posse_declared_20030101', 'br_lula_termo_de_posse_20030101',
     'br_lula_assumes_presidency_first_secretary_20030101', 'br_lula_assumption_recalled_20070101'],
    ['br_lula_posse_declared_20070101', 'br_lula_termo_de_posse_20070101', 'br_lula_new_mandate_inaugural_day_20070101'],
    ['br_dilma_posse_declared_20110101', 'br_dilma_termo_de_posse_20110101',
     'br_dilma_assumes_responsibility_sarney_address_20110101', 'br_dilma_assumes_government_parlatorio_20110101'],
    ['br_dilma_posse_declared_20150101', 'br_dilma_termo_de_posse_20150101',
     'br_dilma_assumes_second_mandate_parlatorio_20150101', 'br_senate_resolution35_removal_rousseff_20160831',
     'br_dou_publishes_resolution35_20160831'],
    ['br_temer_posse_declared_20160831', 'br_temer_termo_de_posse_20160831'],
    ['br_bolsonaro_posse_declared_20190101', 'br_bolsonaro_signed_termo_de_posse_20190101',
     'br_bolsonaro_signs_as_president_mpv870_20190101'],
    ['br_lula_posse_declared_20230101', 'br_lula_signs_as_president_mpv1154_20230101',
     'br_lula_signs_as_president_mpv1388_20260824'],
]
# Claims that must never feed a holder observation.
ELECTIONS = (
    'br_collor_tse_diploma_issued_19891230', 'br_collor_elected_ballots_19891217',
    'br_tse_diplomacao_ceremony_catalogued_19891230', 'br_collor_diplomacao_recited_19891230',
    'br_collor_second_round_recited_19891217', 'br_fhc_elected_first_round_19941003', 'br_fhc_diplomado_tse_19941217',
    'br_fhc_reelected_first_round_19981004', 'br_fhc_diplomado_tse_19981212', 'br_lula_elected_20021027',
    'br_lula_diplomado_tse_20021214', 'br_lula_reelected_20061029', 'br_lula_diplomado_tse_20061214',
    'br_dilma_elected_20101031', 'br_dilma_diplomada_tse_20101217', 'br_dilma_reelected_20141026',
    'br_dilma_diplomada_tse_20141218', 'br_bolsonaro_elected_20181028', 'br_bolsonaro_diplomado_tse_20181210',
    'br_lula_elected_20221030', 'br_lula_diplomacao_recited_20221212', 'br_lula_diplomado_tse_20221212')
OATHS = ('br_fhc_oath_before_congress_19950101', 'br_fhc_oath_before_congress_19990101',
         'br_lula_oath_before_congress_20030101', 'br_lula_oath_before_congress_20070101',
         'br_dilma_oath_before_congress_20110101', 'br_dilma_oath_before_congress_20150101')
IMPEACHMENT = (
    'br_chamber_authorizes_impeachment_process_19920929', 'br_chamber_authorization_read_in_senate_19920930',
    'br_senate_approves_process_opening_opinion_19921001', 'br_senate_mesa_formalizes_summons_19921001',
    'br_senate_summons_mandado_suspension_terms_19921001', 'br_collor_receives_summons_suspended_19921002',
    'br_senate_votes_to_continue_trial_19921229', 'br_senate_disqualification_vote_19921230',
    'br_senate_impeachment_sentence_19921230', 'br_senate_resolution_101_disqualification_19921230',
    'br_senate_resolution_101_first_printed_19921230',
    'br_camara_authorizes_senate_trial_rousseff_20160417', 'br_senate_admits_denuncia_rousseff_20160512',
    'br_senate_intimation_order_suspension_terms_20160512', 'br_agsen_senate_opens_process_0634_20160512',
    'br_rousseff_receives_intimation_suspended_20160512', 'br_radio_senado_notifications_served_20160512',
    'br_senate_record_intimation_receipt_signed_20160512', 'br_sentence_records_suspension_20160512',
    'br_senate_vote_loss_of_office_20160831', 'br_senate_vote_disqualification_20160831',
    'br_sentence_loss_of_office_61_20_20160831', 'br_sentence_disqualification_not_imposed_20160831')
VICE_PRESIDENT = (
    'br_vice_president_instructed_to_assume_19921001', 'br_itamar_states_exercise_from_19921002',
    'br_itamar_exercising_signs_lei_8471_19921007', 'br_senate_record_vp_notified_20160512',
    'br_temer_vp_in_exercise_mpv726_20160512', 'br_dou_masthead_temer_vp_in_exercise_20160831',
    'br_mensagem144_to_vp_in_exercise_20160831', 'br_dcn_vp_in_exercise_at_posse_opening_20160831',
    'br_mourao_vp_in_exercise_d11322_20221230', 'br_mourao_vp_in_exercise_d11324_20221231')
SASH = ('br_dilma_sash_announced_congress_address_20110101', 'br_bolsonaro_sash_ceremony_speech_20190101')
SUCCESSOR_STATEMENTS = ('br_itamar_leaves_government_fhc_address_19950101',
                        'br_lula_leaves_government_dilma_parlatorio_20110101')
SPANS = ('br_bibpr_sarney_government_period_19850315_19900315', 'br_bibpr_collor_first_phase_19900315_19921002',
         'br_bibpr_itamar_phase_19921002_period_19921229')
CONTEXT = ('br_sarney_signs_decreto_99167_19900313', 'br_collor_signs_decreto_663_19921001',
           'br_temer_styled_ex_president_20190101')
NEVER_HOLDER = ELECTIONS + OATHS + IMPEACHMENT + VICE_PRESIDENT + SASH + SUCCESSOR_STATEMENTS + SPANS + CONTEXT
# Event kinds that may date or bound a holder; every other kind never does.
FROM_KINDS = {'posse_declared', 'posse_record'}
UNTIL_KINDS = {'resignation', 'resignation_letter_read', 'vacancy_declared', 'removal_resolution', 'removal_in_force'}
SUPPORT_KINDS = {'assumption_statement', 'assumption_statement_retrospective', 'in_office_attestation'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | SUPPORT_KINDS
# Dates that are never any holder's attested_on, start or end: elections, diplomações, impeachment steps before the
# effect, suspensions, the Vice-President's exercise, the last signatures before an event and every period end
# declared at a posse.
NEVER_HOLDER_DATE = {
    '1989-11-15', '1989-12-17', '1989-12-30', '1990-03-13', '1992-09-29', '1992-09-30', '1992-10-01', '1992-10-02',
    '1992-10-07', '1992-12-30', '1994-10-03', '1994-12-17', '1998-10-04', '1998-12-12', '1998-12-31', '2002-10-27',
    '2002-12-14', '2002-12-31', '2006-10-29', '2006-12-14', '2006-12-31', '2010-10-31', '2010-12-17', '2010-12-31',
    '2014-10-26', '2014-12-18', '2014-12-31', '2016-04-17', '2016-05-12', '2018-10-28', '2018-12-10', '2018-12-31',
    '2022-10-30', '2022-12-12', '2022-12-30', '2022-12-31'}
STARTS = [(h[0], h[2]) for h in HOLDERS if h[2]]
ENDS = [('Fernando Collor de Mello', '1992-12-29'), ('Dilma Rousseff', '2016-08-31')]
SURNAMES = {'José Sarney': 'Sarney', 'Fernando Collor de Mello': 'Collor', 'Itamar Franco': 'Itamar',
            'Fernando Henrique Cardoso': 'Cardoso', 'Luiz Inácio Lula da Silva': 'Lula', 'Dilma Rousseff': 'Rousseff',
            'Michel Temer': 'Temer', 'Jair Bolsonaro': 'Bolsonaro'}
LEAD_URL_MARKERS = ('wikipedia', 'camara.leg.br/noticias', 'transmissao-da-faixa', 'dilma-recebe-a-faixa', 'bdsf',
                    'discursos-e-pronunciamentos', 'geraldo-alckmin', 'biografia-periodo-presidencial', 'DCN02JAN2007',
                    'd99177', 'd0667', 'codDiario=6431', 'codDiario=6487', 'dm=3692516', 'dm=3692525',
                    'Constituicao', 'codDiario=20398', 'codDiario=20405', 'dm=4654044', 'l15490', 'vep-541',
                    'vicentinho-alves-levara', '20190903092728', 'cronologia-do-impeachment',
                    'image_view_fullscreen')
# Claim ids withdrawn or renamed by the checks; none may remain.
STALE_IDS = ('br_temer_declared_mandate_period_20160831_20181231',
             'br_bolsonaro_declared_mandate_period_20190101_20221231',
             'br_lula_declared_mandate_period_20230101_20270104', 'br_temer_posse_president_20160831',
             'br_temer_termo_de_posse_vacancy_20160831', 'br_bolsonaro_posse_president_20190101',
             'br_bolsonaro_election_recorded_20181028', 'br_bolsonaro_tse_diploma_20181210',
             'br_lula_posse_president_20230101', 'br_lula_election_recorded_20221030',
             'br_lula_diplomacao_recorded_20221212', 'attested_period', 'loss_of_political_rights',
             'end_of_term_statement', '2027-01-04')
REPORT = research.RESEARCH / 'brazil-presidents-1990-2026-10.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-10.md'


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][len(ORIGINAL_SOURCES):]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def presidency_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert [i['id'] for i in packet['institutions']] == ['br_presidency'], 'exactly one presidency institution'
    presidency = packet['institutions'][0]
    assert presidency['kind'] == 'executive_institution' and presidency['lifecycle']['status'] == 'unknown'
    assert [r['id'] for r in presidency['roles']] == [PR], 'exactly one role'
    role = presidency['roles'][0]
    assert (role['title'], role['kind']) == (T_PR, 'head_of_state')
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
             if r['kind'] == 'head_of_state']
    assert heads == [PR], 'no other head-of-state role'
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
            assert row['holder_name'] == name and row['role_title'] == T_PR, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        # A start only where a declaration of posse or termo states that day; otherwise no posse claim at all.
        assert (holder['from'] in from_days) if holder['from'] else not from_days, (name, 'start')
        # An end only where a resignation or removal record states that day; otherwise no end claim at all.
        assert (holder['until'] in until_days) if holder['until'] else not until_days, (name, 'end')
        if holder['attested_on']:
            assert holder['attested_on'] in kinds.get('in_office_attestation', set()), (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and (holder['from'] or holder['attested_on']) <= holder['until']
        assert 'Vice-President' not in name and 'Mourão' not in name, name
    # The Vice-President's exercise stays a claim on the role, and nothing on it is dated by a structured period.
    for cid in VICE_PRESIDENT:
        assert cid in role['claim_ids'] and rows[cid]['role_title'] == T_VP, cid
    for cid in SPANS:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid
    for cid in presidency['claim_ids']:
        assert 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid


def presidency_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    presidency_rules(packet, rows)
    role = packet['institutions'][0]['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    # Distinct dated events stay distinct.
    for cid, day in (('br_collor_elected_ballots_19891217', '1989-12-17'),
                     ('br_collor_tse_diploma_issued_19891230', '1989-12-30'),
                     ('br_collor_posse_declared_congress_19900315', '1990-03-15'),
                     ('br_collor_receives_summons_suspended_19921002', '1992-10-02'),
                     ('br_itamar_states_exercise_from_19921002', '1992-10-02'),
                     ('br_collor_resignation_occurred_19921229', '1992-12-29'),
                     ('br_senate_disqualification_vote_19921230', '1992-12-30'),
                     ('br_senate_resolution_101_disqualification_19921230', '1992-12-30'),
                     ('br_fhc_elected_first_round_19941003', '1994-10-03'),
                     ('br_fhc_diplomado_tse_19941217', '1994-12-17'),
                     ('br_lula_diplomado_tse_20061214', '2006-12-14'),
                     ('br_camara_authorizes_senate_trial_rousseff_20160417', '2016-04-17'),
                     ('br_rousseff_receives_intimation_suspended_20160512', '2016-05-12'),
                     ('br_temer_vp_in_exercise_mpv726_20160512', '2016-05-12'),
                     ('br_senate_vote_loss_of_office_20160831', '2016-08-31'),
                     ('br_bolsonaro_elected_20181028', '2018-10-28'),
                     ('br_bolsonaro_diplomado_tse_20181210', '2018-12-10'),
                     ('br_mourao_vp_in_exercise_d11324_20221231', '2022-12-31'),
                     ('br_lula_signs_as_president_mpv1388_20260824', '2026-08-24')):
        assert claims[cid]['attested_on'] == day, cid


class BrazilPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.presidency = cls.packet['institutions'][0]
        cls.role = cls.presidency['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows()
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Brazil'}, {'Brazil': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (48, 112))
        self.assertEqual([s['id'] for s in self.packet['sources']], list(ORIGINAL_SOURCES) + NEW_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (32, 1))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER)), (41, 71))
        # Every new claim and source is cited by the institution and its one role, and by no organization.
        self.assertEqual(self.presidency['claim_ids'], self.new_claims)
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.presidency['sources'], NEW_SOURCES)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        for entry in self.packet['organizations']:
            self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
            self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        # At most ten observations, each reported and each carrying rows.
        observations = re.findall(r'^### (BR-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'BR-PRES-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'BR-PRES-{n:02d}' for n in range(1, 11)})
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)

    def test_holders_are_exactly_as_intended(self):
        presidency_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected_sources = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected_sources:
                    expected_sources.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
        # The institution follows the accepted presidency shape.
        self.assertEqual(list(self.presidency), ['id', 'name', 'kind', 'represented_party_ids', 'reconciled_organization_id',
                                                 'lifecycle', 'roles', 'sources', 'claim_ids', 'coverage'])
        self.assertEqual(self.presidency['represented_party_ids'], [])
        self.assertIsNone(self.presidency['reconciled_organization_id'])
        self.assertEqual(self.presidency['lifecycle'], {
            'status': 'unknown', 'from': None, 'until': None,
            'note': 'These observations do not establish founding, dissolution, legal continuity or exact terms of the institution.'})
        self.assertEqual(self.presidency['coverage']['status'], 'partial')
        self.assertEqual(self.presidency['coverage']['period'], {'from': '1990-01-01', 'through': '2026-09-07'})
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        # The Vice-President's exercise and every never-holder claim keep a kind that cannot date a holder.
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], FROM_KINDS | UNTIL_KINDS | {'assumption_statement'}, cid)
        for cid in VICE_PRESIDENT:
            self.assertEqual(self.rows[cid]['role_title'], T_VP, cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)never a holder|not a holder|claims? only', cid)
        for cid in set(self.new_claims) - set(VICE_PRESIDENT):
            self.assertEqual(self.rows[cid]['role_title'], T_PR, cid)
        self.assertEqual({self.rows[cid]['holder_name'] for cid in VICE_PRESIDENT},
                         {'Itamar Franco', 'Michel Temer', 'Hamilton Mourão', None})

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims = self.claims
        # Starts: a posse record stating a period that begins that day.
        for cid in ('br_collor_posse_declared_congress_19900315', 'br_itamar_posse_declared_congress_19921229',
                    'br_fhc_posse_declared_19950101', 'br_lula_posse_declared_20030101', 'br_dilma_posse_declared_20110101',
                    'br_temer_posse_declared_20160831', 'br_bolsonaro_posse_declared_20190101',
                    'br_lula_posse_declared_20230101'):
            self.assertRegex(claims[cid]['text'], r'(?i)period|empossad', cid)
        # Ends: the resignation 'nesta data' and Resolution No. 35 in force on publication that day.
        self.assertIn('on that date and by that instrument he resigns', claims['br_collor_resignation_letter_read_congress_19921229']['text'])
        self.assertIn('which occurred on 29 December 1992', claims['br_collor_resignation_occurred_19921229']['text'])
        self.assertIn('enters into force on its publication', claims['br_senate_resolution35_removal_rousseff_20160831']['text'])
        self.assertIn('Publication day', claims['br_dou_publishes_resolution35_20160831']['uncertainty'])
        # Periods declared at a posse are text only and never ends; the post-cutoff end is never structured.
        self.assertIn('4 de janeiro de 2027', claims['br_lula_posse_declared_20230101']['text'])
        self.assertIn('never as a structured date or boundary', claims['br_lula_posse_declared_20230101']['uncertainty'])
        for cid in ('br_temer_posse_declared_20160831', 'br_bolsonaro_posse_declared_20190101',
                    'br_fhc_posse_declared_19950101', 'br_fhc_termo_de_posse_19990101', 'br_lula_posse_declared_20070101'):
            self.assertRegex(claims[cid]['uncertainty'], r'prospective', cid)
        # Successor statements, oaths, suspensions and sash ceremonies are claims, never boundaries.
        for cid in SUCCESSOR_STATEMENTS:
            self.assertEqual(self.rows[cid]['event_kind'], 'predecessor_departure_statement')
            self.assertIn('never a boundary', claims[cid]['uncertainty'])
        for cid in OATHS:
            self.assertEqual(self.rows[cid]['event_kind'], 'oath_of_office', cid)
            self.assertIn('never a start or a holder date', claims[cid]['uncertainty'], cid)
        for cid in ('br_collor_receives_summons_suspended_19921002', 'br_rousseff_receives_intimation_suspended_20160512'):
            self.assertEqual(self.rows[cid]['event_kind'], 'suspension')
            self.assertRegex(claims[cid]['uncertainty'], r'not an end', cid)
        self.assertIn('misprint', claims['br_bolsonaro_sash_ceremony_speech_20190101']['uncertainty'])
        self.assertIn('prospective', claims['br_dilma_sash_announced_congress_address_20110101']['uncertainty'].lower())
        # Disqualification is not a loss of political rights; Resolution 101 was first printed on 30 December.
        self.assertEqual(self.rows['br_senate_resolution_101_disqualification_19921230']['event_kind'],
                         'disqualification_from_public_function')
        self.assertIn('reprint', claims['br_senate_resolution_101_disqualification_19921230']['uncertainty'])
        self.assertIn("does not mention political rights", claims['br_senate_resolution_101_disqualification_19921230']['uncertainty'])
        # Retrospective library spans are recorded, never boundaries.
        for cid in SPANS:
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'], cid)
        # Holder wording: stated ends explained, unstated ends refused.
        holders = self.role['holder_claims']
        self.assertIn('earliest 1990 attestation pinned to a byte-stable response', holders[0]['note'])
        self.assertIn("successor's statement", holders[2]['uncertainty'])
        self.assertIn("successor's statement", holders[6]['uncertainty'])
        self.assertIn('not split', holders[1]['uncertainty'])
        self.assertIn('not split', holders[8]['uncertainty'])
        for index in (9, 10):
            self.assertIn('prospective', holders[index]['uncertainty'])
        self.assertIn('never as a holder of it', self.role['scope_note'])
        self.assertIn('procedure only, never a date', self.role['scope_note'])
        self.assertIn("successor's or third party's statement", self.role['scope_note'])
        unresolved = self.presidency['coverage']['unresolved']
        self.assertTrue(unresolved[0].startswith('Presidents 1990-2026 (CLAUDE-C01-10'))
        self.assertIn('Stated ends: Fernando Collor de Mello', unresolved[1])
        self.assertTrue(unresolved[-1].startswith('Keep executive office distinct from party leadership'))
        self.assertEqual(sum('CLAUDE-C01-10' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-23', '2026-09-23'))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'])
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
                self.assertEqual((row['observation_id'], row['role_id']), ('br_presidency', PR))
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind']) for cid, row in self.rows.items()}, EVENTS)
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
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED]
        self.assertEqual(len(others), 29)
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others},
                         {'legis.senado.leg.br', 'imagem.camara.leg.br', 'bibliotecadigital.tse.jus.br',
                          'www12.senado.leg.br'})
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
            # The diary viewer's page-range form is rebuilt on request and is never a recorded identity.
            self.assertNotIn('seqPagina', self.sources[sid]['url'], sid)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        self.assertFalse([s['id'] for s in self.packet['sources'] if s['url'].endswith('comprovante-de-recebimento-de-dilma')])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'g1.globo', 'folha.uol', 'estadao', 'cronologia-do-impeachment', 'agência câmara'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('transmissao-da-faixa-ocorreu-no-parlatorio', 'dilma-recebe-a-faixa-de-lula-no-parlatorio',
                       '208597-dilma-recebe-a-faixa-presidencial', 'DCN02JAN2007', 'biografia-periodo-presidencial',
                       'discurso-do-presidente-lula-no-parlatorio', 'cronologia-do-impeachment',
                       'vicentinho-alves-levara', 'l15490', 'ConstituicaoCompilado'):
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
            return packet['institutions'][0]['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        validator_cases = [
            (lambda p: source(p, 'br_dsf_223_suplemento_19921230')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'br_cn_dcn15_20160901')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'br_dcn_08_19900316')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 11).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 0).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'br_lula_signs_as_president_mpv1388_20260824').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'br_lula_posse_declared_20230101').update(
                period={'from': '2023-01-01', 'through': '2027-01-04'}), 'exceeds cutoff'),
            (lambda p: holder(p, 8).update(until='2014-12-31'), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('br_lula_posse_declared_20030101'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('br_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        vp_holder = {'name': 'Itamar Franco', 'attested_on': None, 'from': '1992-10-02', 'until': None,
                     'sources': ['br_dcn_70_19921230'], 'claim_ids': ['br_itamar_states_exercise_from_19921002']}
        temer_vp = {'name': 'Michel Temer', 'attested_on': '2016-05-12', 'from': None, 'until': None,
                    'sources': ['br_planalto_mpv726_20160512'], 'claim_ids': ['br_temer_vp_in_exercise_mpv726_20160512']}
        mourao = {'name': 'Hamilton Mourão', 'attested_on': '2022-12-30', 'from': None, 'until': None,
                  'sources': ['br_planalto_d11322_20221230'], 'claim_ids': ['br_mourao_vp_in_exercise_d11322_20221230']}
        rule_cases = [
            ('successor posse used as an end (Sarney)', lambda p: holder(p, 0).update(until='1990-03-15')),
            ('successor posse used as an end (Itamar)', lambda p: holder(p, 2).update(until='1995-01-01')),
            ('successor statement cited as an end (Itamar)', lambda p: (holder(p, 2).update(until='1995-01-01'), holder(
                p, 2)['claim_ids'].append('br_itamar_leaves_government_fhc_address_19950101'))),
            ('successor statement cited as an end (Lula 2007)', lambda p: (holder(p, 6).update(until='2011-01-01'), holder(
                p, 6)['claim_ids'].append('br_lula_leaves_government_dilma_parlatorio_20110101'))),
            ('successor posse used as an end (Temer)', lambda p: holder(p, 9).update(until='2019-01-01')),
            ('successor posse used as an end (Bolsonaro)', lambda p: holder(p, 10).update(until='2023-01-01')),
            ('re-election used as an end (Cardoso 1995)', lambda p: holder(p, 3).update(until='1999-01-01')),
            ('declared period used as an end (Temer)', lambda p: holder(p, 9).update(until='2018-12-31')),
            ('declared period used as an end (Bolsonaro)', lambda p: holder(p, 10).update(until='2022-12-31')),
            ('suspension used as an end (Collor)', lambda p: holder(p, 1).update(until='1992-10-02')),
            ('suspension used as an end (Rousseff)', lambda p: holder(p, 8).update(until='2016-05-12')),
            ('judgment used as an end (Collor)', lambda p: holder(p, 1).update(until='1992-12-30')),
            ('election date used as start (Collor)', lambda p: holder(p, 1).update({'from': '1989-12-17'})),
            ('diplomação date used as start (Collor)', lambda p: holder(p, 1).update({'from': '1989-12-30'})),
            ('diplomação date used as start (Lula 2023)', lambda p: holder(p, 11).update({'from': '2022-12-12'})),
            ('election date used as start (Bolsonaro)', lambda p: holder(p, 10).update({'from': '2018-10-28'})),
            ('Vice-President exercise date used as start (Temer)', lambda p: holder(p, 9).update({'from': '2016-05-12'})),
            ('Vice-President interim exercise added as a holder (Itamar)',
             lambda p: role(p)['holder_claims'].insert(2, vp_holder)),
            ('Vice-President interim exercise added as a holder (Temer)',
             lambda p: role(p)['holder_claims'].insert(9, temer_vp)),
            ('Vice-President exercise added as a holder (Mourão)', lambda p: role(p)['holder_claims'].insert(11, mourao)),
            ('Vice-President claim cited by a holder', lambda p: holder(p, 9)['claim_ids'].append(
                'br_dcn_vp_in_exercise_at_posse_opening_20160831')),
            ('election claim cited by a holder', lambda p: holder(p, 4)['claim_ids'].append(
                'br_fhc_reelected_first_round_19981004')),
            ('oath cited by a holder', lambda p: holder(p, 5)['claim_ids'].append('br_lula_oath_before_congress_20030101')),
            ('sash ceremony cited by a holder', lambda p: holder(p, 10)['claim_ids'].append(
                'br_bolsonaro_sash_ceremony_speech_20190101')),
            ('library span given a date', lambda p: claim(p, 'br_bibpr_sarney_government_period_19850315_19900315').update(
                attested_on='1990-03-15')),
            ('declared period stored as a structure', lambda p: claim(p, 'br_temer_posse_declared_20160831').update(
                period={'from': '2016-08-31', 'through': '2018-12-31'})),
            ('observation date without an in-office act (Sarney)', lambda p: holder(p, 0).update(attested_on='1990-03-15')),
            ('second role', lambda p: p['institutions'][0]['roles'].append(dict(copy.deepcopy(role(p)), id='br_president_2'))),
            ('second institution', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][0]),
                                                                           id='br_presidency_2'))),
            ('head of state moved onto an organization', lambda p: p['organizations'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='br_president_org'))),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        presidency_rules(self.packet, self.rows)
        presidency_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    presidency_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    presidency_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('br_fhc_elected_first_round_19941003', '1995-01-01'),
                         ('br_collor_receives_summons_suspended_19921002', '1992-12-29'),
                         ('br_senate_disqualification_vote_19921230', '1992-12-29')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                presidency_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted in part', '02': 'Accepted', '03': 'Accepted', '04': 'Accepted in part',
                     '05': 'Accepted', '06': 'Accepted', '07': 'Accepted', '08': 'Accepted in part',
                     '09': 'Accepted in part', '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| BR-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 9)] + [f'B{n}' for n in range(1, 10)] + [f'C{n}' for n in range(1, 11)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Resolved by removal)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('0d177d7d', 'ffe54b02', 'research-index.json', 'test_brazil_research_s10f.py', 'test_campaign_census',
                     'Observed on'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'brazil-presidents-1990-2026-10.md', 'claude/c01-br-10', '0d177d7d',
                     'ffe54b02', 'test_brazil_presidents_c01_10.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Brazil')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (1, 1))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Brazil'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
