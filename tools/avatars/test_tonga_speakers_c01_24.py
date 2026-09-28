"""CLAUDE-C01-24: Tonga's 1990-2026 Speakers keep royal appointment, the Assembly's election, oath, resignation, revocation
and acting or interim service apart, and state an end only where a source states the day."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256) of the decoded body.
RESPONSES = {
    'to_tlr_1997': (4010541, '7a4726f077c92de73d6cd1e53cf1f868b31011d82bd5c510026fb1a00842e71c'),
    'to_tlr_2003': (1804479, '02cbec45843b0d7f69c9562db4e914dd98cd4be3290031aa5903ea7fb60c8e3e'),
    'to_pmo_legislative_assembly_page_2001': (12527, '83f4ab7dbcece397f8c72984d62bf8379d511ee97a643c48acb37b27947ae84e'),
    'to_pmo_legislative_assembly_page_20011214': (12470, '76eb2f55978c2cf0af3ccdae91020e7101d8cd224b98de1825902b5d542455a4'),
    'to_la_ministers_page_2005': (22559, 'ffae98874c9c1bc51621637d1a876ec0341342f480f9abff47cdae1c97692f5f'),
    'to_pmo_20020523_tuivakano_speaker': (6996, 'd37af6cc0bbd87d1b65f1d5e098cab62b6493633a51c30e90a37b8ac9cd2e796'),
    'to_pmo_legislative_assembly_page_20030408': (13397, 'a941aa7c956002ded2985db6e0646282a357cfd216f0240d6079a400eb1cd610'),
    'to_la_home_2004': (17824, '1256f7efb4c4e9b2a6ae3e706e5407f201c0669bc87bcaad082408fff8cde43e'),
    'to_pmo_legislative_assembly_page_20041226': (12802, 'c230a53068b0a7a85cff7bc9318be50a709cb1e73cc5244d62c62b6bbc5b98f5'),
    'to_la_speaker_page_2005': (4845, '99afe647480b9e51fa6cce9006b2c05b86459f8987d7859b822ac2e5d77beb5f'),
    'to_ipu_2005': (18665, '4c8d5ad55488323d1c1a4d873b774db7ffd77987ff11480660c90513b7f5bb62'),
    'to_pmo_20060209_veikune_directives': (12195, '9a70ba0ec00da615801f74b54b8d266b4dfca119c874dcee6ccff21354ef40e4'),
    'to_la_archives_2006': (17818, 'd97ed973eada8fa332ca800cb9edf8c250b69c2a030de438b7159a85ab1edc02'),
    'to_tlr_2007': (1556139, 'e3f6737aeaa3aa043877af675dfb1d094b92f7cc3d533ccaa5ce2a0f7afe847c'),
    'to_pmo_20080602_speaker_reply': (27356, '3808da91d6403621603d5f3150f7621e5d0c4911ed0bddbbda9509a09c5197c4'),
    'to_pmo_20100629_assembly_transcript': (137572, '410e1c6695ed41e67b0dfba28c3fe0d7676019468c88b11181b3a5cee34c6ad5'),
    'to_la_profile_tuilakepa_2011': (20258, '51ee9f614dbae8e555defc0caf3babd0d8099832fbe376bec8be3e4ba839327c'),
    'to_la_profile_tuivakano_2011': (19855, '011cb5583372eab3006c4df182984fd1644dbaa90fb2dd3b0d558253a8977eab'),
    'to_la_speakers_list_20110203': (38500, 'c75402b51750b1bfa68b31c6807c00c4d59734450f1d1b9e400f085002c45d11'),
    'to_la_speakers_list_20111130': (59808, 'c205e15e9878fda76c56228058cfc96d7f18250012ec4e2d4e94b8525ce3b22c'),
    'to_la_obituary_fusitua_20140505': (44098, 'fcd9294121ef73e339461af46293199af0d5fb530af5ada128b749161c133392'),
    'to_act20_2010_amending_copy': (90497, 'e4e9001f74bf6c87f9713132f054fe9f638b02e14014794e74383cc03cf7db5b'),
    'to_la_amendment_no2_act21_2010': (43541, 'ddd1ce7878a5af23926f577fd89d8d1dca06ee582de8cac9859e1a57450b849e'),
    'to_la_20101206_interim_note_archived': (50753, 'ab47cbb71915929c87f4d3e47d0cb4bec7cc892cf9cebd4f6b236bdd07b888f9'),
    'to_la_20101221_lasike_elected': (19549, '96a30037da923cf8ece0e760f1f27187fb9917769ffa7c81f1abca32bd870653'),
    'to_la_2011_sitting_announced': (45940, '7de3abfe22ba9f44af141654aa8ead8c6d6e47de848b4b43f8ee6f03161a7670'),
    'to_la_20110114_oaths': (19349, 'feca9a69fc29a75ad6ab2c7a443264425fc7ebd78ee96eaac0d6a172eab5b3c9'),
    'to_la_20110404_knesset_visit': (20625, '5bb4d01936cae4b74738efb2a27563c81024db6e5a2e55a6cf18a67047b0205b'),
    'to_sc_r_v_lasike_cr285_2011': (3738281, 'dc116598fedc78f286d6ee6403dbba94901324f641e97942f2141917c37a1746'),
    'to_la_20120717_speaker_revoked': (31363, '86e849be93a43fc3b5351694a72c2a3cc4d3f5e35773c0e27e8cccca019c1638'),
    'to_mic_20120719_fakafanua_elected': (33685, '25a28ad763fbb68c1b4703fa2893a60431b1f91457d2df7902882cd565ba0e60'),
    'to_la_20120719_fili_sea': (14274, '31da815fb10ef0ddbad9af4d4c8c7402cb3d618f6f0276b7e177ce6b26d4794e'),
    'to_la_20120720_fakafanua_appointed': (15678, '7b4fecd84175d925ac67521567f682de6f3ef27e3a74eff03dd1abae7ff79e98'),
    'to_la_20120723_new_speaker_starts': (14950, 'cb7ac33429d3bf181754741cce82c6b65cf8a573c3883ae0912a804d24b943d6'),
    'to_la_20130828_acting_speaker': (18359, '2fd5e32153efdf65f2b5b261cf0946aef6bea2208f547353a9a0e6b762fff9a9'),
    'to_la_20130905_acting_speaker_quorum': (35848, '533663b52220b88f7952312c92a04b76c8da66100246f4dd9203831faf0dd2ff'),
    'to_la_20140815_speaker_judiciary': (37839, '85dd999b679e26be4360335c6fd32394ca54780b716c650c04577d197b9b9ec4'),
    'to_la_news_speaker_china_20150224': (43468, '73c01ed5fdda819262b92a00e4b689d52c389db3ef34709060bdecfb72c7f82f'),
    'to_la_news_mps_sworn_20180118': (36585, '64fc85aa31068343783d860e07579bbd000536f3210189aca88e0a0ea80cc0b5'),
    'to_la_news_speaker_gita_20180305': (37874, '0cd4e36b223392a50dae1601e5786045cbcc01ce22b6d2a76940400554d10198'),
    'to_la_news_pm_designate_20211215': (41088, '74074abe0159e63169bd94de51c5cdbfae8a0ff570ec27f512396d042210adef'),
    'to_la_news_opening_20220111': (39214, '0d5fa34e461426e5c01c76a70d2e68c1563cbaa59c2b5cff271673e6f3c7de74'),
    'to_la_news_speaker_summit_20220217': (40352, '70dcd37b3d1ea0bfde73080558932c5c518bbbdaa0db2a76d8c790109d3d2835'),
    'to_la_notice_pm_meeting_20251212': (57442, '74b015598df5b8f43cfcd74ab5f40ceb2237ce79f80a1411462c8df8355258dc'),
    'to_la_news_pm_designate_20251216': (56646, '5172843be994ee9ce0b1bb8e05022a07f4ab56534ef1c08ce2da43a179579105'),
    'to_la_release_speaker_appointment_to_20251219': (57823, 'a1898f95b6c34ab01b63defabcb2457353d451ef758eade81998c230be43f57c'),
    'to_la_news_speaker_sworn_20260122': (54760, 'fe079867cec9c37f93a66b24b34bd59aa1842cff418da56c21a2315e015c9b4e'),
    'to_la_news_china_visit_20260519': (60118, '2bade793d09cf667e62b52552ccfca74c1aafab87c95afa7282b4607784c0776'),
    'to_la_news_ministers_oath_20260818': (58734, 'c584714726d6f59cc766cbc2156e7cd91ccb8b2f4fc25cc24856e55ac4a566f4'),
}
NEW_SOURCES = set(RESPONSES)
ORDER = list(RESPONSES)
# Captures the archive stores gzip-encoded: the compressed transfer identity, recorded separately.
TRANSFER = {
    'to_la_news_pm_designate_20251216': (15626, 'b293d5eb12dbf308b2509cd2da1dbef749e2668f711ba5d9889f1e1b9aa4c8b9'),
    'to_la_news_speaker_sworn_20260122': (15148, 'e2cb8e69bab4ed42a2b87a8da0a196aa8d63c278d63e863547ed02a597923479'),
    'to_la_news_ministers_oath_20260818': (16119, '1c4ed70883d9432d79d83978c5e5cbe7d2b4546014b1b2390a001a118e7349d9'),
}
# Raw Internet Archive captures: source id -> capture timestamp.
ARCHIVED = {
    'to_pmo_legislative_assembly_page_2001': '20010430084337',
    'to_pmo_legislative_assembly_page_20011214': '20011214102436',
    'to_la_ministers_page_2005': '20070326123611',
    'to_pmo_20020523_tuivakano_speaker': '20020603034323',
    'to_pmo_legislative_assembly_page_20030408': '20030408005903',
    'to_la_home_2004': '20040906221508',
    'to_pmo_legislative_assembly_page_20041226': '20041226045411',
    'to_la_speaker_page_2005': '20070625041819',
    'to_ipu_2005': '20241107233206',
    'to_pmo_20060209_veikune_directives': '20070930153712',
    'to_la_archives_2006': '20060327202443',
    'to_pmo_20080602_speaker_reply': '20090326123327',
    'to_pmo_20100629_assembly_transcript': '20111130052033',
    'to_la_profile_tuilakepa_2011': '20110203113618',
    'to_la_profile_tuivakano_2011': '20110203111152',
    'to_la_speakers_list_20110203': '20110203112202',
    'to_la_speakers_list_20111130': '20111130035919',
    'to_la_obituary_fusitua_20140505': '20220114203857',
    'to_la_20101206_interim_note_archived': '20111130041654',
    'to_la_20101221_lasike_elected': '20110518092758',
    'to_la_2011_sitting_announced': '20111130041631',
    'to_la_20110114_oaths': '20110518092811',
    'to_la_20110404_knesset_visit': '20110518043655',
    'to_la_20120717_speaker_revoked': '20130411091047',
    'to_mic_20120719_fakafanua_elected': '20130422014223',
    'to_la_20120719_fili_sea': '20121030211303',
    'to_la_20120720_fakafanua_appointed': '20121030211302',
    'to_la_20120723_new_speaker_starts': '20121030211259',
    'to_la_20130828_acting_speaker': '20130830220536',
    'to_la_20130905_acting_speaker_quorum': '20141008214220',
    'to_la_20140815_speaker_judiciary': '20141008211705',
    'to_la_news_speaker_china_20150224': '20150702052047',
    'to_la_news_mps_sworn_20180118': '20180302230312',
    'to_la_news_speaker_gita_20180305': '20180322080929',
    'to_la_news_pm_designate_20211215': '20211216000713',
    'to_la_news_opening_20220111': '20220111080954',
    'to_la_news_speaker_summit_20220217': '20220217040741',
    'to_la_notice_pm_meeting_20251212': '20251212191842',
    'to_la_news_pm_designate_20251216': '20251216013725',
    'to_la_release_speaker_appointment_to_20251219': '20251219093117',
    'to_la_news_speaker_sworn_20260122': '20260122035351',
    'to_la_news_china_visit_20260519': '20260519102917',
    'to_la_news_ministers_oath_20260818': '20260818052147',
}
# PDF pages rendered and visually checked for this packet.
PDF_PAGES = {
    'to_tlr_1997': [27, 30, 135, 136],
    'to_tlr_2003': [235],
    'to_tlr_2007': [230],
    'to_act20_2010_amending_copy': [5, 12, 17],
    'to_la_amendment_no2_act21_2010': [5, 6, 7, 8],
    'to_sc_r_v_lasike_cr285_2011': [7, 16],
}
# Rows whose source names nobody carry a null holder_name.
NAMELESS = {
    'to_act20_2010_clause61_speaker_appointment_procedure',
    'to_act20_2010_clause61_speaker_tenure_and_vacancy',
    'to_act20_2010_royal_assent_printed',
    'to_act20_2010_schedule_interim_speaker_procedure',
    'to_fakafanua_adjourned_to_inform_king_2012',
    'to_fakafanua_royal_appointment_pending_20120719',
    'to_interim_speaker_office_ceases_rule_2010',
    'to_la_act21_2010_royal_assent_printed',
    'to_la_act21_2010_section15_speaker_procedure',
    'to_la_act21_2010_section16_deputy_presides_procedure',
    'to_speaker_election_meeting_notice_20251212',
    'to_speaker_successor_expected_20060209',
}
# The event kind of every row.
KINDS = {
    'to_acting_speaker_lasike_19950802': 'acting_service',
    'to_speaker_fusitua_interview_19961114': 'attested_in_office',
    'to_speaker_fusitua_material_times_19961114': 'attested_in_office',
    'to_speaker_warrant_fusitua_19960920': 'attested_in_office',
    'to_speaker_veikune_listed_20010430': 'attested_in_office',
    'to_speaker_veikune_listed_20011214': 'attested_in_office',
    'to_la_ministers_page_tuivakano_appointed_20020701': 'retrospective_statement',
    'to_la_ministers_page_malupo_acting_2000': 'acting_service',
    'to_speaker_tuivakano_appointment_reported_20020523': 'appointment_reported',
    'to_speaker_tuivakano_listed_20030408': 'attested_in_office',
    'to_speaker_tuivakano_listed_20040906': 'attested_in_office',
    'to_speaker_tuivakano_listed_20041226': 'attested_in_office',
    'to_speaker_veikune_royal_appointment_20050322': 'royal_appointment',
    'to_speaker_veikune_prior_term_retro_20070625': 'retrospective_statement',
    'to_ipu_2005_speaker_veikune_order_20050323': 'attested_in_office',
    'to_ipu_2005_tuihaangana_appointment_effective_20060210': 'appointment_effective_reported',
    'to_speaker_veikune_end_20060125': 'end_of_office',
    'to_palace_confirms_veikune_speaker_end_20060203': 'confirmation_of_end',
    'to_speaker_successor_expected_20060209': 'successor_expected',
    'to_la_archives_fusitua_1995_1998_20060327': 'retrospective_list',
    'to_la_archives_veikune_1998_2001_2005_20060327': 'retrospective_list',
    'to_la_archives_tuivakano_2001_2004_20060327': 'retrospective_list',
    'to_speaker_veikune_removal_recited_20071213': 'retrospective_statement',
    'to_speaker_tuilakepa_reply_signed_20080602': 'attested_in_office',
    'to_speaker_tuilakepa_presides_20100629': 'attested_in_office',
    'to_la_profile_tuilakepa_appointed_2008_20100915': 'retrospective_statement',
    'to_la_profile_tuivakano_appointed_20020701': 'retrospective_statement',
    'to_la_list_tuihaangana_2006_2007_20110203': 'retrospective_list',
    'to_la_list_malupo_1987_1989_20111130': 'retrospective_list',
    'to_la_list_fusitua_1990_1998_20111130': 'retrospective_list',
    'to_la_list_veikune_1999_2001_2005_2006_20111130': 'retrospective_list',
    'to_la_list_tuivakano_2002_2004_20111130': 'retrospective_list',
    'to_la_list_tuihaangana_2006_2008_20111130': 'retrospective_list',
    'to_la_list_tuilakepa_2008_2010_20111130': 'retrospective_list',
    'to_la_list_lasike_2010_20111130': 'retrospective_list',
    'to_la_list_tupou_2010_20111130': 'retrospective_list',
    'to_la_obituary_fusitua_appointed_1991_20140505': 'retrospective_statement',
    'to_act20_2010_clause61_speaker_appointment_procedure': 'procedure',
    'to_act20_2010_clause61_speaker_tenure_and_vacancy': 'procedure',
    'to_act20_2010_schedule_interim_speaker_procedure': 'procedure',
    'to_act20_2010_royal_assent_printed': 'enactment_printed',
    'to_la_act21_2010_section15_speaker_procedure': 'procedure',
    'to_la_act21_2010_section16_deputy_presides_procedure': 'procedure',
    'to_la_act21_2010_royal_assent_printed': 'enactment_printed',
    'to_interim_speaker_tupou_styled_20101203': 'interim_service',
    'to_interim_speaker_office_ceases_rule_2010': 'procedure',
    'to_lasike_assembly_selection_unopposed_20101221': 'assembly_election',
    'to_lasike_royal_appointment_announced_20101221': 'appointment_announced_future',
    'to_lasike_announces_first_sitting_2011': 'meeting_notice',
    'to_lasike_speaker_administers_oaths_20110114': 'attested_in_office',
    'to_lasike_speaker_styled_20110404': 'attested_in_office',
    'to_lasike_testimony_speaker_cr285': 'self_description',
    'to_lasike_supreme_court_conviction_20120709': 'court_conviction',
    'to_lasike_speaker_appointment_revoked_20120717': 'revocation_of_appointment',
    'to_lasike_ceased_elected_representative_20120709': 'loss_of_seat',
    'to_fakafanua_assembly_election_20120719': 'assembly_election',
    'to_tuihaateiho_acting_speaker_20120719': 'acting_service',
    'to_fakafanua_assembly_recommendation_20120719': 'assembly_recommendation',
    'to_fakafanua_royal_appointment_pending_20120719': 'appointment_announced_future',
    'to_fakafanua_royal_appointment_reported_20120720': 'royal_appointment_reported',
    'to_fakafanua_adjourned_to_inform_king_2012': 'adjourned_to_report_result_to_king',
    'to_fakafanua_royal_appointment_20120719': 'royal_appointment',
    'to_fakafanua_starts_as_speaker_20120723': 'stated_assumption_of_duties',
    'to_tuihaateiho_acting_speaker_20130828': 'acting_service',
    'to_tuihaateiho_resignation_announced_20130827': 'resignation_announced',
    'to_tuihaateiho_resumed_acting_speaker_20130828': 'acting_service_resumed',
    'to_fakafanua_speaker_abroad_20130828': 'attested_in_office',
    'to_tuihaateiho_acting_speaker_20130904': 'acting_service',
    'to_fakafanua_speaker_20140814': 'attested_in_office',
    'to_speaker_tuivakano_styled_20150224': 'attested_in_office',
    'to_speaker_fakafanua_sworn_news_20180118': 'oath',
    'to_speaker_fakafanua_styled_20180305': 'attested_in_office',
    'to_speaker_fakafanua_assembly_election_20211215': 'assembly_election',
    'to_acting_speaker_tuihaangana_20220111': 'acting_service',
    'to_speaker_fakafanua_styled_20220217': 'attested_in_office',
    'to_speaker_election_meeting_notice_20251212': 'meeting_notice',
    'to_speaker_vaea_assembly_election_20251216': 'assembly_election',
    'to_interim_speaker_tangi_presided_20251216': 'interim_service',
    'to_speaker_vaea_royal_appointment_to_20251218': 'royal_appointment',
    'to_speaker_vaea_oath_news_20260122': 'oath',
    'to_speaker_vaea_led_delegation_20260519': 'attested_in_office',
    'to_acting_speaker_tuihaangana_20260818': 'acting_service',
}

STRING_HOLDER = 'to_speakers_appointment'
FUSITUA, VEIKUNE, TUIVAKANO_HON, TUILAKEPA = "Fusitu'a", 'Hon. Veikune', "Hon. Tu'ivakano", "Lord Tu'ilakepa"
LASIKE, FAKAFANUA, TUIVAKANO, VAEA = 'Lord Lasike', 'Lord Fakafanua', "Lord Tu'ivakano", 'Lord Vaea'
NAMES = [FUSITUA, VEIKUNE, TUIVAKANO_HON, VEIKUNE, TUILAKEPA, LASIKE, FAKAFANUA, TUIVAKANO, FAKAFANUA, FAKAFANUA, VAEA]
# (attested_on, from, until) of the eleven holders this packet adds after the string observation, in list order.
HOLDERS = [
    ('1996-11-14', None, None),
    ('2001-04-30', None, None),
    ('2003-04-08', None, None),
    ('2005-03-22', None, '2006-01-25'),
    ('2008-06-02', None, None),
    ('2011-01-14', None, '2012-07-17'),
    ('2012-07-19', None, None),
    ('2015-02-24', None, None),
    ('2018-03-05', None, None),
    ('2022-02-17', None, None),
    ('2026-05-19', None, None),
]
HOLDER_CLAIMS = [
    ['to_speaker_warrant_fusitua_19960920', 'to_speaker_fusitua_interview_19961114', 'to_speaker_fusitua_material_times_19961114'],
    ['to_speaker_veikune_listed_20010430', 'to_speaker_veikune_listed_20011214'],
    ['to_speaker_tuivakano_listed_20030408', 'to_speaker_tuivakano_listed_20040906', 'to_speaker_tuivakano_listed_20041226'],
    ['to_speaker_veikune_royal_appointment_20050322', 'to_ipu_2005_speaker_veikune_order_20050323',
     'to_speaker_veikune_end_20060125', 'to_palace_confirms_veikune_speaker_end_20060203'],
    ['to_speaker_tuilakepa_reply_signed_20080602', 'to_speaker_tuilakepa_presides_20100629'],
    ['to_lasike_speaker_administers_oaths_20110114', 'to_lasike_speaker_styled_20110404', 'to_lasike_testimony_speaker_cr285',
     'to_lasike_speaker_appointment_revoked_20120717'],
    ['to_fakafanua_royal_appointment_20120719', 'to_fakafanua_royal_appointment_reported_20120720',
     'to_fakafanua_starts_as_speaker_20120723', 'to_fakafanua_speaker_abroad_20130828', 'to_fakafanua_speaker_20140814'],
    ['to_speaker_tuivakano_styled_20150224'],
    ['to_speaker_fakafanua_styled_20180305'],
    ['to_speaker_fakafanua_styled_20220217'],
    ['to_speaker_vaea_led_delegation_20260519'],
]
HOLDER_SOURCES = [
    ['to_tlr_2003', 'to_tlr_1997'],
    ['to_pmo_legislative_assembly_page_2001', 'to_pmo_legislative_assembly_page_20011214'],
    ['to_pmo_legislative_assembly_page_20030408', 'to_la_home_2004', 'to_pmo_legislative_assembly_page_20041226'],
    ['to_la_speaker_page_2005', 'to_ipu_2005', 'to_pmo_20060209_veikune_directives'],
    ['to_pmo_20080602_speaker_reply', 'to_pmo_20100629_assembly_transcript'],
    ['to_la_20110114_oaths', 'to_la_20110404_knesset_visit', 'to_sc_r_v_lasike_cr285_2011', 'to_la_20120717_speaker_revoked'],
    ['to_la_20120723_new_speaker_starts', 'to_la_20120720_fakafanua_appointed', 'to_la_20130828_acting_speaker',
     'to_la_20140815_speaker_judiciary'],
    ['to_la_news_speaker_china_20150224'],
    ['to_la_news_speaker_gita_20180305'],
    ['to_la_news_speaker_summit_20220217'],
    ['to_la_news_china_visit_20260519'],
]
# Acting Speakers, Interim Speakers and the Deputy Speaker presiding: claims on the role, never holders.
ACTING = ('to_acting_speaker_lasike_19950802', 'to_la_ministers_page_malupo_acting_2000', 'to_interim_speaker_tupou_styled_20101203',
          'to_tuihaateiho_acting_speaker_20120719', 'to_tuihaateiho_acting_speaker_20130828',
          'to_tuihaateiho_resumed_acting_speaker_20130828', 'to_tuihaateiho_acting_speaker_20130904',
          'to_acting_speaker_tuihaangana_20220111', 'to_interim_speaker_tangi_presided_20251216',
          'to_acting_speaker_tuihaangana_20260818')
# Claims that must never feed a holder: acting and interim service, the Assembly's elections and recommendations, oaths,
# announcements and notices, procedure, retrospective lists and statements, IPU's effective date for Tu'iha'angana,
# conviction, loss of seat, resignation, the adjournment and the Tongan version of the 2025 appointment.
NEVER_HOLDER = ACTING + (
    'to_lasike_assembly_selection_unopposed_20101221', 'to_lasike_royal_appointment_announced_20101221',
    'to_lasike_announces_first_sitting_2011', 'to_fakafanua_assembly_election_20120719',
    'to_fakafanua_assembly_recommendation_20120719', 'to_fakafanua_royal_appointment_pending_20120719',
    'to_fakafanua_adjourned_to_inform_king_2012', 'to_speaker_fakafanua_assembly_election_20211215',
    'to_speaker_election_meeting_notice_20251212', 'to_speaker_vaea_assembly_election_20251216',
    'to_speaker_fakafanua_sworn_news_20180118', 'to_speaker_vaea_oath_news_20260122',
    'to_speaker_vaea_royal_appointment_to_20251218', 'to_act20_2010_clause61_speaker_appointment_procedure',
    'to_act20_2010_clause61_speaker_tenure_and_vacancy', 'to_act20_2010_schedule_interim_speaker_procedure',
    'to_act20_2010_royal_assent_printed', 'to_la_act21_2010_section15_speaker_procedure',
    'to_la_act21_2010_section16_deputy_presides_procedure', 'to_la_act21_2010_royal_assent_printed',
    'to_interim_speaker_office_ceases_rule_2010', 'to_la_list_malupo_1987_1989_20111130', 'to_la_list_fusitua_1990_1998_20111130',
    'to_la_list_veikune_1999_2001_2005_2006_20111130', 'to_la_list_tuivakano_2002_2004_20111130',
    'to_la_list_tuihaangana_2006_2008_20111130', 'to_la_list_tuilakepa_2008_2010_20111130', 'to_la_list_lasike_2010_20111130',
    'to_la_list_tupou_2010_20111130', 'to_la_list_tuihaangana_2006_2007_20110203', 'to_la_archives_fusitua_1995_1998_20060327',
    'to_la_archives_veikune_1998_2001_2005_20060327', 'to_la_archives_tuivakano_2001_2004_20060327',
    'to_la_obituary_fusitua_appointed_1991_20140505', 'to_la_profile_tuivakano_appointed_20020701',
    'to_la_ministers_page_tuivakano_appointed_20020701', 'to_la_profile_tuilakepa_appointed_2008_20100915',
    'to_speaker_veikune_prior_term_retro_20070625', 'to_speaker_veikune_removal_recited_20071213',
    'to_speaker_tuivakano_appointment_reported_20020523', 'to_ipu_2005_tuihaangana_appointment_effective_20060210',
    'to_speaker_successor_expected_20060209', 'to_lasike_supreme_court_conviction_20120709',
    'to_lasike_ceased_elected_representative_20120709', 'to_tuihaateiho_resignation_announced_20130827')
# Dates that must never be a holder date or boundary: acting and interim days, the contested 2002 appointment dates,
# confirmations, expectations, IPU's effective date, recitals, selections and elections, the conviction, reported and
# first-presiding days, oaths, notices, the 2025 effective date already held by the string observation, and the
# observation dates of retrospective lists and statements.
NOT_BOUNDARIES = {'1995-08-02', '2002-05-23', '2002-07-01', '2006-02-03', '2006-02-09', '2006-02-10', '2007-12-13',
                  '2010-12-03', '2010-12-21', '2012-07-09', '2012-07-20', '2012-07-23', '2013-08-27', '2013-08-28',
                  '2013-09-04', '2018-01-18', '2021-12-15', '2022-01-11', '2025-12-12', '2025-12-16', '2025-12-18',
                  '2026-01-22', '2026-08-18', '2011-11-30', '2011-02-03', '2006-03-27', '2014-05-05', '2010-09-15',
                  '2007-06-25', '1990-01-01'}
EVENT_KINDS = {'acting_service', 'acting_service_resumed', 'adjourned_to_report_result_to_king', 'appointment_announced_future',
               'appointment_effective_reported', 'appointment_reported', 'assembly_election', 'assembly_recommendation',
               'attested_in_office', 'confirmation_of_end', 'court_conviction', 'end_of_office', 'enactment_printed',
               'interim_service', 'loss_of_seat', 'meeting_notice', 'oath', 'procedure', 'resignation_announced',
               'retrospective_list', 'retrospective_statement', 'revocation_of_appointment', 'royal_appointment',
               'royal_appointment_reported', 'self_description', 'stated_assumption_of_duties', 'successor_expected'}
# Leads named in the report that must never become sources: encyclopaedias, news, republications, per-Speaker pages,
# the download-form minutes and the court records that do not mention the Speakership.
LEAD_URL_MARKERS = ('wikipedia', 'rnz.co.nz', 'rnzi.com', 'matangitonga', '3919-fokotuu', '156-deputy-speaker', 'timeline-box',
                    '1573:1996_tlr', 'download=771', 'speakers-of-the-house/2', 'BROCHURE', '247-chinas-top-legislator',
                    '2014-speaker-of-parliament-lord-lasike', 'parliament-pays-tribute', 'rules_of_procedures',
                    'lord-speaker-and-mps-sworn-in-2')
REPORT = research.RESEARCH / 'tonga-speakers-1990-2026-24.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-24.md'


def speaker_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError or KeyError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    entry = next(e for e in packet['institutions'] if e['id'] == 'to_legislative_assembly')
    assert [r['id'] for r in entry['roles']] == ['to_speaker', 'to_deputy_speaker', 'to_peoples_representatives',
                                                 'to_nobles_representatives'], 'no new legislature role'
    role = entry['roles'][0]
    assert [h for h in role['holder_claims'] if isinstance(h, str)] == [STRING_HOLDER], 'one string observation'
    assert role['holder_claims'][0] == STRING_HOLDER, 'the string observation stays first'
    holders = [h for h in role['holder_claims'] if isinstance(h, dict)]
    assert [h['name'] for h in holders] == NAMES, 'exactly the intended holders, in order'
    for holder, expected, cids, sids in zip(holders, HOLDERS, HOLDER_CLAIMS, HOLDER_SOURCES):
        got = (holder['attested_on'], holder['from'], holder['until'])
        assert got == expected, (holder['name'], got)
        assert 'attested_period' not in holder, holder['name']
        assert holder['claim_ids'] == cids and holder['sources'] == sids, holder['name']
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), holder['name']
        assert not {holder['attested_on'], holder['from'], holder['until']} & NOT_BOUNDARIES, holder['name']
        for word in ('Acting', 'Interim', 'Tangi', 'Tupou', "Tu'iha'ateiho", "Tu'iha'angana", 'Malupo', 'Honourable'):
            assert word not in holder['name'], (word, holder['name'])
    dated = [h['attested_on'] for h in holders]
    assert dated == sorted(dated), 'chronological order'
    for cid in ACTING + NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
    # The existing string observation, its claim and the Deputy Speaker role are unchanged.
    assert claims[STRING_HOLDER] == {
        'id': STRING_HOLDER,
        'text': "King Tupou VI appointed Lord Vaea Speaker and Lord Tu'iha'angana Deputy Speaker, with both instruments "
                "effective 18 December 2025. The release was issued the following day.",
        'locator': 'Appointment announcement', 'period': {'from': '2025-12-18', 'through': '2025-12-18'}}
    deputy = entry['roles'][1]
    assert (deputy['sources'], deputy['claim_ids'], deputy['holder_claims']) == (
        ['to_speaker_appointment_2025'], [STRING_HOLDER], [STRING_HOLDER]), 'to_deputy_speaker is not extended'
    # Distinct dated events stay distinct claims.
    assert claims['to_lasike_assembly_selection_unopposed_20101221']['attested_on'] == '2010-12-21'
    assert claims['to_lasike_speaker_administers_oaths_20110114']['attested_on'] == '2011-01-14'
    assert claims['to_lasike_supreme_court_conviction_20120709']['attested_on'] == '2012-07-09'
    assert claims['to_lasike_speaker_appointment_revoked_20120717']['attested_on'] == '2012-07-17'
    assert claims['to_fakafanua_assembly_election_20120719']['attested_on'] == '2012-07-19'
    assert claims['to_fakafanua_royal_appointment_20120719']['attested_on'] == '2012-07-19'
    assert claims['to_fakafanua_royal_appointment_reported_20120720']['attested_on'] == '2012-07-20'
    assert claims['to_fakafanua_starts_as_speaker_20120723']['attested_on'] == '2012-07-23'
    assert claims['to_speaker_vaea_assembly_election_20251216']['attested_on'] == '2025-12-16'
    assert claims['to_speaker_vaea_royal_appointment_to_20251218']['attested_on'] == '2025-12-18'
    assert claims['to_speaker_vaea_oath_news_20260122']['attested_on'] == '2026-01-22'
    # Undated or year-only claims carry no structured date.
    for cid in ('to_la_ministers_page_malupo_acting_2000', 'to_lasike_testimony_speaker_cr285', 'to_lasike_announces_first_sitting_2011',
                'to_fakafanua_adjourned_to_inform_king_2012',
                'to_act20_2010_royal_assent_printed', 'to_la_act21_2010_royal_assent_printed',
                'to_act20_2010_clause61_speaker_appointment_procedure', 'to_interim_speaker_office_ceases_rule_2010'):
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid], cid


class TongaSpeakerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'tonga.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.assembly = cls.entries['to_legislative_assembly']
        cls.role = cls.roles['to_speaker']
        cls.holders = [h for h in cls.role['holder_claims'] if isinstance(h, dict)]
        cls.extracts = {
            sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
            for sid in NEW_SOURCES
        }
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')
        cls.new_claims = [c['id'] for sid in ORDER for c in cls.sources[sid]['claims']]

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Tonga'}, {'Tonga': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_reuse_ids_and_every_claim_is_cited(self):
        ids = self.validate()
        self.assertLessEqual(NEW_SOURCES, set(ids['sources']))
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims), len(set(self.new_claims))), (49, 82, 82))
        # The new sources follow every earlier packet's, in this packet's order.
        self.assertEqual([s['id'] for s in self.packet['sources'][119:]], ORDER)
        self.assertEqual(list(self.sources)[118], 'to_gazette_ext_28_2019')
        # Every new claim sits on the Speaker role and the Assembly entry, and no other entry or role cites one.
        self.assertEqual(self.role['claim_ids'], ['to_constitution_assembly', STRING_HOLDER] + self.new_claims)
        self.assertEqual(self.role['sources'], ['to_constitution_2020', 'to_speaker_appointment_2025'] + ORDER)
        self.assertEqual(self.assembly['claim_ids'][-82:], self.new_claims)
        self.assertEqual(self.assembly['sources'][-49:], ORDER)
        for eid, entry in self.entries.items():
            if eid != 'to_legislative_assembly':
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), eid)
        for rid, role in self.roles.items():
            if rid != 'to_speaker':
                self.assertFalse(set(self.new_claims) & set(role['claim_ids']), rid)
        # No new organization, institution or role; existing identities are reused.
        self.assertEqual(len(ids['entries']), 9)
        self.assertEqual({e['id'] for e in self.packet['institutions']},
                         {'to_crown', 'to_prime_minister', 'to_cabinet', 'to_privy_council', 'to_legislative_assembly'})
        observations = re.findall(r'^### (TO-SPK-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'TO-SPK-{n:02d}' for n in range(1, 11)])
        by_observation = {}
        for sid in ORDER:
            extract = self.extracts[sid]
            for row in extract['rows']:
                by_observation.setdefault(row['review_observation'], []).append(row['claim_id'])
        self.assertEqual(sorted(by_observation), [f'TO-SPK-{n:02d}' for n in range(1, 11)])
        # Every structured date lies inside the period and before the cutoff.
        for cid in self.new_claims:
            value = self.claims[cid].get('attested_on')
            if value:
                self.assertTrue('1990-01-01' <= value <= research.CUTOFF, cid)
            self.assertNotIn('period', self.claims[cid])
        for sid in ORDER:
            self.assertLessEqual(self.sources[sid].get('published_date', ''), research.CUTOFF)

    def test_holders_are_exactly_as_intended(self):
        speaker_invariants(self.packet)
        self.assertEqual(len(self.role['holder_claims']), 12)
        for holder in self.holders:
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            for cid in holder['claim_ids']:
                self.assertIn(self.claim_source[cid], holder['sources'])
        # Tu'iha'angana (2006-2008) has claims but no holder: the Crown packet's gazette claim is not cited here.
        self.assertFalse(any("Tu'iha'angana" in h['name'] for h in self.holders))
        self.assertIn('to_ipu_2005_tuihaangana_appointment_effective_20060210', self.role['claim_ids'])
        crown_claims = {c['id'] for sid in ('to_pmo_20060911_proclamation', 'to_gazette_ext_9_2012', 'to_ipu_2008')
                        for c in self.sources[sid]['claims']}
        self.assertFalse(crown_claims & set(self.role['claim_ids']))
        self.assertFalse(crown_claims & set(self.assembly['claim_ids']))
        # No holder anywhere else names a Speaker-role acting or interim officer.
        for role_id, role in self.roles.items():
            for entry in role['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertFalse(set(ids) & set(ACTING), role_id)
        self.assertIn('never holders or boundaries', self.role['scope_note'])
        self.assertIn(STRING_HOLDER, self.role['scope_note'])

    def test_appointment_election_oath_revocation_and_acting_service_never_collapse(self):
        speaker_invariants(self.packet)
        chain = [self.claims[c]['attested_on'] for c in (
            'to_lasike_assembly_selection_unopposed_20101221', 'to_lasike_speaker_administers_oaths_20110114',
            'to_lasike_supreme_court_conviction_20120709', 'to_lasike_speaker_appointment_revoked_20120717')]
        self.assertEqual(chain, sorted(set(chain)))
        vaea = [self.claims[c]['attested_on'] for c in (
            'to_speaker_election_meeting_notice_20251212', 'to_speaker_vaea_assembly_election_20251216',
            'to_speaker_vaea_royal_appointment_to_20251218', 'to_speaker_vaea_oath_news_20260122',
            'to_speaker_vaea_led_delegation_20260519')]
        self.assertEqual(vaea, sorted(set(vaea)))
        veikune = [self.claims[c]['attested_on'] for c in (
            'to_speaker_veikune_royal_appointment_20050322', 'to_speaker_veikune_end_20060125',
            'to_palace_confirms_veikune_speaker_end_20060203', 'to_speaker_successor_expected_20060209')]
        self.assertEqual(veikune, sorted(set(veikune)))
        # 2012: the Assembly's election, its Tongan recommendation, the dated appointment and its report are four claims.
        july = ('to_fakafanua_assembly_election_20120719', 'to_fakafanua_assembly_recommendation_20120719',
                'to_fakafanua_royal_appointment_20120719', 'to_fakafanua_royal_appointment_reported_20120720')
        self.assertEqual(len({self.claim_source[c] for c in july}), 4)
        self.assertIn("'He was officially APPOINTED by his Majesty King Tupou VI", self.claims[july[2]]['text'])
        self.assertIn('17:23', self.claims[july[2]]['uncertainty'])
        self.assertIn("'toki'", self.claims['to_fakafanua_royal_appointment_pending_20120719']['uncertainty'])
        self.assertIn('conservative alternative is 20 July 2012', self.holders[6]['uncertainty'])
        # Oaths are events, not starts; the 2025 appointment stays the string observation.
        for cid in ('to_speaker_fakafanua_sworn_news_20180118', 'to_speaker_vaea_oath_news_20260122'):
            self.assertIn('not a start', self.claims[cid]['uncertainty'])
        self.assertIn("'na'e kamata ngaue'aki ia mei he 'aho 18 Tīsema 2025'",
                      self.claims['to_speaker_vaea_royal_appointment_to_20251218']['text'])
        self.assertFalse(any(h['from'] == '2025-12-18' or h['attested_on'] == '2025-12-18' for h in self.holders))
        # Acting, interim and resignation claims say what they are.
        for cid in ACTING:
            self.assertRegex(self.claims[cid]['uncertainty'], r'claim only|never a (Speaker )?holder|claim on the role')
        resign = self.claims['to_tuihaateiho_resignation_announced_20130827']['uncertainty']
        self.assertIn('never names the post', resign)
        self.assertIn("'After the tea break", self.claims['to_tuihaateiho_resumed_acting_speaker_20130828']['text'])
        # Procedure and enactment metadata carry no date; statutes date nothing.
        for cid in ('to_act20_2010_royal_assent_printed', 'to_la_act21_2010_royal_assent_printed'):
            self.assertIn('no structured date', self.claims[cid]['uncertainty'])
        # A successor expected is not a vacancy; the IPU effective date is not a consent day.
        self.assertIn('does not say that the Speakership is vacant',
                      self.claims['to_speaker_successor_expected_20060209']['uncertainty'])
        self.assertIn('not the day of the royal consent',
                      self.claims['to_ipu_2005_tuihaangana_appointment_effective_20060210']['uncertainty'])
        # Printed names are kept as printed.
        self.assertIn("'Hon. Tu'ivakano (Speaker of the House)'", self.claims['to_speaker_tuivakano_listed_20040906']['text'])
        self.assertIn("'Fusitu'a was at all material times the Speaker of the Supreme Law making Body of Tonga'",
                      self.claims['to_speaker_fusitua_material_times_19961114']['text'])
        self.assertIn("'Lord Lasike will beformally appointed by the King' [sic]",
                      self.claims['to_lasike_royal_appointment_announced_20101221']['text'])
        for stale in ('to_speaker_vacancy_20060209', 'to_speaker_veikune_removed_20071213',
                      'to_ipu_2005_tuihaangana_consent_20060210', 'to_fakafanua_ballot_reported_to_king_2012',
                      'to_act20_2010_royal_assent_printed_20101124', 'to_la_act21_2010_royal_assent_printed_20101124',
                      'to_mic_20120719_fakafanua_recommended_to', 'to_la_minutes_01_20220113', 'to_la_minutes_02_20220602',
                      'to_la_minutes_01_20260122', 'to_la_minutes_15_20260813', 'to_la_minutes_16_20260818'):
            self.assertNotIn(stale, self.raw)

    def test_ends_only_where_a_source_states_the_day(self):
        ends = [(h['name'], h['until']) for h in self.holders if h['until']]
        self.assertEqual(ends, [(VEIKUNE, '2006-01-25'), (LASIKE, '2012-07-17')])
        self.assertEqual([h['from'] for h in self.holders if h['from']], [])
        self.assertEqual(self.holders[3]['until'], self.claims['to_speaker_veikune_end_20060125']['attested_on'])
        self.assertIn("'his position as the Speaker of the House ended on 25th January, 2006'",
                      self.claims['to_speaker_veikune_end_20060125']['text'])
        self.assertEqual(self.holders[5]['until'], self.claims['to_lasike_speaker_appointment_revoked_20120717']['attested_on'])
        self.assertIn("Tonga effective immediately'", self.claims['to_lasike_speaker_appointment_revoked_20120717']['text'])
        # No end from a successor's appointment, a retrospective span, a conviction or a description.
        self.assertIn('none is inferred', self.holders[2]['uncertainty'])
        self.assertIn('not used as the end', self.holders[5]['uncertainty'])
        self.assertIn('nothing is inferred', self.holders[6]['uncertainty'])
        self.assertIn('not end dates', self.holders[9]['uncertainty'])
        unresolved = self.assembly['coverage']['unresolved']
        rows = [u for u in unresolved if u.startswith('TO-SPK-')]
        self.assertEqual([u.split(' ', 1)[0].rstrip(':') for u in rows],
                         ['TO-SPK-01', 'TO-SPK-02/03', 'TO-SPK-05/06', 'TO-SPK-07'])
        self.assertTrue(unresolved[0].startswith('Build dated membership, Speakers, acting Speakers'))
        self.assertEqual(sum('Speaker packet 24' in u for u in self.packet['coverage']['unresolved']), 1)
        self.assertTrue(self.packet['coverage']['unresolved'][-1].startswith('Speaker packet 24'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source.get('published_date'))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-27', '2026-09-27'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertRegex(extract['stability_check'], r'(\d+) minutes later')
            self.assertGreaterEqual(int(re.search(r'(\d+) minutes later', extract['stability_check']).group(1)), 30)
            # Rows repeat the packet claims exactly, keyed by claim_id, with a holder_name and never a name key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertNotIn('name', row)
                self.assertEqual((row['observation_id'], row['role_id'], row['role_title']),
                                 ('to_legislative_assembly', 'to_speaker', 'Speaker'))
                self.assertIn(row['event_kind'], EVENT_KINDS)
                self.assertEqual((row['text'], row['locator'], row['uncertainty'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim['uncertainty'], claim.get('attested_on')))
                self.assertIn('holder_name', row)
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/tonga-'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertTrue(source['source_type'] and source['scope_note'])
        # Rows whose source names nobody carry no holder name.
        nameless = {r['claim_id'] for e in self.extracts.values() for r in e['rows'] if r['holder_name'] is None}
        self.assertEqual(nameless, NAMELESS)
        kinds = {r['claim_id']: r['event_kind'] for e in self.extracts.values() for r in e['rows']}
        for cid, kind in KINDS.items():
            self.assertEqual(kinds[cid], kind, cid)
        # Raw Internet Archive captures made before the cutoff, recorded as decoded bodies.
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn("'Accept-Encoding: identity'", extract['fetch_recipe'])
            if sid in TRANSFER:
                self.assertEqual((extract['source_response_transfer_bytes'], extract['source_response_transfer_sha256']),
                                 TRANSFER[sid])
                self.assertIn('decoded body', extract['source_response_content_encoding'])
                self.assertIn('gunzip', extract['fetch_recipe'])
            else:
                self.assertEqual(extract['source_response_content_encoding'], 'identity')
                self.assertNotIn('source_response_transfer_bytes', extract)
        self.assertLessEqual(set(TRANSFER), set(ARCHIVED))
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES - set(ARCHIVED)}, {'ago.gov.to'})
        for sid in NEW_SOURCES - set(ARCHIVED):
            self.assertIn('after the 7 September 2026 historical cutoff', self.sources[sid]['scope_note'])

    def test_no_per_request_or_growing_source_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for marker in ('?start=', 'search', 'finder', 'cb=', 'nocache', 'hansards-debates', 'switch_to_desktop',
                           'tmpl=component', 'print=1', 'data.ipu.org/election-summary/HTML/2317_05.htm?'):
                self.assertNotIn(marker, url, sid)
            if sid not in ARCHIVED:
                self.assertRegex(url, r'^https://ago\.gov\.to/cms/(images/LEGISLATION/AMENDING/2010/2010-002[01]/[A-Za-z.0-9]+\.pdf'
                                      r'|ago-materials/publications/tonga-law-reports\.html\?download=15(59|63|72):(1997|2003|2007)_tlr'
                                      r'|judgements/supreme-court-criminal/category/69-cr-2012\.html\?download=820:[a-z0-9-]+)$')
        # The replaced minutes and the MIC republication are not sources; their facts come from the Assembly's own items.
        self.assertFalse(any('miniti-fika' in s['url'] and s['id'] in NEW_SOURCES for s in self.packet['sources']))

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            if source['id'] in NEW_SOURCES:
                self.assertFalse(any(marker in source['url'] for marker in LEAD_URL_MARKERS), source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'matangi', 'rnz', 'four years', 'xinhua'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('wikipedia.org', 'rnz.co.nz', '3919-fokotuu', '156-deputy-speaker', '1573:1996_tlr', 'download=771',
                       'lorem-ipsum-ii/speakers-of-the-house/216', '247-chinas-top-legislator', 'parliament-pays-tribute'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        attempted = self.section('Sources attempted')
        for marker in ('license_agree', '379-miniti-fika-15', '380-miniti-fika-16', '343-miniti-fika-1', '83-miniti-fika-01',
                       '84-miniti-fika-02', 'paclii'):
            self.assertIn(marker, attempted, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'tonga.json').read_bytes()
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
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == 'to_speaker')

        def holder(packet, index):
            return [h for h in role(packet)['holder_claims'] if isinstance(h, dict)][index]

        validator_cases = [
            (lambda p: source(p, 'to_la_20120717_speaker_revoked')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'to_tlr_1997')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'to_ipu_2005')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 10).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update(until='2026-09-30'), 'exceeds cutoff'),
            (lambda p: claim(p, 'to_acting_speaker_tuihaangana_20260818').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 3).update({'from': '2006-02-01'}), 'Reversed historical interval'),
            (lambda p: holder(p, 6).update(claim_ids=['to_speaker_vaea_led_delegation_20260519']), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('to_speaker_does_not_exist'), 'Unknown'),
            (lambda p: claim(p, 'to_la_ministers_page_malupo_acting_2000').update(period={'from': '2000', 'through': '2000'}),
             '(?i)invalid'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('acting Speaker as holder', lambda p: role(p)['holder_claims'].append({
                'name': "Lord Tu'iha'angana", 'attested_on': '2026-08-18', 'from': None, 'until': None,
                'sources': ['to_la_news_ministers_oath_20260818'], 'claim_ids': ['to_acting_speaker_tuihaangana_20260818']})),
            ('interim Speaker as holder', lambda p: role(p)['holder_claims'].insert(6, {
                'name': 'Lord Tevita Tupou', 'attested_on': '2010-12-03', 'from': None, 'until': None,
                'sources': ['to_la_20101206_interim_note_archived'], 'claim_ids': ['to_interim_speaker_tupou_styled_20101203']})),
            ("Tu'iha'angana from IPU", lambda p: role(p)['holder_claims'].insert(5, {
                'name': "Mr. Tu'iha'angana", 'attested_on': None, 'from': '2006-02-10', 'until': None,
                'sources': ['to_ipu_2005'], 'claim_ids': ['to_ipu_2005_tuihaangana_appointment_effective_20060210']})),
            ('duplicate 2025 appointment holder', lambda p: holder(p, 10).update({'from': '2025-12-18'})),
            ('Assembly selection as start', lambda p: holder(p, 5).update({'from': '2010-12-21'})),
            ('oath as start', lambda p: holder(p, 8).update({'from': '2018-01-18'})),
            ('first presiding day as start', lambda p: holder(p, 6).update({'from': '2012-07-23'})),
            ('reported appointment as the date', lambda p: holder(p, 6).update(attested_on='2012-07-20')),
            ('retrospective list as start', lambda p: holder(p, 0).update({'from': '1990-01-01'})),
            ('contested 2002 date as holder date', lambda p: holder(p, 2).update(attested_on='2002-07-01')),
            ('successor appointment as end', lambda p: holder(p, 2).update(until='2005-03-22')),
            ('conviction as end', lambda p: holder(p, 5).update(until='2012-07-09')),
            ('expected successor as end', lambda p: holder(p, 3).update(until='2006-02-09')),
            ('acting claim cited by a holder', lambda p: holder(p, 6)['claim_ids'].append('to_tuihaateiho_acting_speaker_20130828')),
            ('election cited by a holder', lambda p: holder(p, 9)['claim_ids'].append('to_speaker_fakafanua_assembly_election_20211215')),
            ('list cited by a holder', lambda p: holder(p, 0)['claim_ids'].append('to_la_list_fusitua_1990_1998_20111130')),
            ('string observation moved', lambda p: role(p)['holder_claims'].append(role(p)['holder_claims'].pop(0))),
            ('string observation edited', lambda p: claim(p, STRING_HOLDER).update(text='changed')),
            ('Deputy Speaker extended', lambda p: next(r for e in p['institutions'] for r in e['roles']
                                                       if r['id'] == 'to_deputy_speaker')['claim_ids'].append(
                                                           'to_acting_speaker_tuihaangana_20260818')),
            ('second Speaker role', lambda p: next(e for e in p['institutions'] if e['id'] == 'to_legislative_assembly')[
                'roles'].append({'id': 'to_acting_speaker', 'title': 'Acting Speaker', 'kind': 'institutional_office',
                                 'sources': ['to_la_news_ministers_oath_20260818'],
                                 'claim_ids': ['to_acting_speaker_tuihaangana_20260818'], 'holder_claims': []})),
            ('year-only acting claim given a date', lambda p: claim(p, 'to_la_ministers_page_malupo_acting_2000').update(
                attested_on='2000-01-01')),
            ('holders out of order', lambda p: role(p)['holder_claims'].insert(1, role(p)['holder_claims'].pop(4))),
        ]
        speaker_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError)):
                speaker_invariants(mutated(change))

    def test_submitted_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Unresolved', '02': 'Accepted in part', '03': 'Accepted in part', '04': 'Accepted',
                     '05': 'Accepted in part', '06': 'Accepted in part', '07': 'Accepted in part', '08': 'Accepted in part',
                     '09': 'Accepted in part', '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| TO-SPK-{number} ')]
            self.assertIn(f'**{decision}', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 22)] + [f'B{n}' for n in range(1, 15)] + [f'C{n}' for n in range(1, 14)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Declined|Superseded)')
        self.assertEqual(sum('**Declined' in r for r in rows), 2)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('a91a8249', 'f1230bbb', 'a33a8987', 'ffe54b02', 'research-index.json', 'test_tonga_research_s10g.py',
                     'test_tonga_dpfi_c01_04.py', 'test_tonga_transition_c01_02.py', 'test_tonga_pm_1990_2019_c01_08.py'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'tonga-speakers-1990-2026-24.md', 'claude/c01-to-24', 'a91a8249', 'ffe54b02',
                     'f1230bbb', 'test_tonga_speakers_c01_24.py'):
            self.assertIn(text, handoff)
        self.assertNotIn('State: **claimed**', handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Tonga')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual(country['mapping_pending'], 9)
        batch, = [row for row in index['work_orders'] if row['nation'] == 'Tonga']
        self.assertEqual(batch['status'], 'open')
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
