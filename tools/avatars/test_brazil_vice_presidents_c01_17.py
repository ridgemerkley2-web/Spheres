"""CLAUDE-C01-17: Brazil's vice-presidents, 1990-2026, keep election, diplomação, posse, the Vice-President's
exercise of the Presidency, his succession to it, vacancy and any end apart, and state a start or an end only where a
source does. A Vice-President exercising the Presidency is never a holder of either role."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research

PR = 'br_president'
VP = 'br_vice_president'
T_PR = 'President of the Federative Republic of Brazil'
T_VPR = 'Vice-President of the Federative Republic of Brazil'
T_VP_EX = 'Vice-President of the Republic in exercise of the office of President'
ORIGINAL_SOURCE_COUNT = 5
C01_10_SOURCE_COUNT = 48
C01_10_CLAIM_COUNT = 112

# New sources, in packet order, with the original response identity recorded in each extract: (bytes, sha256).
RESPONSES = {
    'br_dcn_36_19850423':
        (237692, '0a2f326ea976372660d4d1096fbdee9054a588f037276e9476ca8ebb2cb5278d'),
    'br_tse_diploma_itamar_19891230':
        (677161, 'cc83b74299e99287ce120b4b8966786d7c70dfac4f1ae4adfc06dba786dd73af'),
    'br_planalto_dnn428_19911211':
        (5934, '02f18ec05b4f1a891baa02e64869b2171edbbd89a3dc3ee0d9cfab661d193612'),
    'br_planalto_lei_8469_19921005':
        (5013, '5843b0cb78f331fc7f75ce834b2d0771c90ad6d29930b99f42f687d6ea500d57'),
    'br_bibpr_itamar_speech_19921005':
        (1616993, '4639b08f839700a669c2ef25012427074a3dc52d6ade6dae8ad97a045b2b2ab0'),
    'br_vpr_biografias_vice_presidentes_2024':
        (2129103, 'ae0c5e07eea9fa87b431e597b43e81ce476ca04188585ba4d82e73314993327f'),
    'br_tse_diploma_maciel_19941217':
        (678470, '120ec1e7e6f17ed060d48aa6e8d84bc5b8b644db1b642ed3bd8fa1ed6d4ba51c'),
    'br_senado_den1_2016_autos_vol48':
        (29934149, '72e1b1e6e629b9b59663f6dd74ed76a414af1febc8ef88fc3312206d946f4b90'),
    'br_cn_dcn1_20230102_p20_26':
        (24950218, 'd6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee'),
    'br_planalto_lei15434_20260616':
        (17609, '074515205282e77edacb93658747cc9ebff54b7930f1fbd69153c3774768fb89'),
    'br_planalto_lei15436_20260617':
        (58054, 'd06c3098377a2c18e7ba5cff5f6b53f62c70176f229ae3eff0c767671aace246'),
}
NEW_SOURCES = list(RESPONSES)
# CLAUDE-C01-10 sources whose recorded response also carries Vice-President claims: the claims are added to the
# same source record (no second record for the same response), and the identity stays CLAUDE-C01-10's.
SHARED = {
    'br_dcn_08_19900316':
        (1255764, '838abf03a2ef59781aabc66121dd9fd938ed78ebd15df36a6f0c02c70c7b6645'),
    'br_dcn_70_19921230':
        (397945, 'dfe8e27ab3045795d36504ed15aa4ef85a914cebe42682756e1a0cd2ba020d33'),
    'br_bibpr_collor_page_2016':
        (42045, 'e0ad5fdb5c40149d18b7ea5237c20e3cc1aa60790b6f59beec75cf03d3cbf49d'),
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
    'br_dcn_1_2015_posse_20150101':
        (8131269, '8f588186c81de17bd8c7009f6ad37322edfc295eda788e33b4afd579fb861787'),
    'br_cn_dcn15_20160901':
        (8585228, '60e2c85c00bf0215c934de795ca0c98a107ab9fc6bfe8ec29128032241824195'),
    'br_cn_dcn1_20190102_p5_11':
        (49257639, '2d8a63d500ff5ffc82d690a1dc27c3239e25ece96511550e5a82398fc90b24eb'),
    'br_cn_dcn1_20190102_full':
        (49257639, '2d8a63d500ff5ffc82d690a1dc27c3239e25ece96511550e5a82398fc90b24eb'),
    'br_cn_dcn1_20230102_p1_8':
        (24950218, 'd6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee'),
}
ADDED_TO_C01_10 = {
    'br_dcn_08_19900316': ['br_itamar_vp_second_round_recited_19891217', 'br_itamar_vp_oath_before_congress_19900315',
                           'br_itamar_vp_posse_declared_19900315', 'br_itamar_vp_termo_de_posse_19900315',
                           'br_itamar_vp_diplomacao_recited_19891230'],
    'br_dcn_70_19921230': ['br_itamar_vp_notification_announced_19921229', 'br_itamar_vp_invested_as_president_19921229'],
    'br_bibpr_collor_page_2016': ['br_bibpr_collor_page_vp_itamar_span_19900315_19921229'],
    'br_dcn_1_1995_posse_19950101': ['br_maciel_elected_first_round_19941003', 'br_maciel_diplomado_tse_19941217',
                                     'br_maciel_oath_before_congress_19950101', 'br_maciel_posse_declared_19950101',
                                     'br_maciel_termo_de_posse_19950101',
                                     'br_maciel_saluted_vice_president_closing_19950101'],
    'br_dcn_1_1999_posse_19990101': ['br_maciel_styled_incumbent_opening_19990101',
                                     'br_maciel_reelected_first_round_19981004', 'br_maciel_diplomado_tse_19981212',
                                     'br_maciel_oath_before_congress_19990101', 'br_maciel_posse_declared_19990101',
                                     'br_maciel_termo_de_posse_19990101',
                                     'br_maciel_saluted_vice_president_cardoso_address_19990101'],
    'br_dcn_1_2003_posse_20030101': ['br_alencar_elected_20021027', 'br_alencar_diplomado_tse_20021214',
                                     'br_alencar_oath_before_congress_20030101', 'br_alencar_posse_declared_20030101',
                                     'br_alencar_termo_de_posse_20030101',
                                     'br_alencar_saluted_vice_president_lula_address_20030101'],
    'br_dcn_1_2007_posse_20070101': ['br_alencar_styled_incumbent_opening_20070101', 'br_alencar_reelected_20061029',
                                     'br_alencar_diplomado_tse_20061214', 'br_alencar_oath_before_congress_20070101',
                                     'br_alencar_posse_declared_20070101', 'br_alencar_termo_de_posse_20070101',
                                     'br_alencar_saluted_vice_president_lula_address_20070101'],
    'br_dcn_1_2011_posse_20110101': ['br_temer_vp_elected_20101031', 'br_temer_vp_diplomado_tse_20101217',
                                     'br_temer_vp_oath_before_congress_20110101', 'br_temer_vp_posse_declared_20110101',
                                     'br_temer_vp_termo_de_posse_20110101', 'br_temer_vp_styled_after_posse_20110101',
                                     'br_alencar_tribute_rousseff_address_20110101'],
    'br_dcn_1_2015_posse_20150101': ['br_temer_vp_reelected_20141026', 'br_temer_vp_diplomado_tse_20141218',
                                     'br_temer_vp_oath_before_congress_20150101', 'br_temer_vp_posse_declared_20150101',
                                     'br_temer_vp_termo_de_posse_20150101', 'br_temer_vp_styled_after_posse_20150101'],
    'br_cn_dcn15_20160901': ['br_temer_vp_succeeds_art79_termo_20160831'],
    'br_cn_dcn1_20190102_p5_11': ['br_mourao_elected_20181028', 'br_mourao_oath_before_congress_20190101',
                                  'br_mourao_posse_declared_20190101', 'br_mourao_termo_read_20190101',
                                  'br_mourao_saluted_vice_president_bolsonaro_address_20190101'],
    'br_cn_dcn1_20190102_full': ['br_mourao_diplomado_tse_20181210', 'br_mourao_termo_de_compromisso_20190101',
                                 'br_mourao_signed_termo_de_posse_20190101'],
    'br_cn_dcn1_20230102_p1_8': ['br_alckmin_elected_20221030', 'br_alckmin_oath_before_congress_20230101',
                                 'br_alckmin_posse_declared_20230101', 'br_alckmin_termo_read_20230101',
                                 'br_alckmin_saluted_vice_president_lula_address_20230101'],
}
ARCHIVED = {
    'br_planalto_dnn428_19911211': '20240724055151',
    'br_planalto_lei_8469_19921005': '20081014095729',
    'br_bibpr_itamar_speech_19921005': '20250421205127',
    'br_vpr_biografias_vice_presidentes_2024': '20251207025501',
    'br_planalto_lei15434_20260616': '20260620021856',
    'br_planalto_lei15436_20260617': '20260816171933',
}
# Rendered pages: new extracts' pdf_pages_one_based, and the pages CLAUDE-C01-17 rendered in the shared extracts
# (CLAUDE-C01-10's own pdf_pages_one_based are pinned unchanged in test_brazil_presidents_c01_10.py).
PDF_PAGES = {
    'br_dcn_36_19850423': [1, 3],
    'br_bibpr_itamar_speech_19921005': [2],
    'br_vpr_biografias_vice_presidentes_2024': [6, 7, 47, 49, 55, 57, 59],
    'br_senado_den1_2016_autos_vol48': [16, 17],
    'br_cn_dcn1_20230102_p20_26': [20, 23, 26],
}
C01_17_PAGES = {
    'br_dcn_08_19900316': [3, 4], 'br_dcn_70_19921230': [3, 6], 'br_bibpr_collor_page_2016': [],
    'br_dcn_1_1995_posse_19950101': [6, 9], 'br_dcn_1_1999_posse_19990101': [3, 5, 6, 7, 8, 9],
    'br_dcn_1_2003_posse_20030101': [3, 5, 6, 7], 'br_dcn_1_2007_posse_20070101': [3, 5, 7, 9],
    'br_dcn_1_2011_posse_20110101': [3, 5, 7, 8, 9], 'br_dcn_1_2015_posse_20150101': [4, 5, 6, 7, 9, 10, 11],
    'br_cn_dcn15_20160901': [6], 'br_cn_dcn1_20190102_p5_11': [6], 'br_cn_dcn1_20190102_full': [15, 18, 20],
    'br_cn_dcn1_20230102_p1_8': [6],
}
# Every CLAUDE-C01-17 claim, in packet order: (observation, attested_on, event_kind, holder_name).
EVENTS = {
    'br_itamar_vp_second_round_recited_19891217': ('BR-VP-02', '1989-12-17', 'popular_election', 'Itamar Franco'),
    'br_itamar_vp_oath_before_congress_19900315': ('BR-VP-02', '1990-03-15', 'oath_of_office', 'Itamar Franco'),
    'br_itamar_vp_posse_declared_19900315': ('BR-VP-02', '1990-03-15', 'posse_declared', 'Itamar Franco'),
    'br_itamar_vp_termo_de_posse_19900315': ('BR-VP-02', '1990-03-15', 'posse_record', 'Itamar Franco'),
    'br_itamar_vp_diplomacao_recited_19891230': ('BR-VP-02', '1989-12-30', 'diplomacao', 'Itamar Franco'),
    'br_itamar_vp_notification_announced_19921229':
        ('BR-VP-03', '1992-12-29', 'vice_president_notification_announced', 'Itamar Franco'),
    'br_itamar_vp_invested_as_president_19921229':
        ('BR-VP-03', '1992-12-29', 'vice_president_succeeds_to_presidency', 'Itamar Franco'),
    'br_bibpr_collor_page_vp_itamar_span_19900315_19921229':
        ('BR-VP-03', None, 'retrospective_term_span', 'Itamar Franco'),
    'br_maciel_elected_first_round_19941003': ('BR-VP-04', '1994-10-03', 'popular_election', 'Marco Maciel'),
    'br_maciel_diplomado_tse_19941217': ('BR-VP-04', '1994-12-17', 'diplomacao', 'Marco Maciel'),
    'br_maciel_oath_before_congress_19950101': ('BR-VP-04', '1995-01-01', 'oath_of_office', 'Marco Maciel'),
    'br_maciel_posse_declared_19950101': ('BR-VP-04', '1995-01-01', 'posse_declared', 'Marco Maciel'),
    'br_maciel_termo_de_posse_19950101': ('BR-VP-04', '1995-01-01', 'posse_record', 'Marco Maciel'),
    'br_maciel_saluted_vice_president_closing_19950101':
        ('BR-VP-04', '1995-01-01', 'in_office_attestation', 'Marco Maciel'),
    'br_maciel_styled_incumbent_opening_19990101':
        ('BR-VP-04', '1999-01-01', 'styled_incumbent_before_posse', 'Marco Maciel'),
    'br_maciel_reelected_first_round_19981004': ('BR-VP-04', '1998-10-04', 'popular_election', 'Marco Maciel'),
    'br_maciel_diplomado_tse_19981212': ('BR-VP-04', '1998-12-12', 'diplomacao', 'Marco Maciel'),
    'br_maciel_oath_before_congress_19990101': ('BR-VP-04', '1999-01-01', 'oath_of_office', 'Marco Maciel'),
    'br_maciel_posse_declared_19990101': ('BR-VP-04', '1999-01-01', 'posse_declared', 'Marco Maciel'),
    'br_maciel_termo_de_posse_19990101': ('BR-VP-04', '1999-01-01', 'posse_record', 'Marco Maciel'),
    'br_maciel_saluted_vice_president_cardoso_address_19990101':
        ('BR-VP-04', '1999-01-01', 'in_office_attestation', 'Marco Maciel'),
    'br_alencar_elected_20021027': ('BR-VP-05', '2002-10-27', 'popular_election', 'José Alencar'),
    'br_alencar_diplomado_tse_20021214': ('BR-VP-05', '2002-12-14', 'diplomacao', 'José Alencar'),
    'br_alencar_oath_before_congress_20030101': ('BR-VP-05', '2003-01-01', 'oath_of_office', 'José Alencar'),
    'br_alencar_posse_declared_20030101': ('BR-VP-05', '2003-01-01', 'posse_declared', 'José Alencar'),
    'br_alencar_termo_de_posse_20030101': ('BR-VP-05', '2003-01-01', 'posse_record', 'José Alencar'),
    'br_alencar_saluted_vice_president_lula_address_20030101':
        ('BR-VP-05', '2003-01-01', 'in_office_attestation', 'José Alencar'),
    'br_alencar_styled_incumbent_opening_20070101':
        ('BR-VP-05', '2007-01-01', 'styled_incumbent_before_posse', 'José Alencar'),
    'br_alencar_reelected_20061029': ('BR-VP-05', '2006-10-29', 'popular_election', 'José Alencar'),
    'br_alencar_diplomado_tse_20061214': ('BR-VP-05', '2006-12-14', 'diplomacao', 'José Alencar'),
    'br_alencar_oath_before_congress_20070101': ('BR-VP-05', '2007-01-01', 'oath_of_office', 'José Alencar'),
    'br_alencar_posse_declared_20070101': ('BR-VP-05', '2007-01-01', 'posse_declared', 'José Alencar'),
    'br_alencar_termo_de_posse_20070101': ('BR-VP-05', '2007-01-01', 'posse_record', 'José Alencar'),
    'br_alencar_saluted_vice_president_lula_address_20070101':
        ('BR-VP-05', '2007-01-01', 'in_office_attestation', 'José Alencar'),
    'br_temer_vp_elected_20101031': ('BR-VP-06', '2010-10-31', 'popular_election', 'Michel Temer'),
    'br_temer_vp_diplomado_tse_20101217': ('BR-VP-06', '2010-12-17', 'diplomacao', 'Michel Temer'),
    'br_temer_vp_oath_before_congress_20110101': ('BR-VP-06', '2011-01-01', 'oath_of_office', 'Michel Temer'),
    'br_temer_vp_posse_declared_20110101': ('BR-VP-06', '2011-01-01', 'posse_declared', 'Michel Temer'),
    'br_temer_vp_termo_de_posse_20110101': ('BR-VP-06', '2011-01-01', 'posse_record', 'Michel Temer'),
    'br_temer_vp_styled_after_posse_20110101': ('BR-VP-06', '2011-01-01', 'in_office_attestation', 'Michel Temer'),
    'br_alencar_tribute_rousseff_address_20110101':
        ('BR-VP-05', '2011-01-01', 'predecessor_tribute_statement', 'José Alencar'),
    'br_temer_vp_reelected_20141026': ('BR-VP-06', '2014-10-26', 'popular_election', 'Michel Temer'),
    'br_temer_vp_diplomado_tse_20141218': ('BR-VP-06', '2014-12-18', 'diplomacao', 'Michel Temer'),
    'br_temer_vp_oath_before_congress_20150101': ('BR-VP-06', '2015-01-01', 'oath_of_office', 'Michel Temer'),
    'br_temer_vp_posse_declared_20150101': ('BR-VP-06', '2015-01-01', 'posse_declared', 'Michel Temer'),
    'br_temer_vp_termo_de_posse_20150101': ('BR-VP-06', '2015-01-01', 'posse_record', 'Michel Temer'),
    'br_temer_vp_styled_after_posse_20150101': ('BR-VP-06', '2015-01-01', 'in_office_attestation', 'Michel Temer'),
    'br_temer_vp_succeeds_art79_termo_20160831':
        ('BR-VP-07', '2016-08-31', 'vice_president_succeeds_to_presidency', 'Michel Temer'),
    'br_mourao_elected_20181028': ('BR-VP-08', '2018-10-28', 'popular_election', 'Hamilton Mourão'),
    'br_mourao_oath_before_congress_20190101': ('BR-VP-08', '2019-01-01', 'oath_of_office', 'Hamilton Mourão'),
    'br_mourao_posse_declared_20190101': ('BR-VP-08', '2019-01-01', 'posse_declared', 'Hamilton Mourão'),
    'br_mourao_termo_read_20190101': ('BR-VP-08', '2019-01-01', 'posse_record', 'Hamilton Mourão'),
    'br_mourao_saluted_vice_president_bolsonaro_address_20190101':
        ('BR-VP-08', '2019-01-01', 'in_office_attestation', 'Hamilton Mourão'),
    'br_mourao_diplomado_tse_20181210': ('BR-VP-08', '2018-12-10', 'diplomacao', 'Hamilton Mourão'),
    'br_mourao_termo_de_compromisso_20190101': ('BR-VP-08', '2019-01-01', 'oath_of_office', 'Hamilton Mourão'),
    'br_mourao_signed_termo_de_posse_20190101': ('BR-VP-08', '2019-01-01', 'posse_record', 'Hamilton Mourão'),
    'br_alckmin_elected_20221030': ('BR-VP-09', '2022-10-30', 'popular_election', 'Geraldo Alckmin'),
    'br_alckmin_oath_before_congress_20230101': ('BR-VP-09', '2023-01-01', 'oath_of_office', 'Geraldo Alckmin'),
    'br_alckmin_posse_declared_20230101': ('BR-VP-09', '2023-01-01', 'posse_declared', 'Geraldo Alckmin'),
    'br_alckmin_termo_read_20230101': ('BR-VP-09', '2023-01-01', 'posse_record', 'Geraldo Alckmin'),
    'br_alckmin_saluted_vice_president_lula_address_20230101':
        ('BR-VP-09', '2023-01-01', 'in_office_attestation', 'Geraldo Alckmin'),
    'br_sarney_vp_oath_recited_19850315': ('BR-VP-01', '1985-03-15', 'oath_recital', 'José Sarney'),
    'br_sarney_vp_exercise_from_recited_19850315':
        ('BR-VP-01', '1985-03-15', 'vice_president_exercise_start', 'José Sarney'),
    'br_presidency_vacant_vp_sarney_succeeds_19850422':
        ('BR-VP-01', '1985-04-22', 'vice_president_succeeds_to_presidency', 'José Sarney'),
    'br_sarney_mensagem232_continues_as_successor_19850421':
        ('BR-VP-01', '1985-04-21', 'succession_statement', 'José Sarney'),
    'br_itamar_vp_tse_diploma_issued_19891230': ('BR-VP-02', '1989-12-30', 'diplomacao', 'Itamar Franco'),
    'br_itamar_vp_elected_ballots_19891217': ('BR-VP-02', '1989-12-17', 'popular_election', 'Itamar Franco'),
    'br_itamar_vp_exercising_dnn428_19911211':
        ('BR-VP-03', '1991-12-11', 'vice_president_exercising_office', 'Itamar Franco'),
    'br_itamar_vp_exercising_lei_8469_19921005':
        ('BR-VP-03', '1992-10-05', 'vice_president_exercising_office', 'Itamar Franco'),
    'br_itamar_vp_in_exercise_speech_19921005':
        ('BR-VP-03', '1992-10-05', 'vice_president_exercising_office', 'Itamar Franco'),
    'br_vpbio_vacancy_chronology_undated': ('BR-VP-01', None, 'retrospective_vacancy_statement', None),
    'br_vpbio_sarney_vp_span_19850315_19850421': ('BR-VP-01', None, 'retrospective_term_span', 'José Sarney'),
    'br_vpbio_itamar_vp_span_19900315_19921229': ('BR-VP-03', None, 'retrospective_term_span', 'Itamar Franco'),
    'br_vpbio_temer_vp_span_20110101_20160831': ('BR-VP-07', None, 'retrospective_term_span', 'Michel Temer'),
    'br_vpbio_chronology_grey_segment_2016_2019': ('BR-VP-07', None, 'retrospective_chronology_depiction', None),
    'br_vpbio_mourao_vp_span_20190101_20221231': ('BR-VP-08', None, 'retrospective_term_span', 'Hamilton Mourão'),
    'br_vpbio_alckmin_vp_since_20230101': ('BR-VP-09', None, 'retrospective_term_span', 'Geraldo Alckmin'),
    'br_maciel_tse_diploma_issued_19941217': ('BR-VP-04', '1994-12-17', 'diplomacao', 'Marco Maciel'),
    'br_temer_notified_to_assume_interim_20160512':
        ('BR-VP-07', '2016-05-12', 'vice_president_notified_to_exercise', 'Michel Temer'),
    'br_temer_vp_notification_filed_signed_20160512':
        ('BR-VP-07', '2016-05-12', 'vice_president_notification_receipt_filed', 'Michel Temer'),
    'br_alckmin_diplomado_tse_20221212': ('BR-VP-09', '2022-12-12', 'diplomacao', 'Geraldo Alckmin'),
    'br_alckmin_signed_termo_de_posse_20230101': ('BR-VP-09', '2023-01-01', 'posse_record', 'Geraldo Alckmin'),
    'br_alckmin_termo_de_compromisso_20230101': ('BR-VP-09', '2023-01-01', 'oath_of_office', 'Geraldo Alckmin'),
    'br_alckmin_vp_exercising_l15434_20260616':
        ('BR-VP-10', '2026-06-16', 'vice_president_exercising_office', 'Geraldo Alckmin'),
    'br_alckmin_vp_exercising_l15436_20260617':
        ('BR-VP-10', '2026-06-17', 'vice_president_exercising_office', 'Geraldo Alckmin'),
}
C01_17_CLAIMS = list(EVENTS)
# CLAUDE-C01-10 claims cited on br_vice_president: not duplicated, not re-keyed; their rows stay br_president.
CITED = ('br_tse_diplomacao_ceremony_catalogued_19891230', 'br_vice_president_instructed_to_assume_19921001',
         'br_itamar_states_exercise_from_19921002', 'br_itamar_exercising_signs_lei_8471_19921007',
         'br_bibpr_itamar_phase_19921002_period_19921229', 'br_radio_senado_notifications_served_20160512',
         'br_senate_record_vp_notified_20160512', 'br_temer_vp_in_exercise_mpv726_20160512',
         'br_dou_masthead_temer_vp_in_exercise_20160831', 'br_mensagem144_to_vp_in_exercise_20160831',
         'br_dcn_vp_in_exercise_at_posse_opening_20160831', 'br_mourao_vp_in_exercise_d11322_20221230',
         'br_mourao_vp_in_exercise_d11324_20221231')
CITED_KINDS = {'diplomacao_ceremony', 'vice_president_notified_to_exercise', 'vice_president_exercise_start',
               'vice_president_exercising_office', 'retrospective_term_span', 'summons_served_report'}
VP_SOURCES = [
    'br_tse_catalogue_diplomacao_1989', 'br_dcn_08_19900316', 'br_senado_autos_impeachment_vol1',
    'br_planalto_lei_8471_19921007', 'br_dcn_70_19921230', 'br_bibpr_collor_page_2016', 'br_bibpr_itamar_page_2016',
    'br_dcn_1_1995_posse_19950101', 'br_dcn_1_1999_posse_19990101', 'br_dcn_1_2003_posse_20030101',
    'br_dcn_1_2007_posse_20070101', 'br_dcn_1_2011_posse_20110101', 'br_dcn_1_2015_posse_20150101',
    'br_radio_senado_notifications_20160512', 'br_senado_den1_2016_materia_20260508', 'br_planalto_mpv726_20160512',
    'br_senado_den1_2016_dou_20160831_extra', 'br_senado_den1_2016_mensagem144', 'br_cn_dcn15_20160901',
    'br_cn_dcn1_20190102_p5_11', 'br_cn_dcn1_20190102_full', 'br_planalto_d11322_20221230',
    'br_planalto_d11324_20221231', 'br_cn_dcn1_20230102_p1_8'] + NEW_SOURCES

# Exact holder observations of br_vice_president: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('Itamar Franco', None, '1990-03-15', None),
    ('Marco Maciel', None, '1995-01-01', None),
    ('Marco Maciel', None, '1999-01-01', None),
    ('José Alencar', None, '2003-01-01', None),
    ('José Alencar', None, '2007-01-01', None),
    ('Michel Temer', None, '2011-01-01', None),
    ('Michel Temer', None, '2015-01-01', None),
    ('Hamilton Mourão', None, '2019-01-01', None),
    ('Geraldo Alckmin', None, '2023-01-01', None),
]
HOLDER_CLAIMS = [
    ['br_itamar_vp_posse_declared_19900315', 'br_itamar_vp_termo_de_posse_19900315'],
    ['br_maciel_posse_declared_19950101', 'br_maciel_termo_de_posse_19950101',
     'br_maciel_saluted_vice_president_closing_19950101'],
    ['br_maciel_posse_declared_19990101', 'br_maciel_termo_de_posse_19990101',
     'br_maciel_saluted_vice_president_cardoso_address_19990101'],
    ['br_alencar_posse_declared_20030101', 'br_alencar_termo_de_posse_20030101',
     'br_alencar_saluted_vice_president_lula_address_20030101'],
    ['br_alencar_posse_declared_20070101', 'br_alencar_termo_de_posse_20070101',
     'br_alencar_saluted_vice_president_lula_address_20070101'],
    ['br_temer_vp_posse_declared_20110101', 'br_temer_vp_termo_de_posse_20110101',
     'br_temer_vp_styled_after_posse_20110101'],
    ['br_temer_vp_posse_declared_20150101', 'br_temer_vp_termo_de_posse_20150101',
     'br_temer_vp_styled_after_posse_20150101'],
    ['br_mourao_posse_declared_20190101', 'br_mourao_termo_read_20190101', 'br_mourao_signed_termo_de_posse_20190101',
     'br_mourao_saluted_vice_president_bolsonaro_address_20190101'],
    ['br_alckmin_posse_declared_20230101', 'br_alckmin_termo_read_20230101', 'br_alckmin_signed_termo_de_posse_20230101',
     'br_alckmin_saluted_vice_president_lula_address_20230101'],
]
# CLAUDE-C01-10's br_president holders, which this packet must not change: (name, attested_on, from, until).
PRESIDENT_HOLDERS = [
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
# Event kinds that may date or bound a Vice-President holder. No reviewed source states the end of a
# vice-presidential term or a vacancy of the Vice-Presidency, so no claim carries an until kind.
FROM_KINDS = {'posse_declared', 'posse_record'}
UNTIL_KINDS = {'vice_presidency_end_stated', 'vice_presidency_vacancy_declared', 'resignation', 'removal_in_force'}
SUPPORT_KINDS = {'in_office_attestation'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | SUPPORT_KINDS
EXERCISE_KINDS = {'vice_president_exercising_office', 'vice_president_exercise_start',
                  'vice_president_notified_to_exercise', 'vice_president_notification_receipt_filed'}
CLAIM_ONLY_KINDS = EXERCISE_KINDS | {
    'popular_election', 'diplomacao', 'oath_of_office', 'oath_recital', 'styled_incumbent_before_posse',
    'predecessor_tribute_statement', 'vice_president_notification_announced', 'vice_president_succeeds_to_presidency',
    'succession_statement', 'retrospective_term_span', 'retrospective_vacancy_statement',
    'retrospective_chronology_depiction'}
SPAN_KINDS = {'retrospective_term_span', 'retrospective_vacancy_statement', 'retrospective_chronology_depiction'}
# Dates that are never a Vice-President holder's attested_on, start or end: the 1985 context, elections,
# diplomações, the Vice-President's exercise and notification, his succession to the Presidency (and the Presidency's
# vacancy declared that day), the last acts before a posse, and every period end declared at a posse.
NEVER_HOLDER_DATE = {
    '1985-03-15', '1985-04-21', '1985-04-22', '1989-11-15', '1989-12-17', '1989-12-30', '1991-12-11', '1992-10-01',
    '1992-10-02', '1992-10-05', '1992-10-07', '1992-12-29', '1994-10-03', '1994-12-17', '1998-10-04', '1998-12-12',
    '1998-12-31', '2002-10-27', '2002-12-14', '2002-12-31', '2006-10-29', '2006-12-14', '2006-12-31', '2010-10-31',
    '2010-12-17', '2010-12-31', '2014-10-26', '2014-12-18', '2014-12-31', '2016-05-12', '2016-08-31', '2018-10-28',
    '2018-12-10', '2018-12-31', '2022-10-30', '2022-12-12', '2022-12-30', '2022-12-31', '2026-06-16', '2026-06-17'}
SURNAMES = {'Itamar Franco': 'Itamar', 'Marco Maciel': 'Maciel', 'José Alencar': 'Alencar', 'Michel Temer': 'Temer',
            'Hamilton Mourão': 'Mourão', 'Geraldo Alckmin': 'Alckmin'}
# The Presidency's vacancy claims must never be cited on br_vice_president.
PRESIDENCY_VACANCY = ('br_presidency_vacancy_declared_19921229', 'br_collor_resignation_occurred_19921229',
                      'br_dou_publishes_resolution35_20160831', 'br_senate_resolution35_removal_rousseff_20160831')
# Leads and blocked items that must stay out of every source URL.
LEAD_URL_MARKERS = ('wikipedia', 'temer.jpg', 'mandado-de-notificacao-michel-temer', 'l15435', 'd12958', 'd12939',
                    'L8470', 'D0363', 'DCD03OUT1992', 'codDiario=6431', 'Ex_presidentesCD_Republica', 'L13332',
                    'DCD0020160901', 'DCD0020160902', 'codDiario=123478', 'geraldo-alckmin-toma-posse',
                    'galeria-de-ex-vice-presidentes', 'vicepresidencia.gov.br', '20240222080132',
                    'biografia-dos-vice-presidentes-da-republica/itamar-franco', '16-12-1992', 'onstituicao',
                    'decretos-nao-numerados', '23f606e7', '1a12c506', 'd74dd68a', '19f3eb3f', '1062d3d7', '77548c34',
                    'jose-sarney/vice-presidente', 'D9690')
# Forms regenerated or personalised per request, and cache-busting parameters, are never recorded identities.
GENERATED_URL_PATTERNS = ('seqPagina', 'nocache', 'cb=', 'rv=', 'jsessionid', 'f5_cspm', 'TSPD', '_=', 'sid=',
                          'token')
# Ids withdrawn or renamed by the checks; none may remain in the packet.
STALE_IDS = ('br_sarney_vp_exercising_presidency_recited_19850422', 'br_vpbio_vacancy_chronology_20250715',
             'br_itamar_vp_summoned_to_succeed_19921229', 'vice_president_summoned_to_succeed',
             'br_vpbio_chronology_segment_2016_2019_20250715', 'vice_president_notified_to_assume_interim',
             'br_temer_vp_notified_jaburu_20160512', 'br_temer_vp_exercising_presidency_mpv726_20160512',
             'br_temer_vp_exercising_dou_expedient_20160831', 'br_temer_vp_in_exercise_at_posse_opening_20160831',
             'br_mourao_vp_exercising_d11322_20221230', 'br_mourao_vp_exercising_d11324_20221231',
             'br_alckmin_vp_exercising_d12958_20260507', 'br_senado_temer_mandado_notificacao_20160512',
             'br_planalto_d12958_20260507', 'name_as_printed', 'attested_period', '20250715')
REPORT = research.RESEARCH / 'brazil-vice-presidents-1990-2026-17.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-17.md'


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][ORIGINAL_SOURCE_COUNT:]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def roles_of(packet):
    presidency = packet['institutions'][0]
    return presidency['roles'][0], presidency['roles'][1]


def vice_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert [i['id'] for i in packet['institutions']] == ['br_presidency'], 'exactly one presidency institution'
    presidency = packet['institutions'][0]
    assert [(r['id'], r['title'], r['kind']) for r in presidency['roles']] == [
        (PR, T_PR, 'head_of_state'), (VP, T_VPR, 'institutional_office')], 'exactly the two presidency roles'
    offices = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
               if r['kind'] == 'institutional_office' or r['id'] == VP]
    assert offices == [VP], 'the Vice-Presidency exists once, on the presidency institution'
    president, vice = presidency['roles']
    # The Vice-President's exercise never makes a br_president holder, and no br_president holder cites a VP row.
    for holder in president['holder_claims']:
        for cid in holder['claim_ids']:
            assert rows[cid]['role_id'] == PR and rows[cid]['event_kind'] not in CLAIM_ONLY_KINDS, (holder['name'], cid)
    assert not set(president['claim_ids']) & set(EVENTS), 'no CLAUDE-C01-17 claim on br_president'
    previous = ''
    for holder in vice['holder_claims']:
        name = holder['name']
        assert isinstance(holder, dict) and name in SURNAMES, name
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] >= previous, (name, 'holders stay in chronological order')
        previous = dated[0]
        assert not {holder['attested_on'], holder['from'], holder['until']} & NEVER_HOLDER_DATE, name
        kinds = {}
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in vice['claim_ids'], cid
            # Cross-role guard: a Vice-President holder rests only on br_vice_president rows of the same person.
            assert row['role_id'] == VP and row['holder_name'] == name and row['role_title'] == T_VPR, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        assert (holder['from'] in from_days) if holder['from'] else not from_days, (name, 'start')
        assert (holder['until'] in until_days) if holder['until'] else not until_days, (name, 'end')
        if holder['attested_on']:
            assert holder['attested_on'] in kinds.get('in_office_attestation', set()), (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and (holder['from'] or holder['attested_on']) <= holder['until']
    # A vacancy is recorded only where a source states one: none is, and the Presidency's is never cited here.
    assert not set(vice['claim_ids']) & set(PRESIDENCY_VACANCY), 'Presidency vacancy cited on the Vice-Presidency'
    for cid in vice['claim_ids']:
        kind = rows[cid]['event_kind']
        assert 'vacancy_declared' not in kind and kind not in UNTIL_KINDS, cid
        assert not {'period', 'attested_period'} & set(claims[cid]), cid
        if kind in SPAN_KINDS:
            assert 'attested_on' not in claims[cid], (cid, 'retrospective claims carry no structured date')
        if rows[cid]['role_id'] == VP:
            assert rows[cid]['role_title'] == (T_VP_EX if kind in EXERCISE_KINDS else T_VPR), cid


def vice_invariants(packet, rows):
    """The rules plus the exact pinned holder list and events this packet intends."""
    vice_rules(packet, rows)
    president, vice = roles_of(packet)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in vice['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in vice['holder_claims']] == HOLDER_CLAIMS
    assert not [h for h in vice['holder_claims'] if h['until']], 'no stated end'
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in president['holder_claims']] == PRESIDENT_HOLDERS
    for cid, (_, day, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # Distinct dated events stay distinct.
    for earlier, later in (('br_itamar_vp_elected_ballots_19891217', 'br_itamar_vp_tse_diploma_issued_19891230'),
                           ('br_itamar_vp_tse_diploma_issued_19891230', 'br_itamar_vp_posse_declared_19900315'),
                           ('br_itamar_vp_exercising_dnn428_19911211', 'br_itamar_vp_exercising_lei_8469_19921005'),
                           ('br_itamar_vp_exercising_lei_8469_19921005', 'br_itamar_vp_invested_as_president_19921229'),
                           ('br_maciel_elected_first_round_19941003', 'br_maciel_tse_diploma_issued_19941217'),
                           ('br_alencar_diplomado_tse_20061214', 'br_alencar_posse_declared_20070101'),
                           ('br_temer_vp_diplomado_tse_20141218', 'br_temer_vp_posse_declared_20150101'),
                           ('br_temer_vp_posse_declared_20150101', 'br_temer_notified_to_assume_interim_20160512'),
                           ('br_temer_notified_to_assume_interim_20160512', 'br_temer_vp_succeeds_art79_termo_20160831'),
                           ('br_mourao_elected_20181028', 'br_mourao_diplomado_tse_20181210'),
                           ('br_mourao_diplomado_tse_20181210', 'br_mourao_posse_declared_20190101'),
                           ('br_alckmin_diplomado_tse_20221212', 'br_alckmin_posse_declared_20230101'),
                           ('br_alckmin_vp_exercising_l15434_20260616', 'br_alckmin_vp_exercising_l15436_20260617')):
        assert claims[earlier]['attested_on'] < claims[later]['attested_on'], (earlier, later)


class BrazilVicePresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'brazil.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.presidency = cls.packet['institutions'][0]
        cls.president, cls.vice = roles_of(cls.packet)
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES + list(SHARED)}
        cls.rows = load_rows()
        cls.c01_10_sources = [s['id'] for s in cls.packet['sources'][ORIGINAL_SOURCE_COUNT:
                                                                     ORIGINAL_SOURCE_COUNT + C01_10_SOURCE_COUNT]]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Brazil'}, {'Brazil': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual(tuple(len(ids[k]) for k in ('entries', 'sources', 'claims', 'roles')), (32, 64, 231, 2))
        self.assertEqual((len(NEW_SOURCES), len(C01_17_CLAIMS), len(CITED)), (11, 85, 13))
        self.assertEqual([s['id'] for s in self.packet['sources']][-len(NEW_SOURCES):], NEW_SOURCES)
        self.assertEqual(len(self.packet['sources']), ORIGINAL_SOURCE_COUNT + C01_10_SOURCE_COUNT + len(NEW_SOURCES))
        # Claims on responses CLAUDE-C01-10 already records are appended to those source records, after its own.
        self.assertEqual(set(ADDED_TO_C01_10), set(SHARED))
        added = []
        for sid in self.c01_10_sources:
            ids_ = [c['id'] for c in self.sources[sid]['claims']]
            extra = ADDED_TO_C01_10.get(sid, [])
            self.assertEqual(ids_[len(ids_) - len(extra):], extra, sid)
            self.assertFalse(set(ids_[:len(ids_) - len(extra)]) & set(EVENTS), sid)
            added += extra
        new_claims = [c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']]
        self.assertEqual(added + new_claims, C01_17_CLAIMS)
        # The institution cites CLAUDE-C01-10's records unchanged, then this packet's.
        c01_10_claims = [c['id'] for sid in self.c01_10_sources for c in self.sources[sid]['claims']
                         if c['id'] not in EVENTS]
        self.assertEqual(len(c01_10_claims), C01_10_CLAIM_COUNT)
        self.assertEqual(self.presidency['claim_ids'], c01_10_claims + C01_17_CLAIMS)
        self.assertEqual(self.presidency['sources'], self.c01_10_sources + NEW_SOURCES)
        self.assertEqual(self.president['claim_ids'], c01_10_claims)
        self.assertEqual(self.president['sources'], self.c01_10_sources)
        # The Vice-President role cites every new claim and the thirteen CLAUDE-C01-10 claims, in packet order.
        wanted = set(C01_17_CLAIMS) | set(CITED)
        self.assertEqual(self.vice['claim_ids'],
                         [c['id'] for s in self.packet['sources'] for c in s['claims'] if c['id'] in wanted])
        self.assertEqual(len(self.vice['claim_ids']), 98)
        self.assertEqual(self.vice['sources'], VP_SOURCES)
        for cid in CITED:
            self.assertIn(cid, c01_10_claims)
            self.assertEqual(self.rows[cid]['role_id'], PR, cid)
            self.assertIn(self.rows[cid]['event_kind'], CITED_KINDS, cid)
        # Every Vice-President claim is a holder claim or a claim that never feeds a holder, never both.
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        never = set(self.vice['claim_ids']) - holder_claims
        self.assertEqual((len(holder_claims), len(never)), (28, 70))
        self.assertTrue(holder_claims <= set(C01_17_CLAIMS))
        for cid in never:
            self.assertIn(self.rows[cid]['event_kind'], CLAIM_ONLY_KINDS | CITED_KINDS, cid)
        for entry in self.packet['organizations']:
            self.assertFalse(wanted & set(entry['claim_ids']), entry['id'])
            self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        # At most ten observations, each reported and each carrying rows.
        observations = re.findall(r'^### (BR-VP-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'BR-VP-{n:02d}' for n in range(1, 11)])
        self.assertEqual({self.rows[cid]['review_observation'] for cid in C01_17_CLAIMS},
                         {f'BR-VP-{n:02d}' for n in range(1, 11)})
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)

    def test_holders_are_exactly_as_intended(self):
        vice_invariants(self.packet, self.rows)
        for holder in self.vice['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            self.assertTrue(holder['uncertainty'].startswith('No end'), holder['name'])
            expected_sources = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected_sources:
                    expected_sources.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
            # No holder rests on an exercise, notification, succession or retrospective claim.
            for cid in holder['claim_ids']:
                self.assertNotIn(self.rows[cid]['event_kind'], CLAIM_ONLY_KINDS, cid)
        self.assertEqual(list(self.vice), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        self.assertEqual(list(self.presidency), ['id', 'name', 'kind', 'represented_party_ids',
                                                 'reconciled_organization_id', 'lifecycle', 'roles', 'sources',
                                                 'claim_ids', 'coverage'])
        # Each person keeps one holder name on every row.
        self.assertEqual({self.rows[cid]['holder_name'] for cid in C01_17_CLAIMS},
                         set(SURNAMES) | {'José Sarney', None})
        # Exercise rows carry the exercise title; every other Vice-President row the office's title.
        for cid in C01_17_CLAIMS:
            kind = EVENTS[cid][2]
            self.assertEqual(self.rows[cid]['role_title'], T_VP_EX if kind in EXERCISE_KINDS else T_VPR, cid)
        exercise = [cid for cid in self.vice['claim_ids'] if self.rows[cid]['event_kind'] in EXERCISE_KINDS]
        self.assertEqual(len(exercise), 18)
        for cid in exercise:
            self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)never a holder|not a holder|claims? only', cid)

    def test_starts_ends_and_vacancies_only_where_a_source_states_one(self):
        claims = self.claims
        for cid in [c for c, e in EVENTS.items() if e[2] == 'posse_declared']:
            self.assertRegex(claims[cid]['text'], r'(?i)period|empossad', cid)
            self.assertRegex(claims[cid]['uncertainty'], r'prospective', cid)
        for cid in [c for c, e in EVENTS.items() if e[2] in {'oath_of_office', 'oath_recital'}]:
            self.assertIn('never a start or a holder date', claims[cid]['uncertainty'], cid)
        for cid in [c for c, e in EVENTS.items() if e[2] in {'popular_election', 'diplomacao'}]:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never a holder date|never a start', cid)
        for cid in [c for c, e in EVENTS.items() if e[2] in SPAN_KINDS]:
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'], cid)
        # The Vice-President's succession to the Presidency states no end and no vacancy of the Vice-Presidency.
        for cid in ('br_itamar_vp_invested_as_president_19921229', 'br_temer_vp_succeeds_art79_termo_20160831'):
            self.assertIn('never an end', claims[cid]['uncertainty'], cid)
            self.assertRegex(claims[cid]['uncertainty'], r'no until and no vacancy', cid)
        self.assertIn("'irei cientificar'", claims['br_itamar_vp_notification_announced_19921229']['uncertainty'])
        self.assertIn('not recorded in the issue', claims['br_itamar_vp_notification_announced_19921229']['uncertainty'])
        self.assertIn('declaration of posse as President', claims['br_temer_vp_succeeds_art79_termo_20160831']['uncertainty'])
        self.assertIn('precedes the reading of the termo', claims['br_temer_vp_succeeds_art79_termo_20160831']['uncertainty'])
        # The 1985 exercise is recited from a day and stores no period.
        self.assertIn('no period is stored', claims['br_sarney_vp_exercise_from_recited_19850315']['uncertainty'])
        # The Termo de Juntada identifies the notification's lower signature.
        self.assertIn('assinado por S.Exa. no anverso', claims['br_temer_vp_notification_filed_signed_20160512']['text'])
        # The 2026 attestations extend nothing; the latest is Law No. 15.436, and the post-cutoff end stays text.
        self.assertIn('latest captured act', claims['br_alckmin_vp_exercising_l15434_20260616']['uncertainty'])
        self.assertIn('en dash', claims['br_alckmin_vp_exercising_l15436_20260617']['uncertainty'])
        self.assertIn('4 de janeiro de 2027', claims['br_alckmin_posse_declared_20230101']['text'])
        self.assertIn('never as a structured date', claims['br_alckmin_posse_declared_20230101']['uncertainty'])
        # Election claims state the election only.
        for cid in ('br_mourao_elected_20181028', 'br_alckmin_elected_20221030'):
            self.assertNotIn('diplomad', claims[cid]['text'], cid)
        vice = self.vice['holder_claims']
        self.assertIn('vacancy of the office of President declared that day', vice[0]['uncertainty'])
        self.assertIn("successor administration's statement", vice[4]['uncertainty'])
        self.assertIn('neither split nor end this holder', vice[6]['uncertainty'])
        self.assertIn('never cited by this holder', vice[8]['uncertainty'])
        scope = self.vice['scope_note']
        for text in ('never as a holder of br_president or of this role', 'procedure only, never a date',
                     'no holder has an end and no vacancy is recorded', 'keep their br_president rows and are cited here'):
            self.assertIn(text, scope)
        unresolved = self.presidency['coverage']['unresolved']
        self.assertTrue(unresolved[0].startswith('Presidents 1990-2026 (CLAUDE-C01-10'))
        self.assertTrue(unresolved[-3].startswith('Vice-Presidents 1990-2026 (CLAUDE-C01-17'))
        self.assertTrue(unresolved[-2].startswith('Still open for the Vice-Presidency'))
        self.assertTrue(unresolved[-1].startswith('Keep executive office distinct from party leadership'))
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertTrue(packet_unresolved[-1].startswith('Vice-presidents 1990-2026 (CLAUDE-C01-17)'))
        self.assertEqual(sum('CLAUDE-C01-17' in u for u in packet_unresolved), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES + list(SHARED):
            source, extract = self.sources[sid], self.extracts[sid]
            new = sid in RESPONSES
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            # New sources were accessed on 24 September 2026; shared ones keep CLAUDE-C01-10's access date.
            day = '2026-09-24' if new else '2026-09-23'
            self.assertEqual((extract['accessed_date'], source['accessed_date']), (day, day))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']),
                             RESPONSES[sid] if new else SHARED[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            if new:
                self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
                self.assertNotIn('c01_17_pdf_pages_one_based', extract['visual_review'])
                self.assertTrue(extract['bounded_scope'].startswith('Vice-President observations for CLAUDE-C01-17'))
            else:
                self.assertEqual(extract['visual_review']['c01_17_pdf_pages_one_based'], C01_17_PAGES[sid])
                self.assertIn('CLAUDE-C01-17 (br_vice_president rows)', extract['visual_review']['method'])
                self.assertIn('For CLAUDE-C01-17', extract['provenance_note'])
                self.assertIn('rows with role_id br_vice_president', extract['bounded_scope'])
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
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id',
                                             'holder_name', 'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
                self.assertEqual((row['observation_id'], row['role_id']),
                                 ('br_presidency', VP if row['claim_id'] in EVENTS else PR))
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES + list(SHARED)}),
                         len(NEW_SOURCES) + len(SHARED))
        got = {cid: (row['review_observation'], row['attested_on'], row['event_kind'], row['holder_name'])
               for cid, row in self.rows.items() if row['role_id'] == VP}
        self.assertEqual(got, EVENTS)
        self.assertEqual(list(got), C01_17_CLAIMS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(source['url'].split('id_/', 1)[1], source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED]
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others},
                         {'legis.senado.leg.br', 'bibliotecadigital.tse.jus.br'})
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
        # The page-scoped DCN 1/2023 record shares CLAUDE-C01-10's stored-issue identity and URL.
        self.assertEqual(self.sources['br_cn_dcn1_20230102_p20_26']['url'], self.sources['br_cn_dcn1_20230102_p1_8']['url'])
        self.assertEqual(RESPONSES['br_cn_dcn1_20230102_p20_26'], SHARED['br_cn_dcn1_20230102_p1_8'])

    def test_no_per_request_generated_url_and_no_secondary_lead(self):
        for sid in NEW_SOURCES + list(SHARED):
            url = self.sources[sid]['url']
            for pattern in GENERATED_URL_PATTERNS:
                self.assertNotIn(pattern, url, (sid, pattern))
            host = urlsplit(url).hostname
            # Live Planalto and gov.br pages carry per-request tokens; only raw archived captures are recorded.
            self.assertNotIn(host, {'www.planalto.gov.br', 'planalto.gov.br', 'www.gov.br', 'www12.senado.leg.br'})
            if host == 'web.archive.org':
                self.assertRegex(url, r'^https://web\.archive\.org/web/\d{14}id_/https?://')
            if 'BuscaPaginasDiario' in url:
                self.assertTrue(url.endswith('&download=true'), sid)
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], (source['id'], marker))
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'g1.globo', 'folha.uol', 'estadao', 'agência câmara'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('temer.jpg', 'l15435', 'd12958', 'L8470', 'D0363', 'DCD03OUT1992', 'L13332',
                       'DCD0020160901', 'codDiario=123478', 'geraldo-alckmin-toma-posse', 'd12939', '23f606e7',
                       '0d83a50c-3eb2-4041-ae8c-076a2f18a49f', 'Ex_presidentesCD_Republica', 'wikipedia'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        # The mistyped TSE lead identifier found by the check is not repeated.
        self.assertNotIn('0d83a50c-2eb2', self.report)

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

        def vice(packet):
            return packet['institutions'][0]['roles'][1]

        def president(packet):
            return packet['institutions'][0]['roles'][0]

        def holder(packet, index):
            return vice(packet)['holder_claims'][index]

        def cite(packet, index, cid, sid):
            holder(packet, index)['claim_ids'].append(cid)
            if sid not in holder(packet, index)['sources']:
                holder(packet, index)['sources'].append(sid)

        validator_cases = [
            (lambda p: source(p, 'br_senado_den1_2016_autos_vol48')['snapshot'].update(sha256='0' * 64),
             'checksum mismatch'),
            (lambda p: source(p, 'br_dcn_1_2015_posse_20150101')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'br_planalto_lei15436_20260617')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 8).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'br_alckmin_vp_exercising_l15436_20260617').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'br_alckmin_posse_declared_20230101').update(
                period={'from': '2023-01-01', 'through': '2027-01-04'}), 'exceeds cutoff'),
            (lambda p: holder(p, 6).update(until='2014-12-31'), 'Reversed historical interval'),
            (lambda p: holder(p, 1)['claim_ids'].append('br_alencar_posse_declared_20030101'), 'cited source'),
            (lambda p: vice(p)['claim_ids'].append('br_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        temer_interim = {'name': 'Michel Temer', 'attested_on': '2016-05-12', 'from': None, 'until': None,
                         'sources': ['br_senado_den1_2016_autos_vol48'],
                         'claim_ids': ['br_temer_notified_to_assume_interim_20160512'], 'note': 'n', 'uncertainty': 'u'}
        itamar_acting = {'name': 'Itamar Franco', 'attested_on': '1991-12-11', 'from': None, 'until': None,
                         'sources': ['br_planalto_dnn428_19911211'],
                         'claim_ids': ['br_itamar_vp_exercising_dnn428_19911211'], 'note': 'n', 'uncertainty': 'u'}
        itamar_from_exercise = {'name': 'Itamar Franco', 'attested_on': None, 'from': '1992-10-02', 'until': None,
                                'sources': ['br_dcn_70_19921230'],
                                'claim_ids': ['br_itamar_states_exercise_from_19921002'], 'note': 'n', 'uncertainty': 'u'}
        alckmin_president = {'name': 'Geraldo Alckmin', 'attested_on': '2026-06-17', 'from': None, 'until': None,
                             'sources': ['br_planalto_lei15436_20260617'],
                             'claim_ids': ['br_alckmin_vp_exercising_l15436_20260617'], 'note': 'n', 'uncertainty': 'u'}
        rule_cases = [
            ("successor's posse used as an end (Itamar Franco, Maciel 1995)", lambda p: holder(p, 0).update(until='1995-01-01')),
            ('posse as President used as an end (Itamar Franco 1992)', lambda p: holder(p, 0).update(until='1992-12-29')),
            ('Presidency vacancy cited as an end (Itamar Franco)', lambda p: (
                holder(p, 0).update(until='1992-12-29'),
                cite(p, 0, 'br_presidency_vacancy_declared_19921229', 'br_dcn_70_19921230'))),
            ('succession record cited as an end (Temer 2015)', lambda p: (
                holder(p, 6).update(until='2016-08-31'),
                cite(p, 6, 'br_temer_vp_succeeds_art79_termo_20160831', 'br_cn_dcn15_20160901'))),
            ('retrospective span cited as an end (Temer 2015)', lambda p: (
                holder(p, 6).update(until='2016-08-31'),
                cite(p, 6, 'br_vpbio_temer_vp_span_20110101_20160831', 'br_vpr_biografias_vice_presidentes_2024'))),
            ('re-election used as an end (Maciel 1995)', lambda p: holder(p, 1).update(until='1999-01-01')),
            ("successor's posse used as an end (Temer 2015, Mourão)", lambda p: holder(p, 6).update(until='2019-01-01')),
            ("successor's posse used as an end (Mourão, Alckmin)", lambda p: holder(p, 7).update(until='2023-01-01')),
            ('last act before a posse used as an end (Mourão)', lambda p: holder(p, 7).update(until='2022-12-31')),
            ('declared period used as an end (Alencar 2007)', lambda p: holder(p, 4).update(until='2010-12-31')),
            ("successor's tribute cited as an end (Alencar 2007)", lambda p: (
                holder(p, 4).update(until='2011-01-01'),
                cite(p, 4, 'br_alencar_tribute_rousseff_address_20110101', 'br_dcn_1_2011_posse_20110101'))),
            ('election date used as start (Itamar Franco)', lambda p: holder(p, 0).update({'from': '1989-12-17'})),
            ('diplomação date used as start (Itamar Franco)', lambda p: holder(p, 0).update({'from': '1989-12-30'})),
            ('election date used as start (Mourão)', lambda p: holder(p, 7).update({'from': '2018-10-28'})),
            ('diplomação date used as start (Alckmin)', lambda p: holder(p, 8).update({'from': '2022-12-12'})),
            ('interim exercise date used as start (Temer)', lambda p: holder(p, 6).update({'from': '2016-05-12'})),
            ('election claim cited by a holder (Maciel 1999)', lambda p: holder(p, 2)['claim_ids'].append(
                'br_maciel_reelected_first_round_19981004')),
            ('oath cited by a holder (Alencar 2003)', lambda p: holder(p, 3)['claim_ids'].append(
                'br_alencar_oath_before_congress_20030101')),
            ('styled incumbent cited by a holder (Maciel 1999)', lambda p: holder(p, 2)['claim_ids'].append(
                'br_maciel_styled_incumbent_opening_19990101')),
            ('interim exercise added as a Vice-President holder (Temer 2016)',
             lambda p: vice(p)['holder_claims'].insert(7, temer_interim)),
            ('acting service added as a Vice-President holder (Itamar Franco 1991)',
             lambda p: vice(p)['holder_claims'].insert(1, itamar_acting)),
            ('Vice-President exercise added as a br_president holder (Itamar Franco 1992)',
             lambda p: president(p)['holder_claims'].insert(2, itamar_from_exercise)),
            ('Vice-President exercise added as a br_president holder (Alckmin 2026)',
             lambda p: president(p)['holder_claims'].append(alckmin_president)),
            ('acting service cited by a Vice-President holder (Alckmin 2026)', lambda p: cite(
                p, 8, 'br_alckmin_vp_exercising_l15436_20260617', 'br_planalto_lei15436_20260617')),
            ('succession record cited by a br_president holder (Temer 2016)', lambda p: president(p)['holder_claims'][9][
                'claim_ids'].append('br_temer_vp_succeeds_art79_termo_20160831')),
            ('cross-role holder: a Vice-President holder moved onto br_president', lambda p: president(p)[
                'holder_claims'].insert(4, vice(p)['holder_claims'].pop(1))),
            ("cross-role claim: a Vice-President holder cites the President's posse (Maciel 1995)",
             lambda p: holder(p, 1)['claim_ids'].append('br_fhc_posse_declared_19950101')),
            ('cross-role claim: a Vice-President holder cites a CLAUDE-C01-10 exercise claim (Temer 2015)',
             lambda p: cite(p, 6, 'br_temer_vp_in_exercise_mpv726_20160512', 'br_planalto_mpv726_20160512')),
            ('cross-institution holder: the Vice-Presidency copied onto an organization',
             lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(vice(p)))),
            ('cross-institution holder: a second institution with the Vice-Presidency',
             lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][0]), id='br_vice_presidency'))),
            ('third role', lambda p: p['institutions'][0]['roles'].append(dict(copy.deepcopy(vice(p)), id='br_vp_2'))),
            ('roles swapped', lambda p: p['institutions'][0]['roles'].reverse()),
            ('Presidency vacancy recorded on the Vice-Presidency', lambda p: vice(p)['claim_ids'].append(
                'br_presidency_vacancy_declared_19921229')),
            ('retrospective span given a date', lambda p: claim(p, 'br_vpbio_itamar_vp_span_19900315_19921229').update(
                attested_on='1992-12-29')),
            ('exercise stored as a period (Sarney 1985)', lambda p: claim(
                p, 'br_sarney_vp_exercise_from_recited_19850315').update(
                attested_period={'from': '1985-03-15', 'through': '1985-04-22'})),
            ('printed name used as the holder name (Mourão)', lambda p: holder(p, 7).update(
                name='Antônio Hamilton Martins Mourão')),
            ('holders out of order', lambda p: vice(p)['holder_claims'].reverse()),
        ]
        vice_rules(self.packet, self.rows)
        vice_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    vice_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    vice_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('br_maciel_elected_first_round_19941003', '1995-01-01'),
                         ('br_temer_notified_to_assume_interim_20160512', '2016-08-31'),
                         ('br_alckmin_vp_exercising_l15436_20260617', '2026-06-16'),
                         ('br_itamar_vp_tse_diploma_issued_19891230', '1990-03-15')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                vice_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Unresolved', '02': 'Accepted', '03': 'Accepted in part', '04': 'Accepted',
                     '05': 'Accepted', '06': 'Accepted', '07': 'Accepted in part', '08': 'Accepted', '09': 'Accepted',
                     '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| BR-VP-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 9)] + [f'B{n}' for n in range(1, 7)] + [f'C{n}' for n in range(1, 14)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Resolved by removal)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('774af12d', '73e5fd36', 'claude/c01-br-10', 'research-index.json', 'test_brazil_research_s10f.py',
                     'test_brazil_presidents_c01_10.py', 'test_campaign_census', 'only file shared'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'brazil-vice-presidents-1990-2026-17.md', 'claude/c01-br-17', '774af12d',
                     'claude/c01-br-10', 'test_brazil_vice_presidents_c01_17.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Brazil')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (1, 2))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Brazil'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
