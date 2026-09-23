"""CLAUDE-C01-09: South Africa's heads of state, 1990-2024, keep election, oath, inauguration, resignation
announcement, resignation effect and acting service apart, and state a start or an end only where a source does."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256). Every new source is reproducible.
RESPONSES = {
    'za_gazette_12059_19890816': (389884, '13e0e473355303672ed304743d1fab06a86746fd4f878675b484a3dfda01e310'),
    'za_gazette_12108_19890919': (414469, '42c1a2f4a34ec348818c17c7c87b78b99cf9d81ca3b578fee1083f7872d8b2ef'),
    'za_gazette_12128_19890929': (354362, 'e1ce9e84b56c6806943f10fec535bbdd5e3622b760b01f57fc003d69df76c7fc'),
    'za_gazette_12286_19900202': (4750042, '7e86500b80a2d0fecf8be8589c14d99780f9896ddcffdd7abe47c7069bd14813'),
    'za_fwdk_foundation_inauguration_speech_19890920': (109220, 'f997290be427fa6a0f0fd382582d79687356bb3f1c2606f4da68dd03c2a0a126'),
    'za_presidency_history_list': (64973, 'bf8b63b08069814874cd3425c7d92f0e4a09f24d9d0ac310bae6ebfa4f269bac'),
    'za_gazette_15725_19940505': (335547, '7d80cf460ad0024e04e3e815620b27cd35cf8a4a62d3e022f31b6da16c2c1101'),
    'za_gazette_15740_19940509': (381753, 'dbea3c7411a12efa892cdeb8d35870fa8be8926b4717fa984ff731a692b3e830'),
    'za_parliament_students_parliament_event_2018': (43212, 'da3505ad4d580ab65161a077174b1e178ae89f4e7193598163474767c5d8a423'),
    'za_mandela_gov_oath_19940510': (4187, '503f9e6076abdfd057be9ab181149bccd10a94c2af8ffd9921482e786d3aeeec'),
    'za_mandela_gov_inauguration_address_19940510': (9284, '3358e6e3a8a98a70ddf7c794cd4339bb7bf295e9ad9d674536a31c26b244bca4'),
    'za_gazette_20215_19990614': (537340, 'b3c1e0ac91b03d174e806803a4ff982bfad315ab5c6a894173a33cf0acfe438b'),
    'za_dfa_mbeki_acceptance_election_19990614': (30799, 'df5d81339bfce6ae39e67641654f6b9c81dcab9acd95b84fb89f5abb575ef02f'),
    'za_dfa_mbeki_speech_index_2004': (120310, '3f705eaafbaac20991ebbac02212af06b42df50bcec57ee8c33e6a802a05f6cf'),
    'za_govza_mbeki_closing_debate_19990630': (61446, '30da93234f9d4e0e9cea59278055f606258e640c3ca54fa3063afe28625b9c9d'),
    'za_infogov_mbeki_inauguration_19990616': (15897, 'f8bfd381431f19ada9e35affa4a132b7245e5108f655ed2fbb71204fc55aba6a'),
    'za_infogov_mandela_farewell_19990616': (13066, '568765267e843656a20c07174adf4cb4365ca1c9132b7ae54e38ca8ee3f2f5d7'),
    'za_parliament_hansard_na_20040423': (126976, 'f81fe25027cfdeb8defd518a64512dd0b5134c433e37042c600e2367f0398e05'),
    'za_presidency_mbeki_post_election_statement_20040423': (22784, 'fd4b7287676473ed90018ceb4fddfb9f610a8d3fe3e568a5262d06ae90bd97b2'),
    'za_presidency_mbeki_inauguration_address_20040427': (37674, '3617e47ea3b12424d20c428c953af954c4e70d25b97fb3c1e11842a4369ba968'),
    'za_presidency_statement_step_down_20080920': (27701, 'd05a8ed46d9d47142794a63744172583fa9237295329f2a0e5fbd0745fcfda48'),
    'za_presidency_mbeki_address_to_nation_20080921': (18021, '6b09cbf2a05402d861bb980716cda1631d2090b97f8e1a0d493edaccff46f86c'),
    'za_presidency_statement_security_services_20080922': (28188, 'f495dcfa277a3b9796b4a968bba095d616acea5b662f76ba67651501fe8eec1f'),
    'za_parliament_hansard_na_20080922': (623104, 'b6ab9c44e717239d27a1a6a269d6795dba6957215280a5613de99a3a1a0badf5'),
    'za_parliament_hansard_na_20080923': (662016, '38a1080eb98974dd58b61802a49a7e58d1cda752af9850cea8badeb086169e91'),
    'za_presidency_swearing_in_advisory_20080924': (7014, 'b6eeadd6fce3c0b0058d11ba807960f10820ad354d7adfc93e43b6bb20c51da9'),
    'za_parliament_hansard_na_20080925': (178176, 'ad14dad050de5575d79fde9a9532c292ede85a78593fd115385dd57eb42a179f'),
    'za_presidency_cabinet_list_20080925': (33333, '14e0bd213098dd9c0ef9f0c076d7611f3eaba82cb2b37e2af4d216a77c015a9f'),
    'za_presidency_profile_motlanthe': (65736, 'e0c27b1b600bec2c0052dd14d940a67c43ace7441cbfcc0253257b6bfa90edde'),
    'za_presidency_political_context': (66159, '412ffa5ab46c630ab46d49cb427b63864b0089d29b67c918d0376676bd0670df'),
    'za_govza_directory_motlanthe': (35579, '88436ae8795b20d8c1e3cfc38b0a677a0e80a5855eee285214a51c2783c3780e'),
    'za_govza_directory_zuma': (37531, '92051c595fd8d84429a576a96d9caf77260d7be2ab6d66893099028503ec15df'),
    'za_parliament_hansard_na_20090506': (174080, '336921b4eb6b324bbd1ee8762536823ae123cde8ce90cd34a788223ed01f2c4c'),
    'za_parliament_minutes_na_20090506': (173801, '53eef917c0eea660095de8357cb71a3adea31032bbb54bf306951b0a6197f4f2'),
    'za_presidency_zuma_inauguration_address_20090509': (16099, '4311d0a17e21f2a8d624cee7a36c90fed0ae12535dfa0c8dca81277bf282e7d0'),
    'za_parliament_na_minutes_20140521': (172594, 'bafafbf03709136e69124690f1d681a13d3b1b7504c09c95b3f4a968a2237190'),
    'za_presidency_zuma_inauguration_address_20140524': (53974, '6db6a52914409a15e0848aabea81d63c8ecb7129f43bb6d502b2abeb8b405d10'),
    'za_presidency_profile_zuma': (84076, 'ca732f4e16a729d33cabc2221f2e3f7270717c6cc41e69a3278556c5b78ea43f'),
    'za_govza_zuma_resignation_statement_20180214': (52991, 'c3fbc5502cceb837faf393cea92b0658546f2853e64a7fdf4d69ced784261779'),
    'za_parliament_atc14_20180215': (4678819, 'e12e46e1181ba8f471b6afcb3db23bed4a8c62e3e8608b3159b6db3fa3f81894'),
    'za_parliament_speaker_receives_resignation_20180215': (52572, '10e486a9f46528db97bac1277e49a2d738aa867874c6b94329e17acac74b7969'),
    'za_gcis_acting_president_statement_20180215': (44484, '8e18b2cd67cf88a50c61716f3e824e1a26796a51d842b5afb37b41d930f6c8bc'),
    'za_parliament_na_minutes_20180215': (98236, '70fc3ce1a88aad550038eeaf3819b2fb6c8888f75fc1dbfc4a81b39e40cfcee9'),
    'za_parliament_procedural_developments_25_2018': (439100, '5aa356997ceaa271c1b3bee97be9ae83769db2f902e85abf0da934039cbdb92f'),
    'za_presidency_ramaphosa_profile': (68987, 'eb3fcd856a66a20fb654e75e8f63a7703caf7df41f5f573052d30ed2164cb1c5'),
    'za_parliament_na_minutes_20190522': (134623, '2de55070fece09c4c5f4af2f44bb96d3fe61f3390ee16d2a3f2120dbfe64243d'),
    'za_parliament_na_elects_president_20190522': (53984, 'b2684e604136ed8ee4ed0d0cb909a8275c5a3b60e9cd54ad39f34093b2768f00'),
    'za_presidency_inauguration_address_20190525': (59461, '507e9cd93c1387291466920df0d579e2a13f0bc3c6f112acef4ea1c55662cbe2'),
    'za_presidency_oath_of_office_20240619': (60579, '022df477036032958df69ec8953e30691a84ad69cd6b17a2ed6d95d47db568ac'),
    'za_presidency_inauguration_address_20240619': (74794, 'aa6f1b35ad4fbf539712be8b3229337b454aa93fe7e3edded50da3eeaec67cc7'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (byte-stable id_ form, all made before the 7 September 2026 cutoff): id -> timestamp.
ARCHIVED = {
    'za_fwdk_foundation_inauguration_speech_19890920': '20260521194407',
    'za_presidency_history_list': '20260415035431',
    'za_parliament_students_parliament_event_2018': '20210302202858',
    'za_mandela_gov_oath_19940510': '20131210154744',
    'za_mandela_gov_inauguration_address_19940510': '20131210135802',
    'za_dfa_mbeki_acceptance_election_19990614': '20041028004630',
    'za_dfa_mbeki_speech_index_2004': '20041024235722',
    'za_govza_mbeki_closing_debate_19990630': '20260622025941',
    'za_infogov_mbeki_inauguration_19990616': '20101204114051',
    'za_infogov_mandela_farewell_19990616': '20050621082812',
    'za_presidency_mbeki_post_election_statement_20040423': '20040603164733',
    'za_presidency_mbeki_inauguration_address_20040427': '20040520232213',
    'za_presidency_statement_step_down_20080920': '20080925143849',
    'za_presidency_mbeki_address_to_nation_20080921': '20080925002649',
    'za_presidency_statement_security_services_20080922': '20080925143854',
    'za_presidency_swearing_in_advisory_20080924': '20080926140147',
    'za_presidency_cabinet_list_20080925': '20081002133337',
    'za_presidency_profile_motlanthe': '20250814020016',
    'za_presidency_political_context': '20260510105155',
    'za_govza_directory_motlanthe': '20260624042057',
    'za_govza_directory_zuma': '20260622025522',
    'za_presidency_zuma_inauguration_address_20090509': '20100207002937',
    'za_presidency_zuma_inauguration_address_20140524': '20190511143348',
    'za_presidency_profile_zuma': '20240410150536',
    'za_govza_zuma_resignation_statement_20180214': '20180215060915',
    'za_parliament_speaker_receives_resignation_20180215': '20220302184735',
    'za_gcis_acting_president_statement_20180215': '20180215113627',
    'za_presidency_ramaphosa_profile': '20240614050310',
    'za_parliament_na_elects_president_20190522': '20210617084528',
    'za_presidency_inauguration_address_20190525': '20190719162723',
    'za_presidency_oath_of_office_20240619': '20240624125344',
    'za_presidency_inauguration_address_20240619': '20240619161128',
}
PDF_PAGES = {
    'za_gazette_12059_19890816': [1], 'za_gazette_12108_19890919': [1], 'za_gazette_12128_19890929': [1],
    'za_gazette_12286_19900202': [1, 2], 'za_gazette_15725_19940505': [1], 'za_gazette_15740_19940509': [1],
    'za_gazette_20215_19990614': [1], 'za_parliament_minutes_na_20090506': [1, 5, 6],
    'za_parliament_na_minutes_20140521': [1, 5], 'za_parliament_atc14_20180215': [3, 4, 6],
    'za_parliament_na_minutes_20180215': [1, 2], 'za_parliament_procedural_developments_25_2018': [4],
    'za_parliament_na_minutes_20190522': [1, 6],
}
ORIGINAL_SOURCES = ('za_iec_national_results_20240621', 'za_iec_national_seats_20240606', 'za_da_kzn_leader_20230403',
                    'za_da_leadership_20260412', 'za_parliament_president_elect_20240614')
SP, PR = 'za_state_president', 'za_president_election'

# Exact holder observations per role: (name, attested_on, from, until), in chronological order.
HOLDERS = {
    SP: [('F. W. de Klerk', '1990-02-02', None, None)],
    PR: [('Nelson Mandela', '1994-05-10', None, '1999-06-16'),
         ('Thabo Mbeki', '1999-06-16', None, None),
         ('Thabo Mbeki', '2004-04-27', None, '2008-09-25'),
         ('Kgalema Motlanthe', '2008-09-25', None, None),
         ('Jacob Zuma', '2009-05-09', None, None),
         ('Jacob Zuma', None, '2014-05-24', '2018-02-14'),
         ('Cyril Ramaphosa', '2018-02-15', None, None),
         ('Cyril Ramaphosa', '2019-05-25', None, None),
         ('Cyril Ramaphosa', '2024-06-14', None, None),
         ('Cyril Ramaphosa', '2024-06-19', None, None)],
}
HOLDER_CLAIMS = {
    SP: [['za_de_klerk_state_president_proclamation_r16_19900202']],
    PR: [['za_mandela_oath_of_office_19940510', 'za_mandela_oath_recalled_zuma_address_20090509',
          'za_mandela_presidency_ended_morning_19990616'],
         ['za_mbeki_styled_president_farewell_19990616'],
         ['za_mbeki_oath_of_office_20040427', 'za_na_resolves_mbeki_resignation_effective_20080923'],
         ['za_motlanthe_sworn_in_president_20080925', 'za_motlanthe_swearing_in_recess_20080925',
          'za_motlanthe_president_cabinet_list_20080925'],
         ['za_zuma_oath_of_office_20090509'],
         ['za_zuma_second_term_assumed_20140524', 'za_zuma_resignation_effective_20180214',
          'za_pd25_zuma_resignation_effective_20180214', 'za_gcis_zuma_resignation_vacancy_20180214'],
         ['za_ramaphosa_oath_of_office_20180215', 'za_presidency_ramaphosa_sworn_in_20180215'],
         ['za_ramaphosa_oath_of_office_20190525'],
         ['za_ramaphosa_president_elect_20240614'],
         ['za_ramaphosa_oath_of_office_20240619']],
}
EXISTING_HOLDER_INDEX = 8
# Claims that must never feed a holder observation.
ELECTIONS = (
    'za_cj_fixes_president_election_sitting_19940504', 'za_mandela_na_nomination_19940509',
    'za_ccpresident_fixes_president_election_sitting_19990608', 'za_mbeki_election_acceptance_speech_19990614',
    'za_dfa_index_mbeki_election_speech_19990614', 'za_mbeki_na_convened_elected_president_19990614',
    'za_mbeki_na_elected_president_20040423', 'za_mbeki_election_acceptance_statement_20040423',
    'za_motlanthe_na_elected_president_20080925', 'za_zuma_na_elected_president_20090506',
    'za_zuma_na_elected_president_minutes_20090506', 'za_zuma_na_elected_president_20140521',
    'za_ramaphosa_na_elected_president_20180215', 'za_parliament_ramaphosa_first_elected_20180215',
    'za_ramaphosa_na_elected_president_20190522', 'za_parliament_ramaphosa_elected_president_20190522')
ANNOUNCEMENTS = (
    'za_mbeki_inauguration_announced_20040423', 'za_mbeki_resignation_intention_announced_20080920',
    'za_mbeki_resignation_announcement_20080921', 'za_mbeki_resignation_submitted_20080921',
    'za_mbeki_resignation_letter_read_20080922', 'za_mbeki_resignation_effective_motion_deferred_20080922',
    'za_cj_vacancy_election_convened_20080923', 'za_new_president_swearing_in_scheduled_20080924',
    'za_zuma_resignation_announcement_20180214', 'za_pd25_zuma_resignation_announced_20180214',
    'za_zuma_resignation_tendered_20180214', 'za_zuma_resignation_letter_tabled_20180215',
    'za_zuma_resignation_letter_received_20180215', 'za_zuma_resignation_announced_to_na_20180215')
ACTING = ('za_de_klerk_acting_state_president_designated_19890815',
          'za_de_klerk_acting_state_president_proclamation_167_19890913', 'za_ramaphosa_acting_president_20180215')
CEREMONIES = (
    'za_de_klerk_post_inauguration_address_19890920', 'za_mandela_inauguration_ceremony_19940510',
    'za_mbeki_inauguration_ceremony_19990616', 'za_dfa_index_mbeki_inauguration_speech_19990616',
    'za_mbeki_inauguration_ceremony_20040427', 'za_zuma_inauguration_ceremony_20090509',
    'za_zuma_inaugurated_first_term_20090509', 'za_presidency_context_zuma_inaugurated_20090509',
    'za_zuma_inauguration_ceremony_20140524', 'za_ramaphosa_inauguration_ceremony_20190525',
    'za_ramaphosa_inauguration_ceremony_20240619')
SPANS = ('za_presidency_list_state_president_de_klerk_19890815_19940510',
         'za_presidency_list_president_mandela_19940510_19990616',
         'za_presidency_list_president_mbeki_19990616_20080924',
         'za_presidency_list_president_motlanthe_20080925_20090509', 'za_govza_directory_motlanthe_term_span',
         'za_govza_directory_zuma_term_span', 'za_presidency_context_motlanthe_succeeded_mbeki_20080925')
CONTEXT = ('za_botha_vacated_state_presidency_19890815', 'za_de_klerk_state_president_proclamation_177_19890928',
           'za_de_klerk_state_president_proclamation_98_19940509', 'za_mbeki_oath_june_1999_zuma_address_20090509',
           'za_mbeki_in_office_after_tender_20080922', 'za_mbeki_outgoing_president_20080925',
           'za_motlanthe_outgoing_president_20090506', 'za_motlanthe_thanked_by_successor_20090509')
NEVER_HOLDER = ELECTIONS + ANNOUNCEMENTS + ACTING + CEREMONIES + SPANS + CONTEXT
# Claims printed without a structured date: retrospective list and directory rows, and a month-only oath.
UNDATED = SPANS[:6] + ('za_mbeki_oath_june_1999_zuma_address_20090509',)
# Event kinds that may date or bound a holder; every other kind never does.
HOLDER_KINDS = {'in_office_signature_as_state_president', 'oath_of_office', 'oath_of_office_context',
                'in_office_attestation', 'end_of_term_statement', 'assumption_of_office', 'resignation_effective',
                'resignation_effective_date_resolution'}
# Dates that are never any holder's attested_on, start or end: acting service, post-inauguration address,
# scheduling notices, elections, the resignation announcements and tender, and the list's 24 September 2008 end.
NEVER_HOLDER_DATE = {'1989-08-15', '1989-09-13', '1989-09-20', '1994-05-04', '1994-05-09', '1999-06-08', '1999-06-14',
                     '2004-04-23', '2008-09-20', '2008-09-21', '2008-09-22', '2008-09-23', '2008-09-24', '2009-05-06',
                     '2014-05-21', '2019-05-22'}
# Every new claim's (attested_on, event_kind), exactly: distinct dated events are never re-dated or relabelled.
EVENTS = {
    'za_botha_vacated_state_presidency_19890815': ('1989-08-15', 'vacated_office'),
    'za_de_klerk_acting_state_president_designated_19890815': ('1989-08-15', 'acting_service_start'),
    'za_de_klerk_acting_state_president_proclamation_167_19890913': ('1989-09-13', 'acting_service'),
    'za_de_klerk_state_president_proclamation_177_19890928': ('1989-09-28', 'in_office_signature_as_state_president'),
    'za_de_klerk_state_president_proclamation_r16_19900202': ('1990-02-02', 'in_office_signature_as_state_president'),
    'za_de_klerk_post_inauguration_address_19890920': ('1989-09-20', 'post_inauguration_address'),
    'za_presidency_list_state_president_de_klerk_19890815_19940510': (None, 'retrospective_list_row'),
    'za_presidency_list_president_mandela_19940510_19990616': (None, 'retrospective_list_row'),
    'za_presidency_list_president_mbeki_19990616_20080924': (None, 'retrospective_list_row'),
    'za_presidency_list_president_motlanthe_20080925_20090509': (None, 'retrospective_list_row'),
    'za_cj_fixes_president_election_sitting_19940504': ('1994-05-04', 'national_assembly_sitting_scheduled'),
    'za_de_klerk_state_president_proclamation_98_19940509': ('1994-05-09', 'in_office_signature_as_state_president'),
    'za_mandela_na_nomination_19940509': ('1994-05-09', 'national_assembly_nomination'),
    'za_mandela_oath_of_office_19940510': ('1994-05-10', 'oath_of_office'),
    'za_mandela_inauguration_ceremony_19940510': ('1994-05-10', 'inauguration_ceremony'),
    'za_ccpresident_fixes_president_election_sitting_19990608': ('1999-06-08', 'national_assembly_sitting_scheduled'),
    'za_mbeki_election_acceptance_speech_19990614': ('1999-06-14', 'election_acceptance_statement'),
    'za_dfa_index_mbeki_election_speech_19990614': ('1999-06-14', 'speech_index_entry'),
    'za_dfa_index_mbeki_inauguration_speech_19990616': ('1999-06-16', 'speech_index_entry'),
    'za_mbeki_na_convened_elected_president_19990614': ('1999-06-14', 'election_reference_retrospective'),
    'za_mbeki_inauguration_ceremony_19990616': ('1999-06-16', 'inauguration_ceremony'),
    'za_mandela_presidency_ended_morning_19990616': ('1999-06-16', 'end_of_term_statement'),
    'za_mbeki_styled_president_farewell_19990616': ('1999-06-16', 'in_office_attestation'),
    'za_mbeki_na_elected_president_20040423': ('2004-04-23', 'national_assembly_election'),
    'za_mbeki_inauguration_announced_20040423': ('2004-04-23', 'inauguration_announced_prospective'),
    'za_mbeki_election_acceptance_statement_20040423': ('2004-04-23', 'election_acceptance_statement'),
    'za_mbeki_inauguration_ceremony_20040427': ('2004-04-27', 'inauguration_ceremony'),
    'za_mbeki_resignation_intention_announced_20080920': ('2008-09-20', 'resignation_intention_announced'),
    'za_mbeki_resignation_announcement_20080921': ('2008-09-21', 'resignation_announcement'),
    'za_mbeki_in_office_after_tender_20080922': ('2008-09-22', 'in_office_after_tender'),
    'za_mbeki_resignation_letter_read_20080922': ('2008-09-22', 'resignation_letter_announced'),
    'za_mbeki_oath_of_office_20040427': ('2004-04-27', 'oath_of_office'),
    'za_mbeki_resignation_effective_motion_deferred_20080922': ('2008-09-22', 'resignation_effective_date_motion_deferred'),
    'za_mbeki_resignation_submitted_20080921': ('2008-09-21', 'resignation_letter_submitted'),
    'za_na_resolves_mbeki_resignation_effective_20080923': ('2008-09-23', 'resignation_effective_date_resolution'),
    'za_cj_vacancy_election_convened_20080923': ('2008-09-23', 'vacancy_election_convened'),
    'za_new_president_swearing_in_scheduled_20080924': ('2008-09-24', 'oath_scheduled_prospective'),
    'za_motlanthe_na_elected_president_20080925': ('2008-09-25', 'national_assembly_election'),
    'za_motlanthe_swearing_in_recess_20080925': ('2008-09-25', 'oath_of_office_context'),
    'za_mbeki_outgoing_president_20080925': ('2008-09-25', 'predecessor_reference'),
    'za_motlanthe_president_cabinet_list_20080925': ('2008-09-25', 'in_office_attestation'),
    'za_motlanthe_sworn_in_president_20080925': ('2008-09-25', 'oath_of_office'),
    'za_presidency_context_motlanthe_succeeded_mbeki_20080925': ('2008-09-25', 'succession_statement_retrospective'),
    'za_presidency_context_zuma_inaugurated_20090509': ('2009-05-09', 'inauguration_ceremony'),
    'za_govza_directory_motlanthe_term_span': (None, 'retrospective_term_span'),
    'za_govza_directory_zuma_term_span': (None, 'retrospective_term_span'),
    'za_zuma_na_elected_president_20090506': ('2009-05-06', 'national_assembly_election'),
    'za_motlanthe_outgoing_president_20090506': ('2009-05-06', 'in_office_attestation'),
    'za_zuma_na_elected_president_minutes_20090506': ('2009-05-06', 'national_assembly_election'),
    'za_mandela_oath_recalled_zuma_address_20090509': ('1994-05-10', 'oath_of_office'),
    'za_mbeki_oath_june_1999_zuma_address_20090509': (None, 'oath_of_office'),
    'za_zuma_inauguration_ceremony_20090509': ('2009-05-09', 'inauguration_ceremony'),
    'za_zuma_oath_of_office_20090509': ('2009-05-09', 'oath_of_office'),
    'za_motlanthe_thanked_by_successor_20090509': ('2009-05-09', 'predecessor_reference'),
    'za_zuma_na_elected_president_20140521': ('2014-05-21', 'national_assembly_election'),
    'za_zuma_inauguration_ceremony_20140524': ('2014-05-24', 'inauguration_ceremony'),
    'za_zuma_inaugurated_first_term_20090509': ('2009-05-09', 'inauguration_ceremony'),
    'za_zuma_second_term_assumed_20140524': ('2014-05-24', 'assumption_of_office'),
    'za_zuma_resignation_tendered_20180214': ('2018-02-14', 'resignation_tendered'),
    'za_zuma_resignation_announcement_20180214': ('2018-02-14', 'resignation_announcement'),
    'za_zuma_resignation_effective_20180214': ('2018-02-14', 'resignation_effective'),
    'za_zuma_resignation_letter_tabled_20180215': ('2018-02-15', 'resignation_letter_tabled'),
    'za_zuma_resignation_letter_received_20180215': ('2018-02-15', 'resignation_letter_received'),
    'za_gcis_zuma_resignation_vacancy_20180214': ('2018-02-14', 'resignation_effective'),
    'za_ramaphosa_acting_president_20180215': ('2018-02-15', 'acting_service'),
    'za_zuma_resignation_announced_to_na_20180215': ('2018-02-15', 'resignation_letter_announced'),
    'za_ramaphosa_na_elected_president_20180215': ('2018-02-15', 'national_assembly_election'),
    'za_pd25_zuma_resignation_announced_20180214': ('2018-02-14', 'resignation_announcement'),
    'za_pd25_zuma_resignation_effective_20180214': ('2018-02-14', 'resignation_effective'),
    'za_ramaphosa_oath_of_office_20180215': ('2018-02-15', 'oath_of_office'),
    'za_presidency_ramaphosa_sworn_in_20180215': ('2018-02-15', 'oath_of_office'),
    'za_ramaphosa_na_elected_president_20190522': ('2019-05-22', 'national_assembly_election'),
    'za_parliament_ramaphosa_elected_president_20190522': ('2019-05-22', 'national_assembly_election'),
    'za_parliament_ramaphosa_first_elected_20180215': ('2018-02-15', 'election_reference_retrospective'),
    'za_ramaphosa_inauguration_ceremony_20190525': ('2019-05-25', 'inauguration_ceremony'),
    'za_ramaphosa_oath_of_office_20190525': ('2019-05-25', 'oath_of_office'),
    'za_ramaphosa_oath_of_office_20240619': ('2024-06-19', 'oath_of_office'),
    'za_ramaphosa_inauguration_ceremony_20240619': ('2024-06-19', 'inauguration_ceremony'),
}
STARTS = [('Jacob Zuma', '2014-05-24')]
ENDS = [('Nelson Mandela', '1999-06-16'), ('Thabo Mbeki', '2008-09-25'), ('Jacob Zuma', '2018-02-14')]
SURNAMES = {'F. W. de Klerk': 'de Klerk', 'Nelson Mandela': 'Mandela', 'Thabo Mbeki': 'Mbeki',
            'Kgalema Motlanthe': 'Motlanthe', 'Jacob Zuma': 'Zuma', 'Cyril Ramaphosa': 'Ramaphosa'}
LEAD_URL_MARKERS = ('sahistory', 'wikipedia', 'parliament-week-07-22', 'pmg.org.za', 'omalley', 'disa.ukzn',
                    'sanews', 'news24', 'mg.co.za', 'voanews', 'politicsweb', 'anc1912', 'anc.org.za', 'polity.org',
                    'brandsouthafrica', 'westerncape.gov.za', 'hathitrust', 'thabo-mvuyelwa-mbeki-mr',
                    '940509_inauguration', '990614123p1003', '20260624042110')
REPORT = research.RESEARCH / 'south-africa-heads-of-state-1990-2024-09.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-09.md'


def hos_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError or KeyError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert len(packet['institutions']) == 1, 'no second presidency institution'
    presidency = packet['institutions'][0]
    assert presidency['id'] == 'za_presidency'
    roles = {r['id']: r for r in presidency['roles']}
    assert list(roles) == [PR, SP], 'exactly the President and the State President roles'
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
             if r['kind'] == 'head_of_state']
    assert heads == [PR, SP], 'no other head-of-state role'
    for role_id, expected in HOLDERS.items():
        holders = roles[role_id]['holder_claims']
        assert all(isinstance(h, dict) for h in holders)
        got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders]
        assert got == expected, (role_id, got)
        assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS[role_id], role_id
        for h in holders:
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert not {h['attested_on'], h['from'], h['until']} & NEVER_HOLDER_DATE, h['name']
            assert 'Acting' not in h['name'], h['name']
            for cid in h['claim_ids']:
                assert cid in roles[role_id]['claim_ids'], cid
    # The State President is never a holder of the President's role, and no President holds the State Presidency.
    assert 'F. W. de Klerk' not in [h['name'] for h in roles[PR]['holder_claims']]
    assert not set(roles[SP]['claim_ids']) & set(roles[PR]['claim_ids'])
    starts = [(h['name'], h['from']) for r in (PR, SP) for h in roles[r]['holder_claims'] if h['from']]
    ends = [(h['name'], h['until']) for r in (PR, SP) for h in roles[r]['holder_claims'] if h['until']]
    assert starts == STARTS and ends == ENDS, (starts, ends)
    # Acting service stays a claim on its role.
    for cid in ACTING:
        assert cid in roles[SP if 'de_klerk' in cid else PR]['claim_ids'], cid
    # Retrospective spans and the month-only oath carry no structured date.
    for cid in UNDATED:
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid
    # Distinct dated events stay distinct.
    assert claims['za_mandela_na_nomination_19940509']['attested_on'] == '1994-05-09'
    assert claims['za_mandela_oath_of_office_19940510']['attested_on'] == '1994-05-10'
    assert claims['za_mbeki_election_acceptance_speech_19990614']['attested_on'] == '1999-06-14'
    assert claims['za_mbeki_inauguration_ceremony_19990616']['attested_on'] == '1999-06-16'
    assert claims['za_mbeki_na_elected_president_20040423']['attested_on'] == '2004-04-23'
    assert claims['za_mbeki_oath_of_office_20040427']['attested_on'] == '2004-04-27'
    assert claims['za_mbeki_resignation_announcement_20080921']['attested_on'] == '2008-09-21'
    assert claims['za_na_resolves_mbeki_resignation_effective_20080923']['attested_on'] == '2008-09-23'
    assert claims['za_zuma_na_elected_president_20090506']['attested_on'] == '2009-05-06'
    assert claims['za_zuma_oath_of_office_20090509']['attested_on'] == '2009-05-09'
    assert claims['za_zuma_resignation_effective_20180214']['attested_on'] == '2018-02-14'
    assert claims['za_zuma_resignation_letter_received_20180215']['attested_on'] == '2018-02-15'
    assert claims['za_ramaphosa_na_elected_president_20190522']['attested_on'] == '2019-05-22'
    assert claims['za_ramaphosa_oath_of_office_20190525']['attested_on'] == '2019-05-25'


class SouthAfricaHeadsOfStateTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'south-africa.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.presidency = cls.packet['institutions'][0]
        cls.roles = {r['id']: r for r in cls.presidency['roles']}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'SouthAfrica'}, {'SouthAfrica': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (50, 78))
        self.assertEqual([s['id'] for s in self.packet['sources']], list(ORIGINAL_SOURCES) + NEW_SOURCES)
        self.assertEqual(len(ids['entries']), 53)
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = {cid for role in HOLDER_CLAIMS.values() for ids_ in role for cid in ids_}
        holder_claims.discard('za_ramaphosa_president_elect_20240614')
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(self.new_claims))
        # Every new claim is cited by the presidency and by exactly one of its two roles.
        for cid in self.new_claims:
            self.assertIn(cid, self.presidency['claim_ids'])
            self.assertEqual(sum(cid in r['claim_ids'] for r in self.roles.values()), 1, cid)
        for entry in self.packet['organizations']:
            self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
        observations = re.findall(r'^### (ZA-HOS-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'ZA-HOS-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'ZA-HOS-{n:02d}' for n in range(1, 11)})
        # Renamed per the checks: no claim ID keeps a collapsed event kind or a dropped source.
        for stale in ('za_de_klerk_inauguration_state_president_19890920', 'za_mbeki_na_election_19990614',
                      'za_cj_letter_mbeki_resignation_effective_20080923', 'za_mandela_na_election_19940509',
                      'za_mandela_president_luncheon_19990613', 'za_mandela_capetown_address_19940509',
                      'za_presidency_list_president_mbeki_19990616"'):
            self.assertNotIn(stale, self.raw)

    def test_holders_are_exactly_as_intended(self):
        hos_invariants(self.packet)
        for role_id in (PR, SP):
            for holder in self.roles[role_id]['holder_claims']:
                self.assertTrue(holder['note'] and holder.get('uncertainty') or holder['claim_ids'] == [
                    'za_ramaphosa_president_elect_20240614'], holder['name'])
                expected_sources = []
                for cid in holder['claim_ids']:
                    if self.claim_source[cid] not in expected_sources:
                        expected_sources.append(self.claim_source[cid])
                self.assertEqual(holder['sources'], expected_sources, holder['name'])
                # Every cited claim names the holder, and its row carries the holder's name and a holder event kind.
                for cid in holder['claim_ids']:
                    if cid == 'za_ramaphosa_president_elect_20240614':
                        continue
                    self.assertIn(SURNAMES[holder['name']], self.claims[cid]['text'], cid)
                    self.assertEqual(self.rows[cid]['holder_name'], holder['name'], cid)
                    self.assertIn(self.rows[cid]['event_kind'], HOLDER_KINDS, cid)
                    self.assertEqual(self.rows[cid]['role_id'], role_id, cid)
        # The existing 2024 election holder is unchanged and stays between the 2019 and 2024 oath observations.
        existing = self.roles[PR]['holder_claims'][EXISTING_HOLDER_INDEX]
        self.assertEqual(existing, {
            'name': 'Cyril Ramaphosa', 'attested_on': '2024-06-14', 'from': None, 'until': None,
            'sources': ['za_parliament_president_elect_20240614'], 'claim_ids': ['za_ramaphosa_president_elect_20240614'],
            'note': 'Dated observation only; no continuous term, game identity, biography or likeness rights granted.'})
        self.assertEqual(self.claim_source['za_ramaphosa_president_elect_20240614'], 'za_parliament_president_elect_20240614')
        # No claim that never feeds a holder does so anywhere; no event kind outside HOLDER_KINDS dates a holder.
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], {'assumption_of_office', 'end_of_term_statement'}, cid)
        for cid in ACTING:
            self.assertTrue(self.rows[cid]['role_title'].startswith('Acting '), cid)
            self.assertIn('never a holder', self.claims[cid]['uncertainty'])
        self.assertEqual(self.rows['za_mbeki_oath_june_1999_zuma_address_20090509']['event_kind'], 'oath_of_office')

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims = self.claims
        self.assertIn('assumed his second term in office as President of the Republic on 24 May 2014',
                      claims['za_zuma_second_term_assumed_20140524']['text'])
        self.assertIn("'the new status I have occupied since this morning'",
                      claims['za_mandela_presidency_ended_morning_19990616']['text'])
        self.assertIn("will take effect on 25 September 2008'", claims['za_na_resolves_mbeki_resignation_effective_20080923']['text'])
        self.assertIn("with immediate effect'", claims['za_zuma_resignation_effective_20180214']['text'])
        self.assertIn('CHECK AGAINST DELIVERY', claims['za_mandela_presidency_ended_morning_19990616']['uncertainty'])
        self.assertIn("not used as Mbeki's start", claims['za_mandela_presidency_ended_morning_19990616']['uncertainty'])
        # Oath and swearing-in statements date observations and are never starts.
        for cid in ('za_mandela_oath_of_office_19940510', 'za_mbeki_oath_of_office_20040427',
                    'za_motlanthe_sworn_in_president_20080925', 'za_zuma_oath_of_office_20090509',
                    'za_ramaphosa_oath_of_office_20180215', 'za_presidency_ramaphosa_sworn_in_20180215',
                    'za_ramaphosa_oath_of_office_20190525', 'za_ramaphosa_oath_of_office_20240619'):
            self.assertEqual(self.rows[cid]['event_kind'], 'oath_of_office', cid)
            self.assertRegex(claims[cid]['uncertainty'], r'not (used as )?a start', cid)
        # Retrospective lists and directory spans are recorded, never boundaries; each says why.
        for cid in SPANS[:6]:
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'], cid)
        self.assertIn("'from 14 June 1999'", claims['za_govza_directory_motlanthe_term_span']['uncertainty'])
        self.assertIn('merges acting and substantive service',
                      claims['za_presidency_list_state_president_de_klerk_19890815_19940510']['uncertainty'])
        self.assertIn('conflicts with', claims['za_presidency_list_president_mbeki_19990616_20080924']['uncertainty'])
        motlanthe = self.roles[PR]['holder_claims'][3]
        self.assertIsNone(motlanthe['until'])
        self.assertIn('not used as boundaries', motlanthe['uncertainty'])
        de_klerk = self.roles[SP]['holder_claims'][0]
        self.assertIn('No start', de_klerk['uncertainty'])
        self.assertIn('No end', de_klerk['uncertainty'])
        mbeki_1999 = self.roles[PR]['holder_claims'][1]
        self.assertIn("not used as Mbeki's start", mbeki_1999['uncertainty'])
        self.assertIn('never holders or boundaries', self.roles[PR]['scope_note'])
        self.assertIn('never a holder of it', self.roles[SP]['scope_note'])
        self.assertIn('procedure only, never a date', self.roles[SP]['scope_note'])
        # A party decision is context, never an office event.
        for cid in self.new_claims:
            self.assertNotIn('recall', self.rows[cid]['event_kind'])
        self.assertIn('party decision', claims['za_mbeki_resignation_intention_announced_20080920']['uncertainty'])
        self.assertIn('context only', claims['za_zuma_resignation_announcement_20180214']['uncertainty'])
        # Coverage records what remains open.
        unresolved = self.presidency['coverage']['unresolved']
        self.assertTrue(unresolved[0].startswith('Heads of state 1990-2024 (CLAUDE-C01-09'))
        self.assertIn("Motlanthe's end", unresolved[1])
        self.assertIn('Keep executive election distinct from party leadership', unresolved[-1])
        self.assertEqual(sum('CLAUDE-C01-09' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source['published_date'])
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-22', '2026-09-22'))
            self.assertLessEqual(source['published_date'] or '', '2024-06-23')
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['rights_note'], source['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'] and source['scope_note'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/south-africa-'))
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
                self.assertEqual(row['observation_id'], 'za_presidency')
                self.assertNotIn('name', row)
                self.assertIn(row['claim_id'], self.roles[row['role_id']]['claim_ids'])
                self.assertIn(sid, self.roles[row['role_id']]['sources'])
                self.assertTrue(row['role_title'].endswith(('President of the Republic of South Africa',
                                                            'State President of the Republic of South Africa')))
        self.assertEqual({cid: (row['attested_on'], row['event_kind']) for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED]
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others},
                         {'archive.gazettes.africa', 'www.parliament.gov.za'})
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            if 'gazettes.africa' in self.sources[sid]['url']:
                self.assertIn('Cloudflare', self.sources[sid]['scope_note'])
            else:
                self.assertIn('Last-Modified 2 March 2026', self.sources[sid]['scope_note'])
        # Archival copies state their limitation.
        for sid in ('za_fwdk_foundation_inauguration_speech_19890920', 'za_mandela_gov_oath_19940510'):
            self.assertEqual(self.sources[sid]['source_type'], 'archival_copy_of_official_speech')
            self.assertIn('Archival copy limitation', self.sources[sid]['scope_note'])
        self.assertEqual(self.sources['za_presidency_history_list']['original_url'], 'https://thepresidency.gov.za/history')
        for sid in ('za_mandela_gov_oath_19940510', 'za_mandela_gov_inauguration_address_19940510'):
            self.assertIsNone(self.sources[sid]['published_date'])
            self.assertIn('December 2013', self.sources[sid]['scope_note'])

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'sahistory', 'pmg.org.za', 'news24', 'voanews', 'loftus versfeld stadium'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('parliament-week-07-22', '940509_inauguration', '990614123p1003', 'sahistory.org.za',
                       'pmg.org.za', 'thabo-mvuyelwa-mbeki-mr', 'presidential-inauguration-2024',
                       '20260624042110', 'hathitrust'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'south-africa.json').read_bytes()
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

        def role(packet, role_id=PR):
            return next(r for r in packet['institutions'][0]['roles'] if r['id'] == role_id)

        def holder(packet, index, role_id=PR):
            return role(packet, role_id)['holder_claims'][index]

        validator_cases = [
            (lambda p: source(p, 'za_parliament_atc14_20180215')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'za_gazette_12286_19900202')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'za_presidency_history_list')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 5).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 0, SP).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'za_ramaphosa_oath_of_office_20240619').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update(until='2014-05-23'), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('za_zuma_oath_of_office_20090509'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('za_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor start used as an end (Motlanthe)', lambda p: holder(p, 3).update(until='2009-05-09')),
            ('successor start used as an end (de Klerk)', lambda p: holder(p, 0, SP).update(until='1994-05-10')),
            ('successor oath used as an end (Ramaphosa 2018)', lambda p: holder(p, 6).update(until='2019-05-25')),
            ('re-election used as an end (Mbeki 1999)', lambda p: holder(p, 1).update(until='2004-04-27')),
            ('election date used as start (Zuma 2009)', lambda p: holder(p, 4).update({'from': '2009-05-06'})),
            ('election date used as start (Mbeki 2004)', lambda p: holder(p, 2).update({'from': '2004-04-23'})),
            ('election date used as observation (Mandela)', lambda p: holder(p, 0).update(attested_on='1994-05-09')),
            ('oath used as start (Ramaphosa 2018)', lambda p: holder(p, 6).update({'from': '2018-02-15'})),
            ('oath used as start (Mandela)', lambda p: holder(p, 0).update({'from': '1994-05-10'})),
            ('predecessor end used as start (Mbeki 1999)', lambda p: holder(p, 1).update({'from': '1999-06-16'})),
            ('announcement used as end', lambda p: holder(p, 2).update(until='2008-09-21')),
            ('list date used as end', lambda p: holder(p, 2).update(until='2008-09-24')),
            ('receipt used as end', lambda p: holder(p, 5).update(until='2018-02-15')),
            ('acting start used for the State President', lambda p: holder(p, 0, SP).update({'from': '1989-08-15'})),
            ('State President added to za_president_election', lambda p: role(p)['holder_claims'].insert(0, {
                'name': 'F. W. de Klerk', 'attested_on': '1990-02-02', 'from': None, 'until': None,
                'sources': ['za_gazette_12286_19900202'],
                'claim_ids': ['za_de_klerk_state_president_proclamation_r16_19900202']})),
            ('State President claim moved onto the President role', lambda p: role(p)['claim_ids'].append(
                'za_de_klerk_state_president_proclamation_r16_19900202')),
            ('acting service as holder', lambda p: role(p)['holder_claims'].insert(6, {
                'name': 'Cyril Ramaphosa', 'attested_on': '2018-02-15', 'from': None, 'until': None,
                'sources': ['za_gcis_acting_president_statement_20180215'],
                'claim_ids': ['za_ramaphosa_acting_president_20180215']})),
            ('acting claim cited by a holder', lambda p: holder(p, 0, SP)['claim_ids'].append(
                'za_de_klerk_acting_state_president_proclamation_167_19890913')),
            ('election claim cited by a holder', lambda p: holder(p, 4)['claim_ids'].append(
                'za_zuma_na_elected_president_20090506')),
            ('ceremony claim cited by a holder', lambda p: holder(p, 7)['claim_ids'].append(
                'za_ramaphosa_inauguration_ceremony_20190525')),
            ('list row given a date', lambda p: claim(p, 'za_presidency_list_president_motlanthe_20080925_20090509').update(
                attested_on='2009-05-09')),
            ('election collapsed into oath', lambda p: claim(p, 'za_mbeki_na_elected_president_20040423').update(
                attested_on='2004-04-27')),
            ('second presidency institution', lambda p: p['institutions'].append(copy.deepcopy(p['institutions'][0]))),
            ('State President role removed', lambda p: p['institutions'][0]['roles'].pop()),
        ]
        hos_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError)):
                hos_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted in part', '02': 'Accepted in part', '03': 'Accepted in part', '04': 'Accepted',
                     '05': 'Accepted in part', '06': 'Accepted in part', '07': 'Accepted', '08': 'Accepted',
                     '09': 'Accepted', '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| ZA-HOS-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 13)] + [f'B{n}' for n in range(1, 12)] + [f'C{n}' for n in range(1, 12)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Resolved by removal)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('0e6bbf94', 'ffe54b02', 'research-index.json', 'test_south_africa_research_s10h.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'south-africa-heads-of-state-1990-2024-09.md', 'claude/c01-za-09', '0e6bbf94',
                     'ffe54b02', 'test_south_africa_heads_of_state_c01_09.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SouthAfrica')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'SouthAfrica'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
