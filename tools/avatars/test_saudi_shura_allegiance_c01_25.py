"""CLAUDE-C01-25: Saudi Shura Council and Allegiance Commission chairs 1990-2026 keep orders, effective days, oaths, term starts and deaths apart."""
import copy
from datetime import datetime
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


REPORT = research.RESEARCH / 'saudi-shura-allegiance-chairs-1990-2026-25.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-25.md'
CHAIR, BCHAIR = 'sa_shura_chair', 'sa_succession_chair'
JUB = 'Muhammad bin Ibrahim bin Jubair'
HUM = 'Saleh bin Abdullah bin Humaid'
ASH = 'Abdullah bin Mohammed bin Ibrahim Al Al-Sheikh'
MSH = 'Mishaal bin Abdulaziz Al Saud'
SHT = 'Bakri bin Salih Shatta'
NAMES = {JUB, HUM, ASH, MSH, SHT}
BASE_SOURCES = 32  # The packet's sources up to and including CLAUDE-C01-06.
ACCESSED = '2026-09-27'

NEW_SOURCES = [
    'sa_boe_shura_law_ar_20250507',
    'sa_boe_shura_law_en_20250513',
    'sa_shura_law_text_20010627',
    'sa_shura_history_20020208',
    'sa_shura_history_20090315',
    'sa_shura_intro_en_20010527',
    'sa_shura_statistics_20020208',
    'sa_shura_speeches_term1_20090415',
    'sa_shura_speeches_term2_20090415',
    'sa_embassy_shura_second_term_19970706',
    'sa_embassy_shura_oath_19970714',
    'sa_shura_news_20010614',
    'sa_embassy_shura_decrees_20010524',
    'sa_embassy_shura_oath_20010604',
    'sa_embassy_king_address_20010604',
    'sa_embassy_shura_session_20010605',
    'sa_shura_news_20020116',
    'sa_embassy_jubair_death_20020124',
    'sa_embassy_shura_mourning_20020127',
    'sa_shura_magazine_column_20020623',
    'sa_embassy_humaid_appointed_20020207',
    'sa_embassy_humaid_statement_20020207',
    'sa_shura_cv_humaid_20020609',
    'sa_shura_magazine_p8_20020824',
    'sa_shura_magazine_p28_20020425',
    'sa_spa_humaid_received_20020212',
    'sa_spa_shura_session_addendum_20020212',
    'sa_spa_english_news_20020212',
    'sa_shura_news_20020401',
    'sa_spa_orders_20090214',
    'sa_spa_order_a13_20090214',
    'sa_spa_order_a15_20090214',
    'sa_spa_dateline_20090228',
    'sa_spa_oath_20090301',
    'sa_shura_cv_alsheikh_20260808',
    'sa_spa_orders_20130111',
    'sa_spa_order_a45_20130111',
    'sa_spa_oath_20130219',
    'sa_spa_oath_addition_20130219',
    'sa_spa_orders_20161202',
    'sa_shura_news_a54_20161202',
    'sa_spa_oath_20161213',
    'sa_spa_orders_20201018_ar',
    'sa_spa_orders_20201018_en',
    'sa_spa_dateline_20201020',
    'sa_spa_oath_notice_20201108',
    'sa_spa_oath_20201111',
    'sa_spa_order_a84_20240902',
    'sa_uqn_order_a84_20240902',
    'sa_spa_dateline_20240906',
    'sa_spa_ninth_term_opening_20240918',
    'sa_spa_a135_statement_20061019',
    'sa_spa_a135_statement_resend_20061020',
    'sa_spa_allegiance_law_arts1_5_20061020',
    'sa_spa_allegiance_law_arts14_18_20061020',
    'sa_spa_order_a164_statute_20071007',
    'sa_spa_statute_arts2_5_20071007',
    'sa_spa_formation_reception_20071210',
    'sa_spa_order_a180_20071210',
    'sa_spa_order_a180_members_20071210',
    'sa_spa_commission_oath_20071210',
    'sa_spa_order_a180_en_20071210',
    'sa_spa_mishaal_statement_ar_20071211',
    'sa_spa_mishaal_statement_en_20071211',
    'sa_spa_mishaal_interview_addition_20080630',
    'sa_spa_mishaal_death_ar_20170503',
    'sa_spa_mishaal_death_en_20170503',
    'sa_spa_mishaal_interview_20080630',
    'sa_spa_mishaal_chair_obs_20151219',
    'sa_spa_shura_chair_obs_20260906',
    'sa_spa_shura_chair_obs_20260907',
]
# Every new claim, in packet order: (attested_on, event_kind, review observation, holder_name); None where no day is printed.
EVENTS = {
    'sa_shura_law_a91_dates_19920301': ('1992-03-01', 'promulgation', 'SA-CHR-01', None),
    'sa_shura_law_a91_order_terms': (None, 'procedure', 'SA-CHR-01', None),
    'sa_shura_law_art3_original_60': (None, 'procedure', 'SA-CHR-01', None),
    'sa_shura_law_art10_chair_by_royal_order': (None, 'procedure', 'SA-CHR-01', None),
    'sa_shura_law_art11_oath_before_work': (None, 'procedure', 'SA-CHR-01', None),
    'sa_shura_law_art13_term': (None, 'procedure', 'SA-CHR-01', None),
    'sa_shura_law_en_cover_19920302': ('1992-03-02', 'promulgation', 'SA-CHR-01', None),
    'sa_shura_site_law_a91_27_8_1412': (None, 'promulgation', 'SA-CHR-01', None),
    'sa_shura_site_law_art3_120_2001': (None, 'procedure', 'SA-CHR-01', None),
    'sa_shura_history_fahd_speech_27_8_1412': (None, 'retrospective_statement', 'SA-CHR-01', None),
    'sa_shura_history_internal_rules_3_3_1414': (None, 'retrospective_statement', 'SA-CHR-01', None),
    'sa_shura_first_session_inaugurated_19931229': ('1993-12-29', 'council_inauguration', 'SA-CHR-01', None),
    'sa_shura_membership_90_19970705': ('1997-07-05', 'membership_change', 'SA-CHR-01', None),
    'sa_shura_term_years_listed': (None, 'retrospective_statement', 'SA-CHR-01', None),
    'sa_jubair_speech_first_term_opening': (None, 'retrospective_statement', 'SA-CHR-02', JUB),
    'sa_jubair_speech_second_term_opening': (None, 'retrospective_statement', 'SA-CHR-02', JUB),
    'sa_jubair_dates_first_term_opening_16_7_1414': (None, 'council_inauguration', 'SA-CHR-02', None),
    'sa_shura_membership_90_a63_19970705': ('1997-07-05', 'membership_change', 'SA-CHR-03', None),
    'sa_jubair_first_term_end_19970706': ('1997-07-06', 'attestation', 'SA-CHR-03', JUB),
    'sa_jubair_extension_a90_19970706': ('1997-07-06', 'continuation', 'SA-CHR-03', JUB),
    'sa_jubair_extension_effective_19970707': ('1997-07-07', 'continuation', 'SA-CHR-03', JUB),
    'sa_order_a72_second_term_19970706': ('1997-07-06', 'council_formation_naming_chair', 'SA-CHR-03', JUB),
    'sa_shura_term_start_19970707': ('1997-07-07', 'council_term_start', 'SA-CHR-03', None),
    'sa_first_council_a16_cited_19930820': ('1993-08-20', 'council_formation', 'SA-CHR-03', None),
    'sa_jubair_oath_second_term_19970714': ('1997-07-14', 'oath', 'SA-CHR-03', JUB),
    'sa_second_term_inaugurated_19970714': ('1997-07-14', 'council_term_opening', 'SA-CHR-03', None),
    'sa_order_a80_third_term_20010524': ('2001-05-24', 'council_formation_naming_chair', 'SA-CHR-03', JUB),
    'sa_order_a72_a62_second_term_cited': ('2001-05-24', 'citation', 'SA-CHR-03', None),
    'sa_order_a78_art3_120_20010524': ('2001-05-24', 'membership_change', 'SA-CHR-03', None),
    'sa_order_a79_vice_chair_20010524': ('2001-05-24', 'deputy_appointment', 'SA-CHR-03', SHT),
    'sa_fahd_letter_chair_end_second_term': ('2001-05-24', 'attestation', 'SA-CHR-03', JUB),
    'sa_jubair_chairs_third_term_session_20010610': ('2001-06-10', 'attestation', 'SA-CHR-03', JUB),
    'sa_jubair_extension_congratulated_20010611': ('2001-06-11', 'continuation', 'SA-CHR-03', JUB),
    'sa_fahd_message_chair_20010524': ('2001-05-24', 'attestation', 'SA-CHR-03', JUB),
    'sa_jubair_oath_third_term_20010604': ('2001-06-04', 'oath', 'SA-CHR-03', JUB),
    'sa_third_term_start_announced_20010604': ('2001-06-04', 'council_term_opening', 'SA-CHR-03', None),
    'sa_jubair_chairs_first_session_20010605': ('2001-06-05', 'attestation', 'SA-CHR-03', JUB),
    'sa_jubair_chairs_48th_session_20020115': ('2002-01-15', 'attestation', 'SA-CHR-03', JUB),
    'sa_jubair_death_20020124': ('2002-01-24', 'death', 'SA-CHR-03', JUB),
    'sa_vice_chair_presides_20020127': ('2002-01-27', 'acting_presiding', 'SA-CHR-03', SHT),
    'sa_jubair_death_stated_sg_column': (None, 'retrospective_statement', 'SA-CHR-03', JUB),
    'sa_humaid_appointed_20020207': ('2002-02-07', 'appointment_reported', 'SA-CHR-03', HUM),
    'sa_humaid_statement_20020207': ('2002-02-07', 'attestation', 'SA-CHR-03', HUM),
    'sa_humaid_cv_chair_since_24_11_1422': ('2002-02-07', 'retrospective_statement', 'SA-CHR-03', HUM),
    'sa_humaid_first_day_20020211': ('2002-02-11', 'stated_assumption_of_office', 'SA-CHR-03', HUM),
    'sa_vice_chair_presides_55th_20020210': ('2002-02-10', 'acting_presiding', 'SA-CHR-03', SHT),
    'sa_humaid_order_issued_before_20020212': ('2002-02-12', 'appointment_statement', 'SA-CHR-03', HUM),
    'sa_humaid_oath_before_20020212': ('2002-02-12', 'oath_stated_undated', 'SA-CHR-03', HUM),
    'sa_humaid_chairs_56th_session_20020212': ('2002-02-12', 'attestation', 'SA-CHR-03', HUM),
    'sa_spa_humaid_chair_received_20020212': ('2002-02-12', 'attestation', 'SA-CHR-03', HUM),
    'sa_spa_humaid_addresses_council_20020212': ('2002-02-12', 'session_report', 'SA-CHR-03', HUM),
    'sa_spa_en_humaid_first_session_20020212': ('2002-02-12', 'attestation', 'SA-CHR-03', HUM),
    'sa_calendar_14221128_20020211': ('2002-02-11', 'calendar_equivalence', 'SA-CHR-03', None),
    'sa_humaid_chairs_65th_session_20020331': ('2002-03-31', 'attestation', 'SA-CHR-03', HUM),
    'sa_royal_orders_issued_20090214': ('2009-02-14', 'issuance_report', 'SA-CHR-04', None),
    'sa_alsheikh_appointed_a13_20090214': ('2009-02-14', 'appointment_by_royal_order', 'SA-CHR-04', ASH),
    'sa_alsheikh_effective_a13_20090228': ('2009-02-28', 'stated_effective_date', 'SA-CHR-04', ASH),
    'sa_shura_formed_a15_20090214': ('2009-02-14', 'council_formation_naming_chair', 'SA-CHR-04', ASH),
    'sa_calendar_14300303_20090228': ('2009-02-28', 'calendar_equivalence', 'SA-CHR-04', None),
    'sa_alsheikh_oath_20090301': ('2009-03-01', 'oath', 'SA-CHR-04', ASH),
    'sa_humaid_styled_sjc_chair_20090301': ('2009-03-01', 'other_office_attestation', 'SA-CHR-04', HUM),
    'sa_shura_cv_a13_retrospective_20260808': ('2026-08-08', 'retrospective_statement', 'SA-CHR-04', ASH),
    'sa_royal_orders_issued_20130111': ('2013-01-11', 'issuance_report', 'SA-CHR-05', None),
    'sa_alsheikh_named_a45_20130111': ('2013-01-11', 'council_formation_naming_chair', 'SA-CHR-05', ASH),
    'sa_alsheikh_oath_20130219': ('2013-02-19', 'oath', 'SA-CHR-05', ASH),
    'sa_sixth_term_opened_20130219': ('2013-02-19', 'council_term_opening', 'SA-CHR-05', None),
    'sa_royal_orders_issued_20161202': ('2016-12-02', 'issuance_report', 'SA-CHR-05', None),
    'sa_alsheikh_named_a54_20161202': ('2016-12-02', 'council_formation_naming_chair', 'SA-CHR-05', ASH),
    'sa_shura_term_start_20161202': ('2016-12-02', 'council_term_start', 'SA-CHR-05', None),
    'sa_shura_site_a54_20161202': ('2016-12-02', 'republication', 'SA-CHR-05', ASH),
    'sa_alsheikh_oath_20161213': ('2016-12-13', 'oath', 'SA-CHR-05', ASH),
    'sa_royal_orders_issued_20201018': ('2020-10-18', 'issuance_report', 'SA-CHR-05', None),
    'sa_alsheikh_named_a146_20201018': ('2020-10-18', 'council_formation_naming_chair', 'SA-CHR-05', ASH),
    'sa_shura_term_start_20201020': ('2020-10-20', 'council_term_start', 'SA-CHR-05', None),
    'sa_alsheikh_speaker_en_20201018': ('2020-10-18', 'translation', 'SA-CHR-05', ASH),
    'sa_calendar_14420303_20201020': ('2020-10-20', 'calendar_equivalence', 'SA-CHR-05', None),
    'sa_shura_oath_scheduled_20201108': ('2020-11-08', 'oath_scheduled', 'SA-CHR-05', ASH),
    'sa_alsheikh_oath_20201111': ('2020-11-11', 'oath', 'SA-CHR-05', ASH),
    'sa_alsheikh_named_a84_20240902': ('2024-09-02', 'council_formation_naming_chair', 'SA-CHR-06', ASH),
    'sa_shura_term_start_20240906': ('2024-09-06', 'council_term_start', 'SA-CHR-06', None),
    'sa_uqn_a84_20240902': ('2024-09-02', 'republication', 'SA-CHR-06', ASH),
    'sa_calendar_14460303_20240906': ('2024-09-06', 'calendar_equivalence', 'SA-CHR-06', None),
    'sa_ninth_term_opened_20240918': ('2024-09-18', 'council_term_opening', 'SA-CHR-06', None),
    'sa_alsheikh_oath_20240918': ('2024-09-18', 'oath', 'SA-CHR-06', ASH),
    'sa_allegiance_law_a135_statement_20061019': ('2006-10-19', 'royal_court_statement', 'SA-CHR-07', None),
    'sa_allegiance_law_a135_basic_law_5c': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_law_a135_future_cases': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_law_a135_statement_yesterday_20061020': ('2006-10-20', 'publication', 'SA-CHR-07', None),
    'sa_allegiance_law_art1_constitution': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_law_art5_oath': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_law_art15_chair_seniority': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_law_art17_chair_convenes': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_statute_a164_published_20071007': ('2007-10-07', 'publication', 'SA-CHR-07', None),
    'sa_allegiance_statute_art3_term': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_statute_art4_chair_committee': (None, 'procedure', 'SA-CHR-07', None),
    'sa_allegiance_formation_reception_20071209': ('2007-12-09', 'meeting', 'SA-CHR-08', None),
    'sa_allegiance_formation_meeting_20071209': ('2007-12-09', 'meeting', 'SA-CHR-08', None),
    'sa_mishaal_designated_a180_20071209': ('2007-12-09', 'commission_formation_naming_chair', 'SA-CHR-08', MSH),
    'sa_allegiance_a180_members_10_35_20071209': ('2007-12-09', 'membership_appointment', 'SA-CHR-08', None),
    'sa_allegiance_oath_20071209': ('2007-12-09', 'collective_oath', 'SA-CHR-08', None),
    'sa_a180_english_no_chair_title_20071210': ('2007-12-10', 'translation_note', 'SA-CHR-08', None),
    'sa_mishaal_chair_thanks_20071211': ('2007-12-11', 'attestation', 'SA-CHR-08', MSH),
    'sa_mishaal_chair_en_20071211': ('2007-12-11', 'translation', 'SA-CHR-08', MSH),
    'sa_mishaal_on_appointment_art15_20080630': ('2008-06-30', 'retrospective_statement', 'SA-CHR-08', MSH),
    'sa_mishaal_death_20170503': ('2017-05-03', 'death', 'SA-CHR-09', MSH),
    'sa_mishaal_death_en_20170503': ('2017-05-03', 'translation', 'SA-CHR-09', MSH),
    'sa_mishaal_chair_interview_20080630': ('2008-06-30', 'attestation', 'SA-CHR-10', MSH),
    'sa_mishaal_chair_obs_20151219': ('2015-12-19', 'attestation', 'SA-CHR-10', MSH),
    'sa_shura_chair_obs_20260906': ('2026-09-06', 'attestation', 'SA-CHR-10', ASH),
    'sa_shura_chair_obs_20260907': ('2026-09-07', 'attestation', 'SA-CHR-10', ASH),
}
PERIODS = {
    'sa_jubair_speech_first_term_opening': ('1993-06-21', '1995-05-30'),
    'sa_jubair_speech_second_term_opening': ('1997-05-08', '1999-04-16'),
    'sa_jubair_dates_first_term_opening_16_7_1414': ('1993-12-29', '1993-12-30'),
}
# Recorded original-response identity (uncompressed body): bytes and SHA-256.
RESPONSES = {
    'sa_boe_shura_law_ar_20250507': (74490, 'c16816025db847abcc14c4a7ebbd2bb0b8a5ea4e554d90d52730e4fc9c072d7c'),
    'sa_boe_shura_law_en_20250513': (261632, 'a58d7c0036b7f5174f7a165b3c76a816774ea325672d101736a0e3f9850770cf'),
    'sa_shura_law_text_20010627': (48227, 'f8cb0c1cc19f56f82fa9ff54ecd217b2fed7c3078a2be8bdd75f47448ce2b5e4'),
    'sa_shura_history_20020208': (12434, 'f4c71b31655d56f7ebc049aeaeaaab3d4c04a134042f45c085ced70a30325bf9'),
    'sa_shura_history_20090315': (22558, '6b3c9db79ba086c44515cf0713681f7fe76d6c4e6006fe1e9a72cbb6e164a3af'),
    'sa_shura_intro_en_20010527': (4486, '1cb57fe9f206bd1a135fb6b85bc5f1481ceb0bb93d2003476683b989698923e0'),
    'sa_shura_statistics_20020208': (10951, 'e4bbef934328a0e7617919f0521da385ad02138a7f76ae033a15a6a364c65605'),
    'sa_shura_speeches_term1_20090415': (33817, '9ef48ff223c2d307d9311597c6075647339b78a6df658bb492c643e237f3a811'),
    'sa_shura_speeches_term2_20090415': (32811, 'c00d4dd0c0d838a329bd3d1f1a7b67aef895541f36c8b4072356b268e6c8e189'),
    'sa_embassy_shura_second_term_19970706': (24777, '0f41939977da960ee62121d097dd7f3924aecaddd1ce16059134e02eac93c271'),
    'sa_embassy_shura_oath_19970714': (7326, 'c0c99c9f685ac6515e23a2c270b62ee4f8dd414605f50de98ef5992d2a1e3645'),
    'sa_shura_news_20010614': (71970, '34afe4935e5b7d3ea5783013f0f1fd390f12af3308c973750c520d965e9b85fe'),
    'sa_embassy_shura_decrees_20010524': (2175, '6841957c98f350fc8da28f81760e1d951d3fb4fd729294dc230651b864b805b4'),
    'sa_embassy_shura_oath_20010604': (2229, '3e82c74d9952198fed9ea7b158dfd01b60655a8c45e2347232fe71d00b27f25d'),
    'sa_embassy_king_address_20010604': (24427, 'a0a4e5f80dc3c6889f46b783c2f64fe9a0a70737f2d245fca6935f427ca628c7'),
    'sa_embassy_shura_session_20010605': (3721, 'd84467cba193de28677bdcaf995ab2f1a90cf6e26a0d933de613b650dd70f94c'),
    'sa_shura_news_20020116': (20106, '7813fabc44f0c66b9634de5ebe011114fe9b70e91560156c1fc141374e06eac5'),
    'sa_embassy_jubair_death_20020124': (1372, 'e67f6859396e77e968dfaa2545ebb1880c5e998056501416c482ebcf4474ea7c'),
    'sa_embassy_shura_mourning_20020127': (1530, '465095988616e72fc243bb3b4993cbecb00428c8c57a7867ebdb58ab53388916'),
    'sa_shura_magazine_column_20020623': (11715, '5e2908592639ec6a0d18796aa11fcb4e417d8110bd610defe25a76ba71a6e968'),
    'sa_embassy_humaid_appointed_20020207': (1041, 'c302aa57aece728a7679c94e69561aea2c687c1e7765516c04dcf1d9edd61d55'),
    'sa_embassy_humaid_statement_20020207': (1930, '86a11955bb33d7e5f7e8eb7eb14aef34486a5872df972d0cde79ef709e6d813f'),
    'sa_shura_cv_humaid_20020609': (9686, 'a24b110d9b4e833bf75b5b2e73d0c0bc301506ed461967cddb99af31f9a5c72a'),
    'sa_shura_magazine_p8_20020824': (15258, '51c93469aa2ec1c768433e4e3bcf289c4a15a5b26c7b46ecb2fc1117c57e1eaa'),
    'sa_shura_magazine_p28_20020425': (64110, '38ac167f648a0c0c05cad23798f1cadbdc13f82d20feba26900868650dca3123'),
    'sa_spa_humaid_received_20020212': (2723, '1b7e40468e2b282d02f7855d495e22390a7cffbc509b127a6e260557159bd147'),
    'sa_spa_shura_session_addendum_20020212': (4044, 'c93ce54262e0c30ff5a544af3e6004a4bb102ae858d405612b9e13c8cc4292bf'),
    'sa_spa_english_news_20020212': (11589, '68d9f877ee59e4f0bf6983bb7cbc4aba55460760b3c3faaeade44a9acd0682f0'),
    'sa_shura_news_20020401': (18782, '9484876fb1de3805cb3313a3428050aff6a71f853f683e82e030b0ebd7055675'),
    'sa_spa_orders_20090214': (1108, '12fc65c03d307d477bfd82ab7cdb3c368aae44fceb07b73a36b5b3f72d43c733'),
    'sa_spa_order_a13_20090214': (2752, '7757999da3e8daff0506ae55ca786c78884e421b82f20c2fe1dac868d01fb8f2'),
    'sa_spa_order_a15_20090214': (3567, 'e3eee58814b7695b2e9e9dcb9096d774889d44b273b4bf9bdc6b71d98db6e798'),
    'sa_spa_dateline_20090228': (2278, '1da545ea383bd2a6fe6e1e3e23928b9a4182a9ff1798f775381e4e6c345a990d'),
    'sa_spa_oath_20090301': (3744, 'bcc605cc1306fd026f5affc34f520858449d18bf51693a332defdc430a8f5d05'),
    'sa_shura_cv_alsheikh_20260808': (60974, 'b44a27c228fd17253825edc5955f76b80da1655143881b3b1018bc0a8c84ef57'),
    'sa_spa_orders_20130111': (3783, '42e627706cb0232eb763d4957bdd5d09a96bf18e39f6edd7edd2c3f92e053b5e'),
    'sa_spa_order_a45_20130111': (3578, '749692cd10b0f47779cf303bdd05a3417ec9a66cf36136fe2624fc53d7baa023'),
    'sa_spa_oath_20130219': (1446, 'cef915cf03953c661dcca26b60dddc8126e240a805926a24f6d477520b6e2e30'),
    'sa_spa_oath_addition_20130219': (3612, '49813626e83895cd47ab63710ce99dcc85ad4f8c4799688905eb60815e6e8687'),
    'sa_spa_orders_20161202': (30893, '359e15d7b4117f0c437dacc547f69dbada4053dd8f48b09c10e674c04137bebb'),
    'sa_shura_news_a54_20161202': (56416, 'bd339eae23a0c531cd5d14beaa8a9f9244a27565c0542f6187916e59979b6a7f'),
    'sa_spa_oath_20161213': (5809, 'a5204e5b50233ff6c628cdf90f1dd0323bdda7229fb3e4f91781c3584898e904'),
    'sa_spa_orders_20201018_ar': (26311, '9b4e91c44ded76d817fd0ddca89b141cf838dd2a2eab492650276f929c75f530'),
    'sa_spa_orders_20201018_en': (13318, '3838ce24a453d4f1aeb260ef41dfb725920547d5ab563b103b6926c8bd126351'),
    'sa_spa_dateline_20201020': (3244, 'dc2c662f9ebe442065a2503dfb8405990aae97f05e49bf2720fdc509ee0a2ed3'),
    'sa_spa_oath_notice_20201108': (7300, '7b1acc42fce14540fb2edbdcaf0da622f03b626242cbc8561c763d35cf5ec9fb'),
    'sa_spa_oath_20201111': (13387, '9637b22ef8a0423ab07010e434df8c0d2ba9c9c98bb17f3f44270ead64979186'),
    'sa_spa_order_a84_20240902': (16335, 'c4bb09c0f7b57987c9fd0f86afe79e7c640c6c19ee1684b2d0ffc00c5a576c5e'),
    'sa_uqn_order_a84_20240902': (52606, 'be596fc3c805e4d171f6120a9a4315194f37d6df6a6f38928b22e7b36f30291f'),
    'sa_spa_dateline_20240906': (1958, 'fc5780cc8826c4b87e08a9f6f966d3ff6205cf0143534d6c052d0e10e100e37d'),
    'sa_spa_ninth_term_opening_20240918': (31715, 'aaaa65af49e60069ccef0f8140105d6b8d4e998dec20b744badd74cf20d18120'),
    'sa_spa_a135_statement_20061019': (2636, 'f1e7dcbd2d4d2acb3a1aaf3055a950e78c610b9b0bf9188175283a221c6c6987'),
    'sa_spa_a135_statement_resend_20061020': (2680, 'ec092537bd01795e9a0000fbc59c85adead469138a8de3f3123ee89b0c1ba9fa'),
    'sa_spa_allegiance_law_arts1_5_20061020': (3882, '5e65e69fb2b8cd68157f260ab7c4f5a2769d84a08b7075f745a2c43079fbb60c'),
    'sa_spa_allegiance_law_arts14_18_20061020': (3349, '1f499fbf81be1d817c8c4b9082efb0a513a6cd2d9344bbe3c684704ab0dc01d5'),
    'sa_spa_order_a164_statute_20071007': (2681, '242ba749e78bbfc7ce26ff0b4b63c8d5b47fe7821fb408533385063a83dd2621'),
    'sa_spa_statute_arts2_5_20071007': (2777, 'd470638aac267e874f9d963f29852841c6234ce9f40a9ca6c76c9b2707ebf387'),
    'sa_spa_formation_reception_20071210': (5273, '9e633a9d0800b727de5829244bc43dfbcda6eed5a046bbc74bf65fc4e836aa5a'),
    'sa_spa_order_a180_20071210': (3753, '1c90e6b3bc107e706222bbf459ccdc18b1e0b3cff18328b8815cc231682db9b6'),
    'sa_spa_order_a180_members_20071210': (4312, '038405c42f1f075712d8ecf60c10825024241c98184ef29d4d10034e6e6aed57'),
    'sa_spa_commission_oath_20071210': (1743, 'a043bbda1af5890eb73955987569cf76e66d7317cbde87b120a83462943e60ab'),
    'sa_spa_order_a180_en_20071210': (1879, '83887899cf905306a1259d87b5618efa63b48eaa05d7e73788482275554f4539'),
    'sa_spa_mishaal_statement_ar_20071211': (5361, '7f42efd40e370b59a0c6581ffab4fafc8805ed9473f8a549d90aba539a30f1f8'),
    'sa_spa_mishaal_statement_en_20071211': (1783, '98f18a8dc647a774e274bc19b922b7f57ca036d3bc13593dcff3ce0fe1a66338'),
    'sa_spa_mishaal_interview_addition_20080630': (3779, 'd84103dd96c2fd17b0bf22fb93d7a99a3311eee6514a86f9347b7215eaa7a1dc'),
    'sa_spa_mishaal_death_ar_20170503': (1952, '8ac17b30878537fa9dc5ee9b2e61fa5b6ede184b84f0dab4dd5d6db00903a929'),
    'sa_spa_mishaal_death_en_20170503': (1359, '6e032b8b4480a96e6f0356f8deec1014a71fc72ee1cbd7de5a3f05291d440da3'),
    'sa_spa_mishaal_interview_20080630': (3557, 'b37e3a28f86558e1fd5abfb27cf69374684e9cfd32383a8aa87fb18943d3a61e'),
    'sa_spa_mishaal_chair_obs_20151219': (2800, 'e0a9e2136f0aa55d2ac8666e76c88496271c05b510158792cf98e83715da3a32'),
    'sa_spa_shura_chair_obs_20260906': (2139, 'cb446c354ac61f68225a5780406dc0d5bdb0c6b5dd34a82f40e9569d75b68352'),
    'sa_spa_shura_chair_obs_20260907': (2204, '9bccefa64e9aa651c080111fb8032dcf2088160a178a48fa2009b0fea82152d4'),
}
TRANSPORT = {
    'sa_shura_cv_alsheikh_20260808': (15780, '4b9b72d3545d58e9a53ed2a3c4ddc6534585e2227f24b88de3a2eb141872edd2'),
    'sa_shura_news_a54_20161202': (18610, '45fdb04366f09e9e7c7bf8c7355204c7ad22233268099477314d53358ca43483'),
}
ARTICLE_CONTENT = {
    'sa_spa_orders_20090214': '483c0d6e254029e35d615c179ba973510e914636c0dc9158437c12dae498f17c',
    'sa_spa_order_a13_20090214': '6a39d55074971894b2fb2c1e2837352ef86cb6a99c3fe76263a4d307cc846439',
    'sa_spa_order_a15_20090214': '82452aaa69df83569a79b37a73f01861499fc181962c700c1a765a9aed04e701',
    'sa_spa_dateline_20090228': 'eacc82fa3f272b43cf724d834930be92c64f243baf4bf2d3a148577ada41c218',
    'sa_spa_oath_20090301': 'fab9c54a239d72605a54dc6529a9fdcc666484000fafb0c8c9842942737f34c9',
    'sa_spa_orders_20130111': '4cd304a69a12d68ed319a54aaed4e933731ecb50586755774ddd4e0e55422e7b',
    'sa_spa_order_a45_20130111': '64c92bbc680a18439fe00bc913b63d09abfc8d4bc04e04d56cb61d33bf772f04',
    'sa_spa_oath_20130219': '90c4cf41de2c19efdfac82f6f820a7b14e2cd80e6afcb63ac3b589110693b2a5',
    'sa_spa_oath_addition_20130219': 'e1e58de2e3a673dbd0d3b2299d4c73c8188c59744f19ebbdeb3e66099e9be9bf',
    'sa_spa_orders_20161202': '4991f62740b7571d89d2e675054d98607c8fdccbffa035404f2dcfb60de4f95c',
    'sa_spa_oath_20161213': 'ddb1139026b3356dca24902c170011df6af7403d7c3077f7be67b50e1724bc7f',
    'sa_spa_orders_20201018_ar': 'f6b5b32f3eb964d6cec1121d996b3cfb5e86034a09ab7bb7c7e0bf32ca6f116a',
    'sa_spa_orders_20201018_en': '051ca953560eb9dbf70a8cfe1249f88a8068ddef0e33e9a76b3fa0df8dd8c0f1',
    'sa_spa_dateline_20201020': '776f5ded4865fc1361706cb5279572e31ed00ef325ca897643bb4a7322f8a2de',
    'sa_spa_oath_notice_20201108': '8208719cd803157832f2f89e96121adaeda912d41fbe33915d2dd5d3eb90c10c',
    'sa_spa_oath_20201111': 'd20f23cb967d4b1b4b16867a45f2cb4495c210dacfa6c748cee3ba2857616967',
    'sa_spa_order_a84_20240902': '51a2344a04f09d882762f0bdea5f565cd95efefd760e0d2ab713557f0f486619',
    'sa_spa_dateline_20240906': '25a49397f84a960a60d9714574e25744c033a714f6f082045defa68e47cf601e',
    'sa_spa_ninth_term_opening_20240918': 'cbefc5db655fac4e5694c82b2dba9d59b7eb45809aa05c610c5316c041dcd321',
    'sa_spa_a135_statement_20061019': 'fc70a1e9c1e5d6b2af777315648684eaf4ebb8ec1210671a6e10db335154f409',
    'sa_spa_a135_statement_resend_20061020': '65c8ea94f49ee8d61b7ef56f08d39cbe7d830ca383d34b89179b205659fa51cc',
    'sa_spa_allegiance_law_arts1_5_20061020': '49e0d08916b23bd2c8e2690c73f6ac46a5a1f0ff0cbaeaf7556fdfcc9ede729e',
    'sa_spa_allegiance_law_arts14_18_20061020': '64bfd0c3e6e3b01c09e1261ed243067a5ebeb4a41f43a99d2cacb801fa0afd0e',
    'sa_spa_order_a164_statute_20071007': '92e3fac8f9d99d995d9cfadfb8b2961fb2bfc7de2935f483f1fe33529a649635',
    'sa_spa_statute_arts2_5_20071007': 'f9df17606c1b53c0015b97972f8fa0b7a48fd942d2b93076792d874bd23efbbc',
    'sa_spa_formation_reception_20071210': 'fc7e4dddccfba0f4ac2a5c437265ddb0b540b56bb9a6465bf4cd298d3729be0a',
    'sa_spa_order_a180_20071210': 'bc4e0a2df41f14ceef5e1cc4f5442b7298e2560adc008e0eaeb5e7616a46561a',
    'sa_spa_order_a180_members_20071210': '62dc66cddc3295a104eebc68fcebf0832296d48056d108bed6b588de605ce684',
    'sa_spa_commission_oath_20071210': '1fdef39787f797c3acc5b4c128cce52bea2c18e323e5484d63fdf5214aae8e55',
    'sa_spa_order_a180_en_20071210': '5fd63854fa0d3f7385f10d823c36d6652b938a6f8beae9486124aa738ae4e32a',
    'sa_spa_mishaal_statement_ar_20071211': '24b5f81b538deca1a092481d2c311647b4431383724cac81db797bf671adc62e',
    'sa_spa_mishaal_statement_en_20071211': 'a537d28f9b59c1c2c75559797fa0be123d5813cf809dc473198c2f049dc02541',
    'sa_spa_mishaal_interview_addition_20080630': 'a234ffa303cb80b848d617b148c86c2f71af8b9d5fe6b4a5e4835b4e0cf7be24',
    'sa_spa_mishaal_death_ar_20170503': 'c94b0000eccc8472a799a300526f409309df417440a461a598f2460020e818e7',
    'sa_spa_mishaal_death_en_20170503': '6fd75d5edf394ca9737ca9d281d5f2b8d1bec13294ebf2f5475758656f779e96',
    'sa_spa_mishaal_interview_20080630': 'a11da8a6e471cc6fca35c62384c1f7e133b66e50732e716778646adfc1a4a1a7',
    'sa_spa_mishaal_chair_obs_20151219': '3717d1f324a77e93544f6c5286286ddfaa223b747c551aa290b0bcc3f7e1320c',
    'sa_spa_shura_chair_obs_20260906': '74553863fb19a2bf05c02b01100a3dd267d75152156f46a6e7fd8d44e19d8050',
    'sa_spa_shura_chair_obs_20260907': '4560cb01b1ba95ee6e4cee143851a751e3521e673fec9731fa7653e3661acc11',
}

# Holder tenures: (name, attested_on, from, until, claim_ids). One per person-tenure, in chronological order.
HOLDERS = {
    CHAIR: [
        (JUB, '1997-07-06', None, '2002-01-24', ['sa_jubair_first_term_end_19970706', 'sa_jubair_death_20020124']),
        (HUM, '2002-02-07', '2002-02-11', None, ['sa_humaid_appointed_20020207', 'sa_humaid_first_day_20020211']),
        (ASH, '2009-02-14', '2009-02-28', None, ['sa_alsheikh_appointed_a13_20090214', 'sa_alsheikh_effective_a13_20090228']),
    ],
    BCHAIR: [
        (MSH, '2007-12-09', None, '2017-05-03', ['sa_mishaal_designated_a180_20071209', 'sa_mishaal_death_20170503']),
    ],
}
# The full holder_claims order: each tenure followed by its dated holder observations (claim IDs).
HOLDER_ORDER = {
    CHAIR: [JUB, 'sa_order_a72_second_term_19970706', 'sa_jubair_oath_second_term_19970714', 'sa_order_a80_third_term_20010524',
            'sa_fahd_letter_chair_end_second_term', 'sa_fahd_message_chair_20010524', 'sa_jubair_oath_third_term_20010604',
            'sa_jubair_chairs_first_session_20010605', 'sa_jubair_chairs_third_term_session_20010610',
            'sa_jubair_chairs_48th_session_20020115',
            HUM, 'sa_humaid_statement_20020207', 'sa_humaid_chairs_56th_session_20020212', 'sa_spa_humaid_chair_received_20020212',
            'sa_spa_en_humaid_first_session_20020212', 'sa_humaid_chairs_65th_session_20020331',
            ASH, 'sa_shura_formed_a15_20090214', 'sa_alsheikh_oath_20090301', 'sa_alsheikh_named_a45_20130111',
            'sa_alsheikh_oath_20130219', 'sa_alsheikh_named_a54_20161202', 'sa_alsheikh_oath_20161213',
            'sa_alsheikh_named_a146_20201018', 'sa_alsheikh_oath_20201111', 'sa_alsheikh_named_a84_20240902',
            'sa_alsheikh_oath_20240918', 'sa_shura_chair_obs_20260906', 'sa_shura_chair_obs_20260907'],
    BCHAIR: [MSH, 'sa_mishaal_chair_thanks_20071211', 'sa_mishaal_chair_interview_20080630', 'sa_mishaal_chair_obs_20151219'],
}
# Kinds that may anchor a tenure, date its from, end it, or stand as a dated holder observation.
ANCHOR_KINDS = {'attestation', 'appointment_reported', 'appointment_by_royal_order', 'commission_formation_naming_chair'}
START_KINDS = {'stated_effective_date', 'stated_assumption_of_office'}
END_KINDS = {'death'}
OBSERVATION_KINDS = {'attestation', 'oath', 'council_formation_naming_chair'}
HOLDER_KINDS = ANCHOR_KINDS | START_KINDS | END_KINDS | OBSERVATION_KINDS
# Everything else is a claim only: never a holder, a holder observation or a boundary.
NEVER_HOLDER_KINDS = {
    'procedure', 'promulgation', 'membership_change', 'retrospective_statement', 'council_inauguration', 'council_formation',
    'council_term_opening', 'council_term_start', 'citation', 'deputy_appointment', 'continuation', 'acting_presiding',
    'appointment_statement', 'oath_stated_undated', 'session_report', 'calendar_equivalence', 'issuance_report',
    'other_office_attestation', 'republication', 'translation', 'translation_note', 'oath_scheduled', 'royal_court_statement',
    'publication', 'meeting', 'membership_appointment', 'collective_oath',
}
# Dates that no holder may take as from or until: orders without an effective day, oaths, term starts and openings, acting
# presiding, last attestations, other offices, laws and calendar anchors.
NEVER_BOUNDARY = {
    '1992-03-01', '1992-03-02', '1993-08-20', '1993-12-29', '1993-12-30', '1997-07-05', '1997-07-06', '1997-07-07',
    '1997-07-14', '2001-05-24', '2001-06-04', '2001-06-05', '2001-06-10', '2001-06-11', '2002-01-15', '2002-01-27',
    '2002-02-07', '2002-02-10', '2002-02-12', '2002-03-31', '2009-02-14', '2009-03-01', '2013-01-11', '2013-02-19',
    '2016-12-02', '2016-12-13', '2020-10-18', '2020-10-20', '2020-11-08', '2020-11-11', '2024-09-02', '2024-09-06',
    '2024-09-18', '2026-08-08', '2026-09-06', '2026-09-07', '2006-10-19', '2006-10-20', '2007-10-07', '2007-12-09',
    '2007-12-10', '2007-12-11', '2008-06-30', '2015-12-19', '2017-06-21',
}
STATED_BOUNDARIES = {('from', '2002-02-11'), ('from', '2009-02-28'), ('until', '2002-01-24'), ('until', '2017-05-03')}
ROLES = {'sa_king', 'sa_crown_prince', 'sa_pm', 'sa_cabinet_ministers', 'sa_shura_chair', 'sa_shura_members',
         'sa_succession_chair', 'sa_succession_secretary', 'sa_succession_members', 'sa_municipal_members'}
INSTITUTIONS = ['sa_crown', 'sa_prime_minister', 'sa_council_ministers', 'sa_shura', 'sa_succession_commission',
                'sa_municipal_councils']
# CLAUDE-C01-06 holders, unchanged: (name or claim ID, attested_on, from, until).
C01_06_HOLDERS = {
    'sa_king': ['sa_salman_king_observation', ('Fahd bin Abdulaziz Al Saud', '1990-08-08', None, None),
                ('Abdullah bin Abdulaziz Al Saud', '2005-08-01', None, '2015-01-23'),
                ('Salman bin Abdulaziz Al Saud', '2015-01-23', None, None), 'sa_salman_king_obs_20260813'],
    'sa_crown_prince': ['sa_mbs_pm_appointment', ('Abdullah bin Abdulaziz Al Saud', '2005-08-01', None, None),
                        ('Sultan bin Abdulaziz Al Saud', '2005-08-01', None, '2011-10-22'),
                        ('Nayef bin Abdulaziz Al Saud', '2011-10-27', None, '2012-06-16'),
                        ('Salman bin Abdulaziz Al Saud', '2012-06-18', None, '2015-01-23'),
                        ('Muqrin bin Abdulaziz Al Saud', '2015-01-23', None, '2015-04-29'),
                        ('Mohammed bin Nayef bin Abdulaziz Al Saud', '2015-04-29', None, '2017-06-21'),
                        ('Mohammed bin Salman bin Abdulaziz Al Saud', '2017-06-21', None, None), 'sa_mbs_cp_pm_obs_20260813'],
    'sa_pm': ['sa_mbs_pm_appointment', 'sa_mbs_cp_pm_obs_20260813'],
}
# CLAUDE-C01-45 appends dated observations after the C01-06 entries (pinned in test_saudi_kings_crown_princes_c01_45.py).
C01_06_HOLDERS['sa_king'] += [
    'sa_fahd_king_obs_19960101', 'sa_fahd_chairs_cabinet_19960212', 'sa_fahd_king_order_a193_20050731',
    'sa_abdullah_king_order_a136_20061020', 'sa_abdullah_king_order_a145_20140520', 'sa_salman_king_order_a267_20150725',
    'sa_salman_king_obs_20260904']
C01_06_HOLDERS['sa_crown_prince'] += [
    'sa_abdullah_cp_obs_19960101', 'sa_abdullah_cp_obs_20050730', 'sa_sultan_cp_obs_a175_20071029', 'sa_nayef_cp_obs_20111107',
    'sa_salman_cp_obs_20120618', 'sa_salman_cp_obs_a145_20140520', 'sa_muqrin_cp_obs_20150428', 'sa_mbn_cp_obs_a267_20150725',
    'sa_mbn_cp_obs_a128_20170225', 'sa_mbs_cp_obs_20260901']
# CLAUDE-C01-50 appends four sa_pm holders, each followed by its dated observations (pinned in
# test_saudi_prime_ministers_c01_50.py).
C01_06_HOLDERS['sa_pm'] += [
    ('Fahd bin Abdulaziz Al Saud', '1996-03-04', None, None), 'sa_fahd_pm_styled_20041003', 'sa_fahd_pm_chairs_cabinet_20050425',
    ('Abdullah bin Abdulaziz Al Saud', '2005-08-01', None, None), 'sa_abdullah_pm_order_a29_20070322',
    'sa_abdullah_pm_chairs_cabinet_20121229',
    ('Salman bin Abdulaziz Al Saud', '2015-01-23', None, None), 'sa_salman_pm_order_a68_20150129',
    'sa_salman_pm_order_a138_20181227', 'sa_salman_pm_chairs_cabinet_20220517',
    ('Mohammed bin Salman bin Abdulaziz Al Saud', '2022-09-27', None, None), 'sa_mbs_pm_order_a62_20220927',
    'sa_mbs_pm_chairs_cabinet_20221025', 'sa_mbs_pm_chairs_cabinet_20260616',
]
# CLAUDE-C01-45 fills sa_succession_secretary: (sources, claim_ids, holders as (name, attested_on, from, until)), exactly.
C01_45_SECRETARY = (['sa_boe_succession_law', 'sa_spa_order_a136_20061020', 'sa_spa_allegiance_law_art24_20061020'],
                    ['sa_succession_membership', 'sa_tuwaijri_sg_appointed_a136_20061020', 'sa_allegiance_law_art24_secretary'],
                    [('Khalid bin Abdulaziz Al-Tuwaijri', '2006-10-20', None, None)])
C01_45_SOURCES = 18  # Appended after this packet's sources.
C01_50_SOURCES = 15  # Appended after the CLAUDE-C01-45 sources.
LATER_UNRESOLVED = {'sa_shura': 0, 'sa_succession_commission': 1}  # CLAUDE-C01-45 items appended after this packet's.
# Roles and entries this packet does not touch: (sources, claim_ids); their holder_claims stay as they were.
UNTOUCHED = {
    'sa_shura_members': (['sa_boe_shura_law', 'sa_cedaw_state_report_2016'], ['sa_shura_composition', 'sa_shura_member_role', 'sa_shura_women_2013']),
    'sa_succession_members': (['sa_boe_succession_law'], ['sa_succession_membership']),
    'sa_municipal_members': (['sa_cedaw_state_report_2016'], ['sa_municipal_elections_2015']),
    'sa_cabinet_ministers': (['sa_basic_law_boe', 'sa_spa_pm_2022', 'sa_spa_orders_20260813'],
                             ['sa_basic_pm_rule', 'sa_king_cabinet_chair_exception', 'sa_mbs_cp_pm_obs_20260813']),
}
# Existing source and claim IDs that stay first, in order, on the two filled roles and their institutions (IDs only).
BASE_PREFIX = {
    CHAIR: (['sa_boe_shura_law'], ['sa_shura_composition']),
    BCHAIR: (['sa_boe_succession_law'], ['sa_succession_membership']),
    'sa_shura': (['sa_boe_shura_law', 'sa_cedaw_state_report_2016'], ['sa_shura_composition', 'sa_shura_member_role', 'sa_shura_women_2013']),
    'sa_succession_commission': (['sa_boe_succession_law', 'sa_spa_nayef_cp_20111027', 'sa_spa_order_a160_20150429', 'sa_spa_order_a255_20170621_ar'],
                                 ['sa_succession_membership', 'sa_succession_function', 'sa_allegiance_law_a135_cited', 'sa_nayef_cp_order_a224',
                                  'sa_mbs_deputy_cp_a160', 'sa_allegiance_31_of_34_20170621']),
}
BASE_UNRESOLVED = {'sa_shura': 3, 'sa_succession_commission': 4, 'packet': 7}
IA_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(https?://\S+)$')
SPA_API_URL = re.compile(r'^https://portalapi\.spa\.gov\.sa/api/v1/news/([0-9a-f]{10}|N\d{7})$')
# Per-request, search, listing or growing-response patterns that must never be a recorded response.
VOLATILE = re.compile(r'(?i)(cdx|wayback/available|/search|api/widget|[?&](q|query|page|pgno|start|rows|cb|chk)=|views_count|'
                      r'viewfullstory|viewstory|sp\.spa\.gov\.sa|/timemap/)')
# Leads and duplicates that must not be recorded as sources.
LEAD_URL_MARKERS = (
    'wikipedia.org', 'saudipedia', 'alriyadh.com', 'alwatan.com.sa', 'al-jazirah.com', 'aawsat.com', 'albayan.ae', 'alarabiya',
    'makkahnewspaper', 'newspens', 'al-marsd', 'ajel.sa', 'okaz.com.sa', 'alyaum.com', 'youm7.com', 'qanoonsa.com',
    'decreesa.com', 'x.com/', 'mofa.gov.sa', 'makkah.gov.sa', 'GovDetail.asp', '02-spa/02-12-shura.htm', 'PRSNT.HTM',
    'MemENSer.asp', 'news/N2669887', 'news/1229f126d0', 'news/c7c196fb34', 'news/f4325ecb58', 'news/a13d5579ec',
    'news/c48c049a95', 'news/187f5a40fb', 'news/5ca7181c8e', 'news/89dceceea3', 'news/4f0cc76d24', 'news/5571925bef',
    'news/3b7e25e5d9', 'news/09ffa5f563', 'news/b25bf61edf', 'news/12c6728cfe', 'news/61a7845061', 'news/216a5ede04',
    'news/4058adbaf7', 'bush41library',
)
EMBASSY = {sid for sid in NEW_SOURCES if sid.startswith('sa_embassy_')}


def chair_invariants(packet):
    """The holder rules of CLAUDE-C01-25, over any copy of the Saudi packet."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    roles = {r['id']: r for e in packet['institutions'] for r in e['roles']}
    assert set(roles) == ROLES and len(roles) == len(ROLES)
    assert [e['id'] for e in packet['institutions']] == INSTITUTIONS
    for role_id, expected in HOLDERS.items():
        entries = roles[role_id]['holder_claims']
        dicts = [h for h in entries if isinstance(h, dict)]
        # The general rules first, so a mutation fails on the rule it breaks; the exact pins follow.
        current = None
        for h in entries:
            if isinstance(h, dict):
                current = h
                kinds = [EVENTS[cid][1] for cid in h['claim_ids']]
                assert all(EVENTS[cid][3] == h['name'] for cid in h['claim_ids']), h['name']
                assert kinds[0] in ANCHOR_KINDS and claims[h['claim_ids'][0]]['attested_on'] == h['attested_on'], h['name']
                assert set(kinds) <= HOLDER_KINDS, h['name']
                starts = [cid for cid in h['claim_ids'] if EVENTS[cid][1] in START_KINDS]
                ends = [cid for cid in h['claim_ids'] if EVENTS[cid][1] in END_KINDS]
                if h['from']:
                    assert [claims[cid]['attested_on'] for cid in starts] == [h['from']], h['name']
                    assert h['attested_on'] <= h['from'], h['name']
                else:
                    assert not starts, h['name']
                if h['until']:
                    assert ends == [h['claim_ids'][-1]] and claims[ends[0]]['attested_on'] == h['until'], h['name']
                else:
                    assert not ends, h['name']
                for key in ('from', 'until'):
                    if h[key]:
                        assert (key, h[key]) in STATED_BOUNDARIES and h[key] not in NEVER_BOUNDARY, (h['name'], key)
            else:
                # A holder observation belongs to the tenure it follows and falls inside it.
                assert current is not None, h
                day, kind, _, name = EVENTS[h]
                assert kind in OBSERVATION_KINDS and name == current['name'], h
                assert current['attested_on'] <= day <= (current['until'] or research.CUTOFF), h
        for a, b in zip(dicts, dicts[1:]):
            assert a['attested_on'] < b['attested_on']
            if a['until']:
                assert a['until'] <= b['attested_on'] and a['until'] not in {b['attested_on'], b['from']}
            # A successor's start is never an end: an until needs the holder's own stated end.
            assert not a['until'] or EVENTS[a['claim_ids'][-1]][1] in END_KINDS
        assert [h['name'] if isinstance(h, dict) else h for h in entries] == HOLDER_ORDER[role_id], role_id
        assert [(h['name'], h['attested_on'], h['from'], h['until'], h['claim_ids']) for h in dicts] == expected, role_id
    for role in roles.values():
        for h in role['holder_claims']:
            for cid in [h] if isinstance(h, str) else h['claim_ids']:
                if cid in EVENTS:
                    assert EVENTS[cid][1] not in NEVER_HOLDER_KINDS, (role['id'], cid)
                    assert role['id'] in (CHAIR, BCHAIR), (role['id'], cid)
    for role_id, expected in C01_06_HOLDERS.items():
        got = [h if isinstance(h, str) else (h['name'], h['attested_on'], h['from'], h['until']) for h in roles[role_id]['holder_claims']]
        assert got == expected, role_id
        assert not set(roles[role_id]['claim_ids']) & set(EVENTS) and not set(roles[role_id]['sources']) & set(NEW_SOURCES)
    for role_id, (sources, claim_ids) in UNTOUCHED.items():
        assert (roles[role_id]['sources'], roles[role_id]['claim_ids'], roles[role_id]['holder_claims']) == (sources, claim_ids, []), role_id
    sec = roles['sa_succession_secretary']
    assert (sec['sources'], sec['claim_ids'], [(h['name'], h['attested_on'], h['from'], h['until']) for h in sec['holder_claims']]) == C01_45_SECRETARY


class SaudiShuraAllegianceChairTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'saudi-arabia.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.entries = {e['id']: e for category in ('organizations', 'institutions') for e in cls.packet[category]}
        cls.roles = {r['id']: r for e in cls.entries.values() for r in e['roles']}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'SaudiArabia'}, {'SaudiArabia': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_appended_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), (71, 110))
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (136, 206, 12, 10))
        self.assertEqual([s['id'] for s in self.packet['sources'][BASE_SOURCES:-(C01_45_SOURCES + C01_50_SOURCES)]], NEW_SOURCES)
        self.assertEqual(len(self.packet['sources']), BASE_SOURCES + len(NEW_SOURCES) + C01_45_SOURCES + C01_50_SOURCES)
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], list(EVENTS))
        self.assertEqual({v[1] for v in EVENTS.values()}, HOLDER_KINDS | NEVER_HOLDER_KINDS)
        self.assertFalse(HOLDER_KINDS & NEVER_HOLDER_KINDS)
        self.assertEqual({v[2] for v in EVENTS.values()}, {f'SA-CHR-{n:02d}' for n in range(1, 11)})
        observations = re.findall(r'^### (SA-CHR-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'SA-CHR-{n:02d}' for n in range(1, 11)])
        # Every new claim is cited by exactly one place: its role when it has one, otherwise its institution.
        for cid, row in self.rows.items():
            with self.subTest(cid=cid):
                inst, role = self.entries[row['observation_id']], row['role_id']
                self.assertIn(row['observation_id'], ('sa_shura', 'sa_succession_commission'))
                self.assertIn(role, (CHAIR, BCHAIR, None))
                if role:
                    self.assertIn(cid, self.roles[role]['claim_ids'])
                    self.assertNotIn(cid, inst['claim_ids'])
                    self.assertEqual(role, CHAIR if row['observation_id'] == 'sa_shura' else BCHAIR)
                else:
                    self.assertIn(cid, inst['claim_ids'])
                    self.assertFalse([r for r in self.roles.values() if cid in r['claim_ids']])
        # Existing records stay first on the two roles and their institutions; the rest is appended in packet order.
        for key, (sources, claim_ids) in BASE_PREFIX.items():
            row = self.roles.get(key) or self.entries[key]
            self.assertEqual((row['sources'][:len(sources)], row['claim_ids'][:len(claim_ids)]), (sources, claim_ids), key)
            new = [cid for cid in row['claim_ids'][len(claim_ids):]]
            self.assertEqual(new, [cid for cid in EVENTS if cid in new])
            self.assertEqual(row['sources'][len(sources):], list(dict.fromkeys(self.claim_source[c] for c in new)))
        for key in ('sa_shura', 'sa_succession_commission'):
            unresolved = self.entries[key]['coverage']['unresolved']
            added = unresolved[BASE_UNRESOLVED[key]:len(unresolved) - LATER_UNRESOLVED[key]]
            later = unresolved[len(unresolved) - LATER_UNRESOLVED[key]:]
            self.assertTrue(all(u.startswith('CLAUDE-C01-45 (SA-KCP-') for u in later), key)
            self.assertTrue(added and all(u.startswith('CLAUDE-C01-25 (SA-CHR-') for u in added), key)
            self.assertFalse([u for u in unresolved[:BASE_UNRESOLVED[key]] if 'CLAUDE-C01-25' in u])
        packet_unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(len(packet_unresolved), BASE_UNRESOLVED['packet'] + 1)
        self.assertTrue(packet_unresolved[-1].startswith('CLAUDE-C01-25 fills sa_shura_chair'))
        for role_id in (CHAIR, BCHAIR):
            self.assertIn('CLAUDE-C01-25', self.roles[role_id]['scope_note'])

    def test_holders_are_exactly_as_intended(self):
        chair_invariants(self.packet)
        for role_id in (CHAIR, BCHAIR):
            for holder in self.roles[role_id]['holder_claims']:
                if isinstance(holder, str):
                    continue
                with self.subTest(role=role_id, name=holder['name']):
                    self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note', 'uncertainty'])
                    self.assertEqual(holder['sources'], list(dict.fromkeys(self.claim_source[c] for c in holder['claim_ids'])))
                    for cid in holder['claim_ids']:
                        row = self.rows[cid]
                        self.assertEqual((row['holder_name'], row['role_id']), (holder['name'], role_id), cid)
        by_name = lambda role_id, name: next(h for h in self.roles[role_id]['holder_claims'] if isinstance(h, dict) and h['name'] == name)
        jubair, humaid, alsheikh = (by_name(CHAIR, n) for n in (JUB, HUM, ASH))
        mishaal = by_name(BCHAIR, MSH)
        self.assertIn('until reverts to null', jubair['uncertainty'])
        self.assertIn('from is null', jubair['uncertainty'])
        self.assertIn('passed away this morning', self.claims['sa_jubair_death_20020124']['text'])
        self.assertIn('began his work in the Council', self.claims['sa_humaid_first_day_20020211']['text'])
        self.assertIn('neither is used as an end', humaid['uncertainty'])
        self.assertIn('اعتباراً من 3/3/1430هـ', self.claims['sa_alsheikh_effective_a13_20090228']['text'])
        self.assertIn('from reverts to null', alsheikh['uncertainty'])
        self.assertIn('continuity to 7 September 2026 is not established', alsheikh['uncertainty'])
        self.assertIn('هذا اليوم الأربعاء', self.claims['sa_mishaal_death_20170503']['text'])
        self.assertIn('Article 15', mishaal['uncertainty'])
        # Holder observations are dated primary records naming the holder; claim-only kinds never appear on a role.
        for role_id in (CHAIR, BCHAIR):
            for entry in self.roles[role_id]['holder_claims']:
                if isinstance(entry, str):
                    self.assertIn(EVENTS[entry][1], OBSERVATION_KINDS, entry)
                    self.assertEqual(self.claims[entry]['attested_on'], EVENTS[entry][0])
        never = {cid for cid, v in EVENTS.items() if v[1] in NEVER_HOLDER_KINDS}
        for role in self.roles.values():
            for entry in role['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertFalse(set(ids) & never, role['id'])

    def test_orders_effective_days_oaths_term_starts_and_deaths_never_collapse(self):
        for cid, (day, kind, obs, name) in EVENTS.items():
            with self.subTest(cid=cid):
                claim = self.claims[cid]
                self.assertEqual(claim.get('attested_on'), day)
                self.assertEqual('period' in claim, cid in PERIODS)
                if cid in PERIODS:
                    self.assertEqual((claim['period']['from'], claim['period']['through']), PERIODS[cid])
                    self.assertIsNone(day)
                if day:
                    self.assertLessEqual(day, research.CUTOFF)
                    stamp = re.search(r'_(\d{8})$', cid)
                    if stamp:
                        self.assertEqual(stamp.group(1), day.replace('-', ''))
                if kind == 'procedure':
                    self.assertIsNone(day)
                    self.assertIn('Procedure only', claim['uncertainty'])
                if kind in NEVER_HOLDER_KINDS - {'procedure', 'promulgation'} and name:
                    self.assertRegex(claim['uncertainty'], r'(?i)(never|claim only|claims only|context only|not a boundary)')
        text = lambda cid: self.claims[cid]['text']
        doubt = lambda cid: self.claims[cid]['uncertainty']
        # 1997: the order, its stated effective day, the extension and the oath are separate claims on separate days.
        self.assertEqual([EVENTS[c][0] for c in ('sa_order_a72_second_term_19970706', 'sa_shura_term_start_19970707',
                                                 'sa_jubair_extension_a90_19970706', 'sa_jubair_extension_effective_19970707',
                                                 'sa_jubair_oath_second_term_19970714')],
                         ['1997-07-06', '1997-07-07', '1997-07-06', '1997-07-07', '1997-07-14'])
        self.assertIn('effective from 3/3/1418 H (July 7, 1997)', text('sa_order_a72_second_term_19970706'))
        self.assertIn('from 3/3/1418 H (July 7, 1997)', text('sa_shura_term_start_19970707'))
        self.assertIn('as from 3/3/1418 H (July 7, 1997)', text('sa_jubair_extension_effective_19970707'))
        self.assertIn('not a from', doubt('sa_order_a72_second_term_19970706'))
        # 2001: the order (24 May), the oath (4 June), the announced term start and the first session (5 June).
        self.assertLess(EVENTS['sa_order_a80_third_term_20010524'][0], EVENTS['sa_jubair_oath_third_term_20010604'][0])
        self.assertIn('term of office has been renewed', text('sa_jubair_chairs_first_session_20010605'))
        self.assertIn('continuation wording', doubt('sa_jubair_chairs_first_session_20010605'))
        # 2002: death (24 Jan), acting presiding (27 Jan, 10 Feb), decree (7 Feb), first day (11 Feb), first session (12 Feb).
        order = ['sa_jubair_death_20020124', 'sa_vice_chair_presides_20020127', 'sa_humaid_appointed_20020207',
                 'sa_vice_chair_presides_55th_20020210', 'sa_humaid_first_day_20020211', 'sa_humaid_chairs_56th_session_20020212']
        self.assertEqual([EVENTS[c][0] for c in order], sorted(EVENTS[c][0] for c in order))
        self.assertIn("statement's", doubt('sa_humaid_oath_before_20020212'))
        self.assertIn("not the order's", doubt('sa_humaid_order_issued_before_20020212'))
        self.assertIn('28/11/1422 H. CORRESPONDING TO FEBRUARY 11, 2002', text('sa_calendar_14221128_20020211'))
        self.assertNotIn('printed weekday matches', doubt('sa_humaid_cv_chair_since_24_11_1422'))
        # 2009-2024: each order, effective day or term start, and oath stays its own claim.
        for order_id, start_id, oath_id in (
                ('sa_alsheikh_appointed_a13_20090214', 'sa_alsheikh_effective_a13_20090228', 'sa_alsheikh_oath_20090301'),
                ('sa_alsheikh_named_a54_20161202', 'sa_shura_term_start_20161202', 'sa_alsheikh_oath_20161213'),
                ('sa_alsheikh_named_a146_20201018', 'sa_shura_term_start_20201020', 'sa_alsheikh_oath_20201111'),
                ('sa_alsheikh_named_a84_20240902', 'sa_shura_term_start_20240906', 'sa_alsheikh_oath_20240918')):
            self.assertLessEqual(EVENTS[order_id][0], EVENTS[start_id][0])
            self.assertLess(EVENTS[start_id][0], EVENTS[oath_id][0])
        self.assertNotIn('past the', text('sa_shura_term_start_20240906'))
        self.assertIn('not computed', doubt('sa_shura_term_start_20240906'))
        self.assertIn('Scheduled only', doubt('sa_shura_oath_scheduled_20201108'))
        # 2006-2017: the statement, the order's Hijri day and the release are kept apart; the death ends the chair.
        self.assertIn('no Gregorian order date', doubt('sa_allegiance_law_a135_statement_20061019'))
        self.assertIn("Gregorian date is not printed", doubt('sa_allegiance_statute_a164_published_20071007'))
        self.assertIn('never read as the start', doubt('sa_mishaal_death_20170503'))
        # Names in claim text are as printed: never the encyclopaedia forms.
        records = json.dumps([self.sources[sid] for sid in NEW_SOURCES], ensure_ascii=False)
        for form in ('Al ash-Sheikh', 'al-Ashaikh', 'Mishal bin Abdulaziz', 'Ibn Humayd'):
            self.assertNotIn(form, records)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            with self.subTest(source=sid):
                self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
                self.assertEqual((extract['source_id'], extract['source_url'], extract['source_response_url']),
                                 (sid, source['url'], source['url']))
                self.assertEqual(extract['original_url'], source['original_url'])
                self.assertEqual((extract['source_title'], extract['scope_note']), (source['title'], source['scope_note']))
                self.assertEqual((extract['access_method'], extract['published_date']), (source['access_method'], source['published_date']))
                self.assertEqual((extract['accessed_date'], source['accessed_date']), (ACCESSED, ACCESSED))
                self.assertFalse(extract['source_response_checked_in'])
                self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
                self.assertIn('not checked into this repository', extract['provenance_note'])
                self.assertIn('derived factual extract', extract['provenance_note'])
                self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
                self.assertIn('CLAUDE-C01-25 only', extract['bounded_scope'])
                self.assertEqual(set(extract['visual_review']), {'pdf_pages_one_based', 'facsimile_pages_one_based', 'method'})
                self.assertTrue(source['source_type'].startswith('primary_'))
                snapshot = source['snapshot']
                self.assertEqual(snapshot['kind'], 'derived_factual_extract')
                self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/saudi-arabia-[a-z0-9-]+-\d{8}-facts\.json$')
                data = (research.ROOT / snapshot['path']).read_bytes()
                self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
                self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
                self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
                self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
                # This packet's two downloads were at least 30 minutes apart and matched the recorded identity.
                times = re.findall(r'This packet: downloaded at (\S+Z) and again at (\S+Z), (\d+) minutes later', extract['stability_check'])
                self.assertEqual(len(times), 1)
                first, second, minutes = times[0]
                gap = (datetime.fromisoformat(second.replace('Z', '+00:00')) - datetime.fromisoformat(first.replace('Z', '+00:00')))
                self.assertGreaterEqual(gap.total_seconds(), 1800)
                self.assertEqual(int(gap.total_seconds() // 60), int(minutes))
                self.assertIn(f'{RESPONSES[sid][0]} bytes, SHA-256 {RESPONSES[sid][1]}', extract['stability_check'])
                # Rows repeat the packet claims exactly, in order, keyed by claim_id; no row has a bare 'name' key.
                self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
                for row, claim in zip(extract['rows'], source['claims']):
                    self.assertEqual((row['text'], row['locator'], row['attested_on']), (claim['text'], claim['locator'], claim.get('attested_on')))
                    self.assertEqual(row.get('period'), claim.get('period'))
                    self.assertNotIn('name', row)
                    self.assertEqual('printed_name' in row, row['holder_name'] is not None)
                    self.assertIn(row['holder_name'], NAMES | {None})
                    if row['role_id']:
                        self.assertTrue(row['role_title'])
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['holder_name'])
                          for cid, row in self.rows.items()}, EVENTS)
        # Internet Archive raw captures before the cutoff, identity of the uncompressed body.
        archived = [sid for sid in NEW_SOURCES if self.sources[sid]['access_method'] == 'internet_archive_raw_capture']
        spa = [sid for sid in NEW_SOURCES if self.sources[sid]['access_method'] == 'spa_portal_news_api_json']
        self.assertEqual((len(archived), len(spa)), (32, 39))
        self.assertEqual(set(archived) | set(spa), set(NEW_SOURCES))
        for sid in archived:
            source, extract = self.sources[sid], self.extracts[sid]
            match = IA_URL.match(source['url'])
            self.assertIsNotNone(match, sid)
            stamp, original = match.groups()
            self.assertLess(stamp, '20260907')
            self.assertEqual(original, source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('the recorded identity is the uncompressed body', extract['provenance_note'])
            self.assertEqual('transport_response' in extract, sid in TRANSPORT)
            if sid in TRANSPORT:
                transport = extract['transport_response']
                self.assertEqual((transport['bytes'], transport['sha256'], transport['content_encoding']), TRANSPORT[sid] + ('gzip',))
                self.assertIn(f"{TRANSPORT[sid][0]}-byte gzip transport body (SHA-256 {TRANSPORT[sid][1]})", extract['provenance_note'])
        self.assertEqual(self.extracts['sa_boe_shura_law_en_20250513']['visual_review']['pdf_pages_one_based'], [1, 4])
        for sid in spa:
            source, extract = self.sources[sid], self.extracts[sid]
            match = SPA_API_URL.match(source['url'])
            self.assertIsNotNone(match, sid)
            self.assertEqual(source['original_url'], 'https://www.spa.gov.sa/' + match.group(1))
            self.assertEqual(extract['portal_metadata']['uuid'], match.group(1))
            self.assertEqual(extract['article_content_sha256'], ARTICLE_CONTENT[sid])
            self.assertIn('views_count', extract['provenance_note'])
            self.assertIn('cache-busting query (Cloudflare MISS, served by the origin)', extract['stability_check'])
        self.assertEqual(set(ARTICLE_CONTENT), set(spa))
        for sid in EMBASSY:
            self.assertEqual(self.sources[sid]['source_type'], 'primary_official_government_news_release')
            self.assertIn('Royal Embassy of Saudi Arabia', self.sources[sid]['publisher'])
            self.assertIn('not the Arabic original', self.sources[sid]['scope_note'])

    def test_leads_and_volatile_urls_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            recorded = {source['url'], extract['source_url'], extract['source_response_url']}
            self.assertEqual(len(recorded), 1, sid)
            url = source['url']
            # The recorded response is a fixed raw capture or a byte-stable single-item API response, never a live page
            # that embeds view counters or request tokens, a search, a listing that grows, or a cache-busting variant.
            self.assertIn(urlsplit(url).hostname, {'web.archive.org', 'portalapi.spa.gov.sa'}, sid)
            self.assertTrue(IA_URL.match(url) or SPA_API_URL.match(url), sid)
            for candidate in (url, source['original_url']):
                tail = candidate.split('id_/', 1)[-1]
                self.assertIsNone(VOLATILE.search(tail), candidate)
                for marker in LEAD_URL_MARKERS:
                    self.assertNotIn(marker.lower(), candidate.lower())
            self.assertIn(urlsplit(source['original_url']).hostname,
                          {'www.spa.gov.sa', 'laws.boe.gov.sa', 'www.shura.gov.sa', 'shura.gov.sa', 'www.saudiembassy.net',
                           'saudiembassy.net', 'www.uqn.gov.sa'}, sid)
        new_records = json.dumps([self.sources[sid] for sid in NEW_SOURCES] + [self.roles[CHAIR], self.roles[BCHAIR]],
                                 ensure_ascii=False).lower()
        for marker in LEAD_URL_MARKERS:
            self.assertNotIn(marker.lower(), new_records, marker)
        for token in ('okaz', 'alriyadh', 'wikipedia', 'encyclopa'):
            self.assertNotIn(token, new_records)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('N2669887', '1229f126d0', 'GovDetail.asp', '02-12-shura.htm', 'wikipedia', 'youm7', 'qanoonsa', '4058adbaf7'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_separation_from_the_executive_packet_and_the_bush_source_repair(self):
        chair_invariants(self.packet)
        for sid in NEW_SOURCES:
            self.assertNotIn('bush41', json.dumps(self.sources[sid]))
        for role_id in (CHAIR, BCHAIR):
            self.assertNotIn('sa_bush41_address_19900808', self.roles[role_id]['sources'])
        for entry in ('sa_shura', 'sa_succession_commission'):
            self.assertNotIn('sa_bush41_address_19900808', self.entries[entry]['sources'])
        self.assertEqual({s['id'] for s in self.packet['sources'][:BASE_SOURCES]} & set(NEW_SOURCES), set())
        for prefix in ('sa_boe_shura_law_', 'sa_embassy_', 'sa_shura_', 'sa_uqn_'):
            self.assertFalse([s for s in self.packet['sources'][:BASE_SOURCES] if s['id'].startswith(prefix)])
        # The new holders never reuse a C01-06 claim, and no executive claim feeds a chair.
        executive = {cid for rid in ('sa_king', 'sa_crown_prince', 'sa_pm') for cid in self.roles[rid]['claim_ids']}
        for role_id in (CHAIR, BCHAIR):
            self.assertFalse(executive & set(self.roles[role_id]['claim_ids']), role_id)
            for entry in self.roles[role_id]['holder_claims']:
                ids = [entry] if isinstance(entry, str) else entry['claim_ids']
                self.assertTrue(set(ids) <= set(EVENTS), role_id)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'saudi-arabia.json').read_bytes()
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

        def role(packet, role_id=CHAIR):
            return next(r for e in packet['institutions'] for r in e['roles'] if r['id'] == role_id)

        def holder(packet, name, role_id=CHAIR):
            return next(h for h in role(packet, role_id)['holder_claims'] if isinstance(h, dict) and h['name'] == name)

        def cite(packet, name, cid, role_id=CHAIR, at=None):
            h = holder(packet, name, role_id)
            h['claim_ids'].insert(len(h['claim_ids']) if at is None else at, cid)
            if self.claim_source[cid] not in h['sources']:
                h['sources'].append(self.claim_source[cid])

        validator_cases = [
            (lambda p: source(p, 'sa_embassy_jubair_death_20020124')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'sa_spa_order_a13_20090214')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'sa_shura_cv_alsheikh_20260808')['snapshot'].update(path='Cargo.toml'), 'escapes'),
            (lambda p: holder(p, ASH).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'sa_shura_chair_obs_20260907').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, HUM).update(until='2002-02-10'), 'Reversed historical interval'),
            (lambda p: holder(p, JUB)['claim_ids'].append('sa_humaid_first_day_20020211'), 'cited source'),
            (lambda p: source(p, 'sa_embassy_shura_oath_20010604').update(url=source(p, 'sa_embassy_shura_oath_20010604')['original_url']),
             'Invalid public source URL'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def new_holder(name, day, cid, role_id=CHAIR, index=None):
            def change(p):
                entries = role(p, role_id)['holder_claims']
                entry = {'name': name, 'attested_on': day, 'from': None, 'until': None,
                         'sources': [self.claim_source[cid]], 'claim_ids': [cid]}
                entries.insert(len(entries) if index is None else index, entry)
            return change

        invariant_cases = [
            ('Secretary observation day changed', lambda p: role(p, 'sa_succession_secretary')['holder_claims'][0].update(attested_on='2006-10-21')),
            ('Secretary appointment made an effective start', lambda p: role(p, 'sa_succession_secretary')['holder_claims'][0].update({'from': '2006-10-20'})),
            ('Secretary given an unsupported end', lambda p: role(p, 'sa_succession_secretary')['holder_claims'][0].update(until='2015-01-29')),
            ('successor start used as an end (Jubair)', lambda p: holder(p, JUB).update(until='2002-02-07')),
            ('successor start used as an end (Humaid)', lambda p: holder(p, HUM).update(until='2009-02-14')),
            ('successor order cited as an end (Humaid)', lambda p: (cite(p, HUM, 'sa_alsheikh_appointed_a13_20090214'),
                                                                   holder(p, HUM).update(until='2009-02-14'))),
            ('another office used as an end (Humaid 2009)', lambda p: (cite(p, HUM, 'sa_humaid_styled_sjc_chair_20090301'),
                                                                      holder(p, HUM).update(until='2009-03-01'))),
            ('last attestation used as an end (Jubair)', lambda p: holder(p, JUB).update(until='2002-01-15')),
            ('latest attestation used as an end (Al Al-Sheikh)', lambda p: holder(p, ASH).update(until='2026-09-07')),
            ('undated death statement used as the end', lambda p: holder(p, JUB).update(
                claim_ids=['sa_jubair_first_term_end_19970706', 'sa_jubair_death_stated_sg_column'])),
            ('acting presiding used as an end (Jubair)', lambda p: holder(p, JUB).update(until='2002-01-27')),
            ('decree day used as from (Humaid)', lambda p: holder(p, HUM).update({'from': '2002-02-07'})),
            ('first session used as from (Humaid)', lambda p: holder(p, HUM).update({'from': '2002-02-12'})),
            ('order day used as from (Al Al-Sheikh)', lambda p: holder(p, ASH).update({'from': '2009-02-14'})),
            ('oath used as from (Al Al-Sheikh 2009)', lambda p: holder(p, ASH).update({'from': '2009-03-01'})),
            ('term start used as from (2024)', lambda p: holder(p, ASH).update({'from': '2024-09-06'})),
            ('order day used as from (Mishaal)', lambda p: holder(p, MSH, BCHAIR).update({'from': '2007-12-09'})),
            ('extension day used as from (Jubair 1997)', lambda p: holder(p, JUB).update({'from': '1997-07-07'})),
            ('inauguration used as from (Jubair 1993)', lambda p: holder(p, JUB).update({'from': '1993-12-29', 'attested_on': '1993-12-29'})),
            ('effective day re-dated to the oath, claim and from together', lambda p: (
                claim(p, 'sa_alsheikh_effective_a13_20090228').update(attested_on='2009-03-01'), holder(p, ASH).update({'from': '2009-03-01'}))),
            ('death re-dated to the successor decree, claim and until together', lambda p: (
                claim(p, 'sa_jubair_death_20020124').update(attested_on='2002-02-07'), holder(p, JUB).update(until='2002-02-07'))),
            ('from set without a stated start (Mishaal oath)', lambda p: (cite(p, MSH, 'sa_mishaal_chair_thanks_20071211', BCHAIR, 1),
                                                                         holder(p, MSH, BCHAIR).update({'from': '2007-12-11'}))),
            ('forming order made a new tenure (2013)', new_holder(ASH, '2013-01-11', 'sa_alsheikh_named_a45_20130111')),
            ('acting presiding made a holder (Shatta)', new_holder(SHT, '2002-02-10', 'sa_vice_chair_presides_55th_20020210', index=10)),
            ('Article 15 used to name a successor', new_holder('Validation fixture', '2017-05-04', 'sa_allegiance_law_art15_chair_seniority', BCHAIR)),
            ('continuation cited by a holder (A/90)', lambda p: cite(p, JUB, 'sa_jubair_extension_a90_19970706', at=1)),
            ('retrospective CV cited by a holder', lambda p: cite(p, HUM, 'sa_humaid_cv_chair_since_24_11_1422', at=1)),
            ('scheduled oath as a holder observation', lambda p: role(p)['holder_claims'].append('sa_shura_oath_scheduled_20201108')),
            ('translation as a holder observation', lambda p: role(p)['holder_claims'].append('sa_alsheikh_speaker_en_20201018')),
            ('republication as a holder observation', lambda p: role(p)['holder_claims'].append('sa_uqn_a84_20240902')),
            ('untitled translation as a holder observation', lambda p: role(p, BCHAIR)['holder_claims'].append('sa_a180_english_no_chair_title_20071210')),
            ('observation filed under the wrong holder', lambda p: role(p)['holder_claims'].insert(
                role(p)['holder_claims'].index(holder(p, HUM)) + 1, role(p)['holder_claims'].pop(9))),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
            ('holder dropped', lambda p: role(p)['holder_claims'].remove(holder(p, HUM))),
            ('chair observation on a C01-06 role', lambda p: role(p, 'sa_king')['holder_claims'].append('sa_shura_chair_obs_20260907')),
            ('C01-06 holder end changed', lambda p: holder(p, 'Fahd bin Abdulaziz Al Saud', 'sa_king').update(until='2005-08-01')),
            ('untouched role given a claim', lambda p: role(p, 'sa_succession_members')['claim_ids'].append('sa_allegiance_oath_20071209')),
            ('new role added', lambda p: next(e for e in p['institutions'] if e['id'] == 'sa_shura')['roles'].append(
                {'id': 'sa_shura_vice_chair', 'title': 'Vice Chairman', 'kind': 'institutional_office', 'sources': ['sa_boe_shura_law'],
                 'claim_ids': ['sa_shura_composition'], 'holder_claims': []})),
            ('new institution added', lambda p: p['institutions'].append(copy.deepcopy(p['institutions'][3]))),
        ]
        chair_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration, ValueError)):
                chair_invariants(mutated(change))

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted', '02': 'Accepted in part', '03': 'Accepted', '04': 'Accepted', '05': 'Accepted',
                     '06': 'Accepted in part', '07': 'Accepted', '08': 'Accepted', '09': 'Accepted in part', '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| SA-CHR-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 14)] + [f'B{n}' for n in range(1, 11)] + [f'C{n}' for n in range(1, 10)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Declined|Resolved by removal)\.\*\*')
        missing = self.section('Missing primary records found by the checks')
        rows = [line for line in missing.splitlines() if re.match(r'\| [ABC]M\d+ ', line)]
        self.assertEqual(len(rows), 27)
        self.assertEqual(sum('**Imported**' in row for row in rows), 20)
        self.assertEqual(sum('**Not imported:**' in row for row in rows), 7)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section("Integration notes (outside this packet's file boundary)")
        for text in ('5b80d442', 'c7caa2b8', 'a33a8987', 'research-index.json', 'test_saudi_executive_c01_06.py',
                     'test_campaign_census', 'CLAUDE-C01-SOURCE-06', 'sa_bush41_address_19900808'):
            self.assertIn(text, notes)
        for heading in ('Sources added', 'Response identities and stability checks', 'Sources attempted',
                        'Suggested next work orders', 'Checks'):
            self.section(heading)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'saudi-shura-allegiance-chairs-1990-2026-25.md', 'claude/c01-sa-25', '5b80d442',
                     'c7caa2b8', 'test_saudi_shura_allegiance_c01_25.py', 'pending Codex acceptance'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SaudiArabia')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['source_claims'], country['role_observations'], country['mapping_pending']), (206, 10, 12))
        self.assertEqual([w['status'] for w in index['work_orders'] if w['nation'] == 'SaudiArabia'], ['open', 'open'])
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
