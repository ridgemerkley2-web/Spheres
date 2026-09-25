"""CLAUDE-C01-21: South Africa's Deputy Presidents, 1994-2026, are a separate role in za_presidency. The President's
announcement, the appointment and its notice, the oath and its scheduling, a stated assumption, a release or resignation
and its stated effect, and Assembly-seat events stay separate claims; a holder has a start or an end only where a
source states one, and acting service as President is never a holder."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research

DP = 'za_deputy_president'
PR, SP = 'za_president_election', 'za_state_president'
DP_TITLE = 'Deputy President of the Republic of South Africa'
ANC_ID = 'za_iec_n2024_014'
# The packet's sources before this packet: the S10h intake, CLAUDE-C01-09 and CLAUDE-C01-16 (pinned in their tests).
EARLIER_SOURCE_COUNT = 109

# Original response identity recorded in each extract, (bytes, sha256), fetched as the extract's fetch_recipe says.
RESPONSES = {
    'za_govza_mandela_inauguration_19940510': (47416, 'a8e8a97ad842e278f156f86f914077cbc3739d1e149c79f8cf61100277a35c1f'),
    'za_un_spv3379_19940525': (145245, 'c6ba5d28bc8660253b3fa0e66494ac5b9dbe0f48f1bdf097324c9106e3c23469'),
    'za_dfa_mbeki_security_council_statement_19940525': (28919, '2efd3e354a71067b169d60392be2c7a784d727c1a3c8ce8b88dbd06683b37e51'),
    'za_gazette_15792_19940603': (2117040, '3f3735f069be6caf24dfd3a680f41050dbde59bcd88f79b152beb0c5b65868bf'),
    'za_presidency_profile_former_president_mbeki': (68368, '55478200cf4ef9820d07dba88d6a8841101595833a105ca7df39e24646b00799'),
    'za_infogov_executive_deputy_president_rdp_statement_19960418': (13463, '3983491cced8bab63754e9a60e64d2d527d2bf73922ebd9159e48923958034e5'),
    'za_mandela_gov_np_withdrawal_statement_19960509': (6755, '3f1caf6067a1cbc5fa3f6dda65d9fcc018feeee09f74e99fde05b9dfeb7f5a81'),
    'za_mandela_gov_new_cabinet_members_statement_19960513': (5876, '6a8fb70472b334a77c06736116d38ff89818c058057db15320f34c350948b2bb'),
    'za_gazette_20261_19990702': (1932684, '9e8e6237fdf81afb14cb0d54f35cbe6137fa7cca21a37f8907c28e4f85473be1'),
    'za_parliament_hansard_na_19990628': (330752, '715a3397257fb2c5b512df8b630e5a4ac66d97403d617ae10edff1e61086fa1f'),
    'za_presidency_profile_former_deputy_president_zuma': (83907, 'f3abdd3e48066515f16efa69fb5e124be7edd8a5adec21163f9176727b952dab'),
    'za_presidency_history_list_deputy_presidents': (64973, 'bf8b63b08069814874cd3425c7d92f0e4a09f24d9d0ac310bae6ebfa4f269bac'),
    'za_parliament_hansard_joint_sitting_20050614': (41984, 'f9cbda1a0f50f399f77fb1070b18e6f2b4b423c90b5edb6cb313dd3978d311a8'),
    'za_infogov_mbeki_release_of_zuma_statement_20050614': (18939, '19899ee85d7030a9d8a711a9aa2c599462cfa6a41f479c77cc8afb99c2aacf80'),
    'za_infogov_zuma_statement_on_release_20050614': (11335, '37beb00de39921167fa26182315e39c7d024083879e5747979bd76564b019d40'),
    'za_presidency_statement_npa_decision_20050620': (24358, '7ae6b8a620ece06bd49c7eff33a1201012b202a9ec01df083af6ca027e15913a'),
    'za_dfa_gcis_cabinet_statement_20050622': (17742, 'c87e98a389de48f537bda8f06cacb0e2e414c1a9a7578643536aa6ad0a91daf8'),
    'za_parliament_hansard_na_20050622': (275968, 'd0d032d2d4c9f5771884adf025b0097a4d40637b727fdb4639d34733d34d876c'),
    'za_dfa_presidency_swearing_in_advisory_20050623': (13420, 'fd7c40477220404dd36dcdd833833190cf2eabb5b3120f4ca748db027cd7f026'),
    'za_presidency_deputy_profile_mlambo_ngcuka_2007': (25332, '6712eb12cd8f1a0ac4756fcd3accf7bb36cef31feeb47bd0fc581ae41e9611e9'),
    'za_presidency_profile_mlambo_ngcuka_20260219': (65525, '0f377b62aefc4a1ea2ef7e718a4a206096f9e002caa148e19e0d506d4d30e5e1'),
    'za_infogov_presidency_cabinet_resignations_20080923': (7731, 'f5e9cfd64381c2f5bb28d33b1b1fb948e47138c6d6f8fed5bb86b7a9d27ee0fb'),
    'za_presidency_latest_news_list_20081014': (85540, '9c5501f20ce4a15d24805256c618dae72ab2cbbe3c21ab126f9cf19e377236cd'),
    'za_parliament_hansard_na_20080925_deputy_president': (178176, 'ad14dad050de5575d79fde9a9532c292ede85a78593fd115385dd57eb42a179f'),
    'za_presidency_cabinet_list_20080925_deputy_president': (33333, '14e0bd213098dd9c0ef9f0c076d7611f3eaba82cb2b37e2af4d216a77c015a9f'),
    'za_parliament_hansard_na_20081021': (507904, '7ee380c49ee2945bfd6f457044298a987a3bf5eb2981d568c650facdef14cf72'),
    'za_presidency_deputy_profile_mbete_2009': (36104, '0cfdbc6d5fa0d2402ceddec43d22e696e5c46af080731de00b549c3cadb0ba19'),
    'za_presidency_zuma_cabinet_statement_20090510': (28712, '1c3e0630388c31b1c631ab42d8ef30c73b6e2d3cb59fed4f75a91ed28a6df760'),
    'za_govza_directory_motlanthe_deputy_president': (35579, '88436ae8795b20d8c1e3cfc38b0a677a0e80a5855eee285214a51c2783c3780e'),
    'za_govza_zuma_national_executive_20140525': (52362, 'a0dff1bdd86345e55cfbd0f6ab84acb880275abac34ab7c41898088075eb065a'),
    'za_parliament_atc6_20140609': (126866, '1c4f61774139699c7cfa990adf7c6fed19e1035ebbc25bd4e53cde20bef87f30'),
    'za_parliament_na_minutes_20140618': (102826, '52220ece1e82a072fbf4d775397af16bdb14aadfcbfa8497399198ae75cb2e79'),
    'za_parliament_procedural_developments_21_2014': (355964, '9d20ae19bc3525683ff2d5243aa348594adf1c8a36ffd5e8d503e132c8d49ee2'),
    'za_presidency_profile_former_deputy_president_ramaphosa': (68591, 'dbf134ba7a7483f35db1f908e41c9b5db78c53f0b12623ec4d5c44a61e268f3e'),
    'za_govza_ramaphosa_national_executive_changes_20180226': (56872, 'b0de5d4133c97d5ce371e13382877d74044ac384ff0f0bc24d3e10a9a6fcb945'),
    'za_parliament_na_minutes_20180227': (238856, '2c7ceb4185776d10c518d91283d90e404121a1e40f6024eaa0c9d570c5994695'),
    'za_govza_presidency_swearing_in_advisory_20180227': (44521, '992ae2392605713733a9847ce3d197054b18b51acefc68493e8ffc16aa9d5cb8'),
    'za_parliament_atc21_20180305': (97958, 'fcb24b7654ca539e7136aedc0a830fcf4fd7cc8468409a7d648468ee3354939c'),
    'za_presidency_profile_mabuza': (66591, 'c4503a0e6e2b0fcffb1d6f7e1b6f1ee96292a6cffb5edac11ef823007a16fb3b'),
    'za_govza_ramaphosa_national_executive_20190529': (51202, 'acd3350b821c824caf1f524c75740a16e1a3bd6f4a8c3fc2990f30cf9f62942d'),
    'za_govza_presidency_swearing_in_advisory_20190530': (54267, '0b891ff5cc01da84f490bcdbe67d7122506636c97737a69be0b3193b299ddf44'),
    'za_parliament_sixth_parliament_composition_20190606': (56431, '552b74f72d38ac295b442b0d5bcba93d3dbf4ae60be635a948991a1688983900'),
    'za_parliament_atc6_20190612': (67674, 'da17455cd4e2ecf3d29dfcedce97d5cc43d7039f4e962b068e0a33d3c2a9160d'),
    'za_govza_ramaphosa_sona_reply_20230216': (68292, 'ce65860b2115b5bf9a844b91739890a1cf645e5b68ee4dda70e2011433aa314e'),
    'za_parliament_atc25_20230301': (201134, '050a78a2b9f1e1eaeedfcd4b380e6c342ec5b6fe12f70a60c87fe3f722830059'),
    'za_govza_presidency_mabuza_resignation_20230301': (54351, '4fd05f78bb777336d51cf24139715171bf5aa2dae816f56a6a1bb6f044b8cefc'),
    'za_parliament_mabuza_assembly_resignation_20230301': (59685, 'b0503c592c9ddde3d14ea6e34e43ad414cafaf447856bac63ca64e2441cc2ee3'),
    'za_govza_directory_mabuza': (34408, '3e10b25f71121198c76594707104253c623c789a32b236772dca70ac7d226ba1'),
    'za_govza_ramaphosa_new_national_executive_20230306': (50649, '740037e6336c7c27b0444cb1fa327a14e9e76ca2c021fb4c9a308b4980ceafb6'),
    'za_govza_presidency_swearing_in_advisory_20230307': (43590, 'e70113ceb6b5c9a854815b42ca564fd62a26f99fe775193e1e91f638c1809667'),
    'za_parliament_atc31_20230309': (1759642, '41717dbf70b7196defd61ede5be10135a9ffa5a98426cfd1475d70f75609111e'),
    'za_presidency_deputy_president_office_appointments_20230411': (13746, 'f215d57a7b32074ec1318d1862d890168bcd962439b1852e4a8ff9caf3fdb15c'),
    'za_presidency_profile_mashatile': (66973, 'da02a48f7a79519b028cc372c8d03952d45aa9884018069201a2d933c4845476'),
    'za_presidency_national_executive_20240630': (72987, '712b0081b9ed5c932015db43bd561d4cfddd5fe9a4f486ba6fc3162f917062f1'),
    'za_govza_presidency_swearing_in_advisory_20240701': (46239, 'eb8a28e8b36f7af5484fdd03a7995c8cf701928e4f19fa9dd746e999be7c56bd'),
    'za_parliament_atc4_20240705': (155962, 'f4a050d330af5b2922ee6017c0189d8b9245c1d7cf61df51906aef1d530cb1d1'),
    'za_presidency_opening_of_parliament_address_20240718': (91572, '175b965ac4fd3255dd7d3524113b403d0f69bab7a160731e5f83ead28ccac7ae'),
    'za_govza_directory_mashatile': (36842, 'e60660747ce4f6eeb1849a1b1fe67c235ab5b384503bf36774b026ff41e805d7'),
    'za_govza_deputy_president_recovering_20260830': (13095, 'ddc3525bce74e7acb058ef731da9b004618cb9e143365eea13001c7a21d0f69e'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (id_ form, all made before the 7 September 2026 cutoff): id -> capture timestamp.
ARCHIVED = {
    'za_govza_mandela_inauguration_19940510': '20260622025944',
    'za_dfa_mbeki_security_council_statement_19940525': '20041028003708',
    'za_presidency_profile_former_president_mbeki': '20251114031652',
    'za_infogov_executive_deputy_president_rdp_statement_19960418': '20080127171652',
    'za_mandela_gov_np_withdrawal_statement_19960509': '20131210152110',
    'za_mandela_gov_new_cabinet_members_statement_19960513': '20131210203657',
    'za_presidency_profile_former_deputy_president_zuma': '20260215193421',
    'za_presidency_history_list_deputy_presidents': '20260415035431',
    'za_infogov_mbeki_release_of_zuma_statement_20050614': '20060427000645',
    'za_infogov_zuma_statement_on_release_20050614': '20081201082441',
    'za_presidency_statement_npa_decision_20050620': '20071113142741',
    'za_dfa_gcis_cabinet_statement_20050622': '20060924022538',
    'za_dfa_presidency_swearing_in_advisory_20050623': '20060924021138',
    'za_presidency_deputy_profile_mlambo_ngcuka_2007': '20070208183308',
    'za_presidency_profile_mlambo_ngcuka_20260219': '20260219034143',
    'za_infogov_presidency_cabinet_resignations_20080923': '20080925232835',
    'za_presidency_latest_news_list_20081014': '20081014101332',
    'za_presidency_cabinet_list_20080925_deputy_president': '20081002133337',
    'za_presidency_deputy_profile_mbete_2009': '20090430151759',
    'za_presidency_zuma_cabinet_statement_20090510': '20090518003642',
    'za_govza_directory_motlanthe_deputy_president': '20260624042057',
    'za_govza_zuma_national_executive_20140525': '20260705051807',
    'za_presidency_profile_former_deputy_president_ramaphosa': '20250814210606',
    'za_govza_ramaphosa_national_executive_changes_20180226': '20230129215710',
    'za_govza_presidency_swearing_in_advisory_20180227': '20180227122205',
    'za_presidency_profile_mabuza': '20260808202248',
    'za_govza_ramaphosa_national_executive_20190529': '20260626022845',
    'za_govza_presidency_swearing_in_advisory_20190530': '20190530105105',
    'za_parliament_sixth_parliament_composition_20190606': '20210617051708',
    'za_govza_ramaphosa_sona_reply_20230216': '20260621183313',
    'za_govza_presidency_mabuza_resignation_20230301': '20230302084617',
    'za_parliament_mabuza_assembly_resignation_20230301': '20251207005320',
    'za_govza_directory_mabuza': '20260622025611',
    'za_govza_ramaphosa_new_national_executive_20230306': '20260627102206',
    'za_govza_presidency_swearing_in_advisory_20230307': '20260627055121',
    'za_presidency_deputy_president_office_appointments_20230411': '20250405034841',
    'za_presidency_profile_mashatile': '20260611072933',
    'za_presidency_national_executive_20240630': '20260610065618',
    'za_govza_presidency_swearing_in_advisory_20240701': '20240703022751',
    'za_presidency_opening_of_parliament_address_20240718': '20240723235600',
    'za_govza_directory_mashatile': '20260621160552',
    'za_govza_deputy_president_recovering_20260830': '20260831130647',
}
# Captures the archive stores and serves gzip-encoded: decoded identity (bytes, sha256), recorded beside the served one.
GZIP = {
    'za_presidency_deputy_president_office_appointments_20230411': (63123, '68862c8b23f45584f202130b4bc6488ca60453f584aef0fe7c2c9d52b8312b94'),
    'za_govza_deputy_president_recovering_20260830': (43514, 'ee91981115b82f12ed5eaae159f76429318548b42b6e51f42a1d5e9621bc927e'),
}
# Four responses already cited by CLAUDE-C01-09 are carried by separate source records for their deputy-president rows,
# so the CLAUDE-C01-09 extracts stay unchanged: new id -> existing id with the same url and response.
REIMPORTS = {
    'za_presidency_history_list_deputy_presidents': 'za_presidency_history_list',
    'za_parliament_hansard_na_20080925_deputy_president': 'za_parliament_hansard_na_20080925',
    'za_presidency_cabinet_list_20080925_deputy_president': 'za_presidency_cabinet_list_20080925',
    'za_govza_directory_motlanthe_deputy_president': 'za_govza_directory_motlanthe',
}
# PDF pages rendered and visually reviewed.
PDF_PAGES = {
    'za_un_spv3379_19940525': [1, 2], 'za_gazette_15792_19940603': [1], 'za_gazette_20261_19990702': [1],
    'za_parliament_procedural_developments_21_2014': [1, 7, 27], 'za_parliament_atc31_20230309': [3],
}
OTHER_HOSTS = {'documents.un.org', 'archive.gazettes.africa', 'www.parliament.gov.za'}

# Exact holder observations of za_deputy_president, (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('F. W. de Klerk', '1994-05-10', None, None),
    ('Thabo Mbeki', '1994-05-25', None, None),
    ('Jacob Zuma', None, '1999-06-17', None),
    ('Phumzile Mlambo-Ngcuka', '2005-06-22', None, None),
    ('Baleka Mbete', '2008-10-21', None, None),
    ('Kgalema Motlanthe', '2009-05-11', None, None),
    ('Cyril Ramaphosa', '2014-05-30', None, None),
    ('David Mabuza', '2018-02-27', None, None),
    ('David Mabuza', '2019-05-30', None, None),
    ('Paul Mashatile', '2023-03-07', None, None),
    ('Paul Mashatile', '2024-07-04', None, None),
    ('Paul Mashatile', '2026-08-30', None, None),
]
HOLDER_CLAIMS = [
    ['za_de_klerk_styled_second_deputy_president_inauguration_19940510'],
    ['za_mbeki_first_executive_deputy_president_unsc_19940525', 'za_dfa_mbeki_first_deputy_statement_security_council_19940525'],
    ['za_zuma_deputy_president_appointment_effective_19990617'],
    ['za_mlambo_ngcuka_assumed_duties_20050622', 'za_mlambo_ngcuka_appointment_noted_by_assembly_20050622'],
    ['za_mbete_deputy_president_logb_announced_20081021'],
    ['za_motlanthe_sworn_in_deputy_president_caption_20090511'],
    ['za_ramaphosa_deputy_president_logb_letter_20140530'],
    ['za_presidency_profile_mabuza_sworn_in_20180227'],
    ['za_presidency_profile_mabuza_sworn_in_20190530'],
    ['za_mashatile_dp_sworn_in_20230307', 'za_presidency_profile_mashatile_sworn_in_20230307'],
    ['za_mashatile_deputy_president_logb_letter_20240704'],
    ['za_mashatile_dp_in_office_20260830'],
]
# The observation each holder answers.
HOLDER_REVIEW = ['ZA-DP-01', 'ZA-DP-01', 'ZA-DP-03', 'ZA-DP-04', 'ZA-DP-05', 'ZA-DP-06', 'ZA-DP-07', 'ZA-DP-08',
                 'ZA-DP-08', 'ZA-DP-09', 'ZA-DP-10', 'ZA-DP-10']
# Claims that must never feed a holder observation.
ANNOUNCEMENTS = (
    'za_mlambo_ngcuka_appointment_decision_announced_20050622', 'za_motlanthe_intends_appointing_mbete_deputy_president_20080925',
    'za_mbete_deputy_president_cabinet_list_20080925', 'za_mbete_designation_congratulated_in_house_20080925',
    'za_motlanthe_deputy_president_announced_20090510', 'za_ramaphosa_dp_appointment_announced_20140525',
    'za_ramaphosa_styled_dp_20140525', 'za_mabuza_dp_appointment_intended_20180226',
    'za_mabuza_dp_appointment_announced_20190529', 'za_mashatile_dp_appointment_announced_20230306',
    'za_mashatile_dp_reappointment_announced_20240630')
APPOINTMENT_RECORDS = (
    'za_zuma_deputy_president_appointment_gazetted_19990702', 'za_mlambo_ngcuka_appointed_per_profile_20050621',
    'za_ramaphosa_dp_appointment_communicated_na_20140716', 'za_presidency_profile_ramaphosa_dp_appointed_20140525',
    'za_mabuza_dp_appointed_20180226')
CEREMONIES = (
    'za_mlambo_ngcuka_swearing_in_scheduled_20050623', 'za_mabuza_dp_swearing_in_scheduled_20180227',
    'za_ministers_swearing_in_scheduled_20190530', 'za_national_executive_swearing_in_scheduled_20230307',
    'za_national_executive_swearing_in_scheduled_20240701')
DEPARTURES = (
    'za_np_gnu_withdrawal_communicated_by_de_klerk_19960509',
    'za_single_executive_deputy_president_and_vacancies_from_19960701_announced_19960513',
    'za_mbeki_announces_release_of_zuma_joint_sitting_20050614', 'za_mbeki_releases_zuma_as_deputy_president_20050614',
    'za_zuma_departure_void_in_executive_20050614', 'za_zuma_accepts_release_20050614',
    'za_zuma_styled_former_deputy_president_20050620', 'za_deputy_president_resignation_accepted_20080923',
    'za_deputy_president_resignation_effect_by_reference_20080923', 'za_mlambo_ngcuka_announces_resignation_20080923',
    'za_mlambo_ngcuka_styled_former_deputy_president_20081021', 'za_mabuza_step_down_request_sona_reply_20230216',
    'za_mabuza_resignation_ends_dp_term_20230301', 'za_mabuza_step_down_request_referenced_presidency_20230216',
    'za_deputy_president_successor_to_be_announced_20230301',
    'za_mabuza_step_down_request_referenced_cabinet_statement_20230216')
ASSEMBLY_AND_OTHER_OFFICES = (
    'za_mbete_vacates_speakership_for_deputy_presidency_20080925', 'za_mabuza_assembly_seat_filled_20180226',
    'za_mabuza_assembly_oath_20190528', 'za_mabuza_assembly_resignation_letter_20230228',
    'za_mabuza_assembly_resignation_received_20230228')
CONTINUATION = (
    'za_mbeki_executive_deputy_president_rdp_statement_19960418', 'za_de_klerk_deputy_president_thanked_in_office_19960509',
    'za_zuma_deputy_president_leader_of_government_business_announced_19990628',
    'za_ramaphosa_logb_designation_announced_minutes_20140618', 'za_ramaphosa_dp_logb_designation_announced_20140618',
    'za_mabuza_deputy_president_logb_letter_20180301', 'za_mabuza_styled_dp_parliament_20190606',
    'za_mabuza_deputy_president_logb_letter_20190605', 'za_mabuza_styled_dp_sona_reply_20230216',
    'za_mashatile_deputy_president_logb_letter_20230308', 'za_mashatile_styled_dp_20240718')
CONTEXT = ('za_executive_deputy_presidents_consulted_on_cabinet_19940603',)
# Claims printed without a structured date: retrospective lists, profiles and directory spans, month-only and live
# page titles.
UNDATED = (
    'za_presidency_profile_mbeki_executive_deputy_president_span', 'za_presidency_profile_zuma_appointed_deputy_president_1999',
    'za_presidency_list_deputy_president_mbeki_19940513_19990616', 'za_presidency_list_deputy_president_de_klerk_19940513_19960630',
    'za_presidency_list_deputy_president_zuma_19990617_20050614',
    'za_presidency_list_deputy_president_mlambo_ngcuka_20050623_20080924',
    'za_presidency_list_deputy_president_mbete_20080905', 'za_presidency_profile_mlambo_ngcuka_span_20050622_20080923',
    'za_presidency_profile_mbete_deputy_since_september_2008', 'za_govza_directory_motlanthe_deputy_span_20090511_20140525',
    'za_govza_directory_mabuza_dp_span', 'za_presidency_profile_mashatile_current_title_at_capture',
    'za_govza_directory_mashatile_dp_spans')
NEVER_HOLDER = (ANNOUNCEMENTS + APPOINTMENT_RECORDS + CEREMONIES + DEPARTURES + ASSEMBLY_AND_OTHER_OFFICES + CONTINUATION
                + CONTEXT + UNDATED)
# Event kinds that may date a holder, and the one kind that states a start; every other kind never does either.
HOLDER_KINDS = {'in_office_attestation', 'oath_of_office', 'assumption_of_office', 'appointment_noted_by_resolution',
                'appointment_effective'}
BOUNDARY_KINDS = {'appointment_effective', 'assumption_of_office', 'resignation_effective', 'end_of_term_statement',
                  'term_end_effective', 'release_effective'}
# Dates that are never any holder's attested_on, start or end: retrospective list and directory days, announcements,
# intentions, notices, scheduled oaths, releases, resignations and their references, Assembly-seat events and
# continuation attestations.
NEVER_HOLDER_DATE = {
    '1994-05-13', '1994-06-03', '1996-04-18', '1996-05-09', '1996-05-13', '1996-06-30', '1996-07-01', '1999-06-13',
    '1999-06-16', '1999-06-28', '1999-07-02', '2005-06-14', '2005-06-20', '2005-06-21', '2005-06-23', '2008-09-05',
    '2008-09-23', '2008-09-24', '2008-09-25', '2008-10-06', '2009-05-10', '2014-05-25', '2014-05-26', '2014-06-18',
    '2014-07-16', '2018-02-26', '2018-03-01', '2019-05-28', '2019-05-29', '2019-06-05', '2019-06-06', '2023-02-16',
    '2023-02-28', '2023-03-01', '2023-03-06', '2023-03-08', '2024-06-19', '2024-06-30', '2024-07-01', '2024-07-03',
    '2024-07-18', '2026-06-11'}
STARTS = [('Jacob Zuma', '1999-06-17')]
ENDS = []
# Every new claim's (attested_on, event_kind), exactly: distinct dated events are never re-dated or relabelled.
EVENTS = {
    'za_de_klerk_styled_second_deputy_president_inauguration_19940510': ('1994-05-10', 'in_office_attestation'),
    'za_mbeki_first_executive_deputy_president_unsc_19940525': ('1994-05-25', 'in_office_attestation'),
    'za_dfa_mbeki_first_deputy_statement_security_council_19940525': ('1994-05-25', 'in_office_attestation'),
    'za_executive_deputy_presidents_consulted_on_cabinet_19940603': ('1994-06-03', 'office_referenced_unnamed'),
    'za_presidency_profile_mbeki_executive_deputy_president_span': (None, 'retrospective_term_span'),
    'za_mbeki_executive_deputy_president_rdp_statement_19960418': ('1996-04-18', 'in_office_continuation_attestation'),
    'za_np_gnu_withdrawal_communicated_by_de_klerk_19960509': ('1996-05-09', 'withdrawal_decision_communicated'),
    'za_de_klerk_deputy_president_thanked_in_office_19960509': ('1996-05-09', 'in_office_continuation_attestation'),
    'za_single_executive_deputy_president_and_vacancies_from_19960701_announced_19960513': ('1996-05-13', 'office_reduction_announced_prospective'),
    'za_zuma_deputy_president_appointment_effective_19990617': ('1999-06-17', 'appointment_effective'),
    'za_zuma_deputy_president_appointment_gazetted_19990702': ('1999-07-02', 'appointment_notice_published'),
    'za_zuma_deputy_president_leader_of_government_business_announced_19990628': ('1999-06-28', 'in_office_continuation_attestation'),
    'za_presidency_profile_zuma_appointed_deputy_president_1999': (None, 'retrospective_profile_statement'),
    'za_presidency_list_deputy_president_mbeki_19940513_19990616': (None, 'retrospective_list_row'),
    'za_presidency_list_deputy_president_de_klerk_19940513_19960630': (None, 'retrospective_list_row'),
    'za_presidency_list_deputy_president_zuma_19990617_20050614': (None, 'retrospective_list_row'),
    'za_presidency_list_deputy_president_mlambo_ngcuka_20050623_20080924': (None, 'retrospective_list_row'),
    'za_presidency_list_deputy_president_mbete_20080905': (None, 'retrospective_list_row'),
    'za_mbeki_announces_release_of_zuma_joint_sitting_20050614': ('2005-06-14', 'release_from_office_announced'),
    'za_mbeki_releases_zuma_as_deputy_president_20050614': ('2005-06-14', 'release_from_office_announced'),
    'za_zuma_departure_void_in_executive_20050614': ('2005-06-14', 'departure_void_stated'),
    'za_zuma_accepts_release_20050614': ('2005-06-14', 'release_accepted'),
    'za_zuma_styled_former_deputy_president_20050620': ('2005-06-20', 'former_holder_styled'),
    'za_mlambo_ngcuka_appointment_decision_announced_20050622': ('2005-06-22', 'appointment_decision_announced'),
    'za_mlambo_ngcuka_appointment_noted_by_assembly_20050622': ('2005-06-22', 'appointment_noted_by_resolution'),
    'za_mlambo_ngcuka_swearing_in_scheduled_20050623': ('2005-06-23', 'oath_scheduled_prospective'),
    'za_mlambo_ngcuka_assumed_duties_20050622': ('2005-06-22', 'assumption_of_office'),
    'za_mlambo_ngcuka_appointed_per_profile_20050621': ('2005-06-21', 'appointment_stated'),
    'za_presidency_profile_mlambo_ngcuka_span_20050622_20080923': (None, 'retrospective_term_span'),
    'za_deputy_president_resignation_accepted_20080923': ('2008-09-23', 'resignation_accepted'),
    'za_deputy_president_resignation_effect_by_reference_20080923': ('2008-09-23', 'resignation_effect_stated_by_reference'),
    'za_mlambo_ngcuka_announces_resignation_20080923': ('2008-09-23', 'resignation_announced'),
    'za_motlanthe_intends_appointing_mbete_deputy_president_20080925': ('2008-09-25', 'appointment_intention_announced'),
    'za_mbete_vacates_speakership_for_deputy_presidency_20080925': ('2008-09-25', 'other_office_vacated_for_appointment'),
    'za_mbete_designation_congratulated_in_house_20080925': ('2008-09-25', 'members_remarks_on_designation'),
    'za_mbete_deputy_president_cabinet_list_20080925': ('2008-09-25', 'cabinet_list_as_announced'),
    'za_mbete_deputy_president_logb_announced_20081021': ('2008-10-21', 'in_office_attestation'),
    'za_mlambo_ngcuka_styled_former_deputy_president_20081021': ('2008-10-21', 'former_holder_styled'),
    'za_presidency_profile_mbete_deputy_since_september_2008': (None, 'profile_current_position_month_only'),
    'za_motlanthe_deputy_president_announced_20090510': ('2009-05-10', 'appointment_announcement'),
    'za_motlanthe_sworn_in_deputy_president_caption_20090511': ('2009-05-11', 'oath_of_office'),
    'za_govza_directory_motlanthe_deputy_span_20090511_20140525': (None, 'retrospective_term_span'),
    'za_ramaphosa_dp_appointment_announced_20140525': ('2014-05-25', 'appointment_announcement'),
    'za_ramaphosa_styled_dp_20140525': ('2014-05-25', 'styled_in_appointment_announcement'),
    'za_ramaphosa_deputy_president_logb_letter_20140530': ('2014-05-30', 'in_office_attestation'),
    'za_ramaphosa_logb_designation_announced_minutes_20140618': ('2014-06-18', 'in_office_continuation_attestation'),
    'za_ramaphosa_dp_logb_designation_announced_20140618': ('2014-06-18', 'in_office_continuation_attestation'),
    'za_ramaphosa_dp_appointment_communicated_na_20140716': ('2014-07-16', 'appointment_communicated_to_assembly'),
    'za_presidency_profile_ramaphosa_dp_appointed_20140525': ('2014-05-25', 'appointment_stated'),
    'za_mabuza_dp_appointment_intended_20180226': ('2018-02-26', 'appointment_intention_announced'),
    'za_mabuza_assembly_seat_filled_20180226': ('2018-02-26', 'assembly_seat_filled'),
    'za_mabuza_dp_appointed_20180226': ('2018-02-26', 'appointment_stated'),
    'za_mabuza_dp_swearing_in_scheduled_20180227': ('2018-02-27', 'oath_scheduled_prospective'),
    'za_mabuza_deputy_president_logb_letter_20180301': ('2018-03-01', 'in_office_continuation_attestation'),
    'za_presidency_profile_mabuza_sworn_in_20180227': ('2018-02-27', 'oath_of_office'),
    'za_presidency_profile_mabuza_sworn_in_20190530': ('2019-05-30', 'oath_of_office'),
    'za_mabuza_dp_appointment_announced_20190529': ('2019-05-29', 'appointment_announcement'),
    'za_ministers_swearing_in_scheduled_20190530': ('2019-05-30', 'context_ministers_only_ceremony_scheduled'),
    'za_mabuza_assembly_oath_20190528': ('2019-05-28', 'assembly_membership_oath'),
    'za_mabuza_styled_dp_parliament_20190606': ('2019-06-06', 'in_office_continuation_attestation'),
    'za_mabuza_deputy_president_logb_letter_20190605': ('2019-06-05', 'in_office_continuation_attestation'),
    'za_mabuza_styled_dp_sona_reply_20230216': ('2023-02-16', 'in_office_continuation_attestation'),
    'za_mabuza_step_down_request_sona_reply_20230216': ('2023-02-16', 'step_down_request_announced'),
    'za_mabuza_assembly_resignation_letter_20230228': ('2023-02-28', 'assembly_seat_resignation_effective'),
    'za_mabuza_resignation_ends_dp_term_20230301': ('2023-03-01', 'term_ended_stated_without_day'),
    'za_mabuza_step_down_request_referenced_presidency_20230216': ('2023-02-16', 'step_down_request_reference_retrospective'),
    'za_deputy_president_successor_to_be_announced_20230301': ('2023-03-01', 'successor_appointment_to_be_announced'),
    'za_mabuza_assembly_resignation_received_20230228': ('2023-02-28', 'assembly_seat_resignation_received'),
    'za_govza_directory_mabuza_dp_span': (None, 'retrospective_term_span'),
    'za_mashatile_dp_appointment_announced_20230306': ('2023-03-06', 'appointment_announcement'),
    'za_mabuza_step_down_request_referenced_cabinet_statement_20230216': ('2023-02-16', 'step_down_request_reference_retrospective'),
    'za_national_executive_swearing_in_scheduled_20230307': ('2023-03-07', 'oath_scheduled_prospective'),
    'za_mashatile_deputy_president_logb_letter_20230308': ('2023-03-08', 'in_office_continuation_attestation'),
    'za_mashatile_dp_sworn_in_20230307': ('2023-03-07', 'oath_of_office'),
    'za_presidency_profile_mashatile_sworn_in_20230307': ('2023-03-07', 'oath_of_office'),
    'za_presidency_profile_mashatile_current_title_at_capture': (None, 'current_title_at_capture'),
    'za_mashatile_dp_reappointment_announced_20240630': ('2024-06-30', 'appointment_announcement'),
    'za_national_executive_swearing_in_scheduled_20240701': ('2024-07-01', 'oath_scheduled_prospective'),
    'za_mashatile_deputy_president_logb_letter_20240704': ('2024-07-04', 'in_office_attestation'),
    'za_mashatile_styled_dp_20240718': ('2024-07-18', 'in_office_continuation_attestation'),
    'za_govza_directory_mashatile_dp_spans': (None, 'retrospective_term_span'),
    'za_mashatile_dp_in_office_20260830': ('2026-08-30', 'in_office_attestation'),
}
SURNAMES = {'F. W. de Klerk': 'de Klerk', 'Thabo Mbeki': 'Mbeki', 'Jacob Zuma': 'Zuma',
            'Phumzile Mlambo-Ngcuka': 'Mlambo-Ngcuka', 'Baleka Mbete': 'Mbete', 'Kgalema Motlanthe': 'Motlanthe',
            'Cyril Ramaphosa': 'Ramaphosa', 'David Mabuza': 'Mabuza', 'Paul Mashatile': 'Mashatile'}
# Rows whose source names nobody.
UNNAMED = {'za_executive_deputy_presidents_consulted_on_cabinet_19940603',
           'za_single_executive_deputy_president_and_vacancies_from_19960701_announced_19960513',
           'za_deputy_president_resignation_accepted_20080923', 'za_deputy_president_resignation_effect_by_reference_20080923',
           'za_ministers_swearing_in_scheduled_20190530', 'za_deputy_president_successor_to_be_announced_20230301',
           'za_national_executive_swearing_in_scheduled_20230307', 'za_national_executive_swearing_in_scheduled_20240701'}
REVIEW = [f'ZA-DP-{n:02d}' for n in range(1, 11)]
# The CLAUDE-C01-09 presidency holders and the CLAUDE-C01-16 ANC holders, unchanged: (name, attested_on, from, until).
PRESIDENT_HOLDERS = [
    ('Nelson Mandela', '1994-05-10', None, '1999-06-16'), ('Thabo Mbeki', '1999-06-16', None, None),
    ('Thabo Mbeki', '2004-04-27', None, '2008-09-25'), ('Kgalema Motlanthe', '2008-09-25', None, None),
    ('Jacob Zuma', '2009-05-09', None, None), ('Jacob Zuma', None, '2014-05-24', '2018-02-14'),
    ('Cyril Ramaphosa', '2018-02-15', None, None), ('Cyril Ramaphosa', '2019-05-25', None, None),
    ('Cyril Ramaphosa', '2024-06-14', None, None), ('Cyril Ramaphosa', '2024-06-19', None, None)]
STATE_PRESIDENT_HOLDERS = [('F. W. de Klerk', '1990-02-02', None, None)]
ANC_HOLDERS = [
    ('Oliver Tambo', '1990-01-08', None, None), ('Nelson Mandela', '1991-07-18', None, None),
    ('Nelson Mandela', '1994-12-22', None, None), ('Thabo Mbeki', '1997-12-20', None, None),
    ('Thabo Mbeki', '2002-12-20', None, None), ('Jacob Zuma', '2007-12-20', None, None),
    ('Jacob Zuma', '2012-12-20', None, None), ('Cyril Ramaphosa', '2017-12-20', None, None),
    ('Cyril Ramaphosa', '2023-01-08', None, None), ('Cyril Ramaphosa', '2026-05-15', None, None)]
# Secondary leads and unimported pages that must never be a source.
LEAD_URL_MARKERS = ('sanews', 'news24', 'wikipedia', 'sahistory', 'britannica', 'dirco.gov.za', 'gcis.gov.za',
                    'former-principals', 'swearing-ceremony-new-national-executive', 'statement-resignation-deputy-president',
                    'anc.org.za', 'mbek0614', 'zuma0614', '09051016451001', '09051017051001', '09051210451001',
                    '08092610451002', 'pr10081111', '20080930232338', '08092314451001', '940524_sona', '92089_1',
                    '90959_1', '990723444p1003', 'eulogy-former-deputy-president', '960513_0w531', 'no-17332',
                    'no-15771', '50604_1', '50510_1', '1154bc2c', '28af2ecb', '20260831043754', 'NA-Procedural-Devs/30',
                    'quest_inte')
# Page shapes a server can generate per request or that grow over time (search, API, listings, cache-busting queries).
PER_REQUEST_URL = re.compile(r'docsjson|/api/|symbol/access|cdx/search|/search[/?]|[?&]search|[?&]s=|[?&]_=|[?&]cb=|'
                             r'nocache|cachebust|page-range|download\.php|wp-json', re.I)
# Dossier claim ids renamed or dropped per the checks, and a synthetic period key: none may remain.
STALE = ('za_presidency_list_deputy_zuma_19990617_20050614', 'za_presidency_list_deputy_mlambo_ngcuka_20050623_20080924',
         'za_presidency_list_deputy_mbete_20080905', 'za_mbete_styled_deputy_president_in_house_20080925',
         'za_national_executive_swearing_in_scheduled_20190530', 'za_mabuza_assembly_resignation_received_effective_20230228',
         'za_deputy_presidency_vacancy_20230301', 'za_mabuza_step_down_request_announced_20230216',
         'za_mabuza_step_down_recalled_20230306', 'za_presidency_profile_mashatile_current_20260611', 'attested_period')
REPORT = research.RESEARCH / 'south-africa-deputy-presidents-1994-2026-21.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-21.md'


def dp_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or ValueError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    assert len(packet['institutions']) == 1, 'no second presidency institution'
    presidency = packet['institutions'][0]
    assert presidency['id'] == 'za_presidency'
    roles = {r['id']: r for r in presidency['roles']}
    assert list(roles) == [PR, SP, DP], 'the President, State President and Deputy President roles, in that order'
    role = roles[DP]
    assert (role['title'], role['kind']) == (DP_TITLE, 'institutional_office')
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles'] if r['kind'] == 'head_of_state']
    assert heads == [PR, SP], 'the Deputy President is never a head-of-state role'
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    for h in holders:
        assert 'Acting' not in h['name'] and 'acting' not in h['name'], h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_HOLDER_DATE, h['name']
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        expected = []
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'], cid
            assert claims[cid]['attested_on'] == (h['attested_on'] or h['from']), cid
            if claim_source[cid] not in expected:
                expected.append(claim_source[cid])
        assert h['sources'] == expected, h['name']
    starts = [(h['name'], h['from']) for h in holders if h['from']]
    ends = [(h['name'], h['until']) for h in holders if h['until']]
    assert starts == STARTS and ends == ENDS, (starts, ends)
    # The President's, State President's and ANC holders are unchanged, and no role feeds another.
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in roles[PR]['holder_claims']] == PRESIDENT_HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in roles[SP]['holder_claims']] == STATE_PRESIDENT_HOLDERS
    anc, = [o for o in packet['organizations'] if o['id'] == ANC_ID]
    assert [r['id'] for r in anc['roles']] == ['za_anc_president']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in anc['roles'][0]['holder_claims']] == ANC_HOLDERS
    dp_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    dp_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    for other in [roles[PR], roles[SP], anc['roles'][0]]:
        other_claims = set(other['claim_ids']) | {c for h in other['holder_claims'] for c in h['claim_ids']}
        other_sources = set(other['sources']) | {s for h in other['holder_claims'] for s in h['sources']}
        assert not dp_claims & other_claims and not dp_sources & other_sources, other['id']
    assert all(claim_source[c] in dp_sources for c in dp_claims)
    for entry in packet['organizations']:
        assert not set(entry['claim_ids']) & dp_claims and not set(entry['sources']) & dp_sources, entry['id']
        assert not any(r['id'] == DP for r in entry['roles']), entry['id']
    assert set(role['claim_ids']) <= set(presidency['claim_ids']) and set(role['sources']) <= set(presidency['sources'])
    # Undated claims carry no structured date at all.
    for cid in UNDATED:
        assert not {'attested_on', 'attested_period', 'period'} & set(claims[cid]), cid
    # Distinct dated events stay distinct.
    for cid, (day, _kind) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class SouthAfricaDeputyPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'south-africa.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.presidency = cls.packet['institutions'][0]
        cls.role = next(r for r in cls.presidency['roles'] if r['id'] == DP)
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (59, 82))
        order = [s['id'] for s in self.packet['sources']]
        self.assertEqual(len(order), EARLIER_SOURCE_COUNT + len(NEW_SOURCES))
        self.assertEqual(order[EARLIER_SOURCE_COUNT:], NEW_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (53, 7))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(holder_claims), 15)
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual(len(holder_claims) + len(NEVER_HOLDER), len(self.new_claims))
        # The role cites exactly this packet's sources and claims, and the presidency cites them after its own.
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.presidency['claim_ids'][-len(self.new_claims):], self.new_claims)
        self.assertEqual(self.presidency['sources'][-len(NEW_SOURCES):], NEW_SOURCES)
        # At most ten observations, ZA-DP-01..10.
        observations = re.findall(r'^### (ZA-DP-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        for stale in STALE:
            self.assertNotIn(f'"{stale}"', self.raw)

    def test_holders_are_exactly_as_intended(self):
        dp_invariants(self.packet)
        for index, holder in enumerate(self.role['holder_claims']):
            self.assertTrue(holder['note'].startswith('Observed on ' if holder['attested_on'] else 'From '), holder['name'])
            self.assertTrue(holder['uncertainty'], holder['name'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertIn(SURNAMES[holder['name']], self.claims[cid]['text'], cid)
                self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (holder['name'], DP, DP_TITLE), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                self.assertEqual(row['review_observation'], HOLDER_REVIEW[index], cid)
                self.assertRegex(self.claims[cid]['uncertainty'], r'[Dd]ates the holder observation|start', cid)
        # The only start is stated by an appointment_effective claim; no other holder claim carries that kind.
        self.assertEqual([cid for cid, row in self.rows.items() if row['event_kind'] == 'appointment_effective'],
                         ['za_zuma_deputy_president_appointment_effective_19990617'])
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS | BOUNDARY_KINDS, cid)
        for cid in CONTINUATION:
            self.assertEqual(self.rows[cid]['event_kind'], 'in_office_continuation_attestation', cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'continuation', cid)
        # Holder names are normalised; the printed forms stay in the claim text; rows naming nobody carry null.
        self.assertTrue({r['holder_name'] for r in self.rows.values()} <= set(SURNAMES) | {None})
        self.assertEqual({cid for cid, r in self.rows.items() if r['holder_name'] is None}, UNNAMED)
        self.assertTrue(all(r['role_title'] == DP_TITLE for r in self.rows.values()))
        self.assertIn("'my Second Deputy President, the Honourable F.W. de Klerk'",
                      self.claims['za_de_klerk_styled_second_deputy_president_inauguration_19940510']['text'])
        self.assertIn('Mr J. G. Zuma', self.claims['za_zuma_deputy_president_appointment_effective_19990617']['text'])
        self.assertIn('Ms B Mbete', self.claims['za_mbete_deputy_president_logb_announced_20081021']['text'])
        self.assertIn('First Deputy Vice-President',
                      self.claims['za_dfa_mbeki_first_deputy_statement_security_council_19940525']['text'])
        self.assertIn('Mr Shipokosa Paulus Mashatile',
                      self.claims['za_mashatile_deputy_president_logb_letter_20230308']['text'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims = self.claims
        self.assertIn("with effect from 17 June 1999'", claims['za_zuma_deputy_president_appointment_effective_19990617']['text'])
        self.assertIn('not seen', self.role['holder_claims'][2]['uncertainty'])
        # Stated assumption and Assembly note date Mlambo-Ngcuka's observation; the conflicting evidence is named.
        mlambo = self.role['holder_claims'][3]
        for phrase in ('23 June 2005 at 14h00', '21 June 2005', 'No start', 'No end', 'unnamed holder'):
            self.assertIn(phrase, mlambo['uncertainty'])
        self.assertIn("'notes the appointment today", claims['za_mlambo_ngcuka_appointment_noted_by_assembly_20050622']['text'])
        self.assertIn('National Assembly', claims['za_mlambo_ngcuka_appointed_per_profile_20050621']['uncertainty'])
        # Mbete is dated by the Assembly record of 21 October 2008, not by the designation of 25 September 2008.
        mbete = self.role['holder_claims'][4]
        for phrase in ('designation', "'nomination'", "'to become the Deputy President'", 'Leader of Government Business'):
            self.assertIn(phrase, mbete['uncertainty'])
        for word in ("nomination as the Deputy President'", "'this wonderful opportunity she is being given to become the Deputy President'"):
            self.assertIn(word, claims['za_mbete_designation_congratulated_in_house_20080925']['text'])
        # The Assembly-seat resignation's stated day is never the end of the office.
        self.assertIn("as of 28 February 2023'", claims['za_mabuza_assembly_resignation_letter_20230228']['text'])
        self.assertIn("'effective immediately'", claims['za_mabuza_assembly_resignation_received_20230228']['uncertainty'])
        self.assertIn('not the day it ended', claims['za_mabuza_resignation_ends_dp_term_20230301']['uncertainty'])
        self.assertIn('implied, not stated', claims['za_deputy_president_successor_to_be_announced_20230301']['uncertainty'])
        self.assertIn('inference', self.role['holder_claims'][8]['uncertainty'])
        # Oath statements date observations and are never starts.
        for cid, row in self.rows.items():
            if row['event_kind'] == 'oath_of_office':
                self.assertIn('not a start', claims[cid]['uncertainty'], cid)
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
        self.assertIn('live page', claims['za_presidency_profile_mashatile_current_title_at_capture']['uncertainty'])
        scope = self.role['scope_note']
        for phrase in ('only where a source states', 'Oath statements date a holder observation but are never starts',
                       "infer an outgoing holder's last day", 'never boundaries', 'procedure only, never a date',
                       'never a holder of za_president_election', "ANC's party office"):
            self.assertIn(phrase, scope)
        unresolved = self.presidency['coverage']['unresolved']
        self.assertTrue(unresolved[-2].startswith('Deputy Presidents 1994-2026 (CLAUDE-C01-21'))
        self.assertTrue(unresolved[-1].startswith('Keep executive election distinct from party leadership'))
        top = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-21' in u for u in top), 1)
        self.assertTrue(top[-2].startswith('Deputy Presidents 1994-2026 (CLAUDE-C01-21'))
        self.assertTrue(top[-1].startswith('ANC Presidents 1990-2026 (CLAUDE-C01-16'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-25', '2026-09-25'))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('no automatic decoding', extract['fetch_recipe'])
            if sid in GZIP:
                self.assertEqual(extract['source_response_content_encoding'], 'gzip')
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), GZIP[sid])
                self.assertIn('gzip-encoded', source['scope_note'])
                self.assertNotIn('uncompressed body', extract['provenance_note'])
            else:
                self.assertEqual(extract['source_response_content_encoding'], 'identity')
                self.assertNotIn('decoded_response_sha256', extract)
            self.assertRegex(extract['stability_check'], r'again')
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'] and source['scope_note'] and source['publisher'])
            self.assertIn('CLAUDE-C01-21', extract['bounded_scope'])
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
                self.assertEqual((row['observation_id'], row['role_id']), ('za_presidency', DP))
                self.assertNotIn('name', row)
                self.assertIn(row['review_observation'], REVIEW)
                self.assertEqual('printed_range' in row, row['attested_on'] is None)
            url = urlsplit(source['url'])
            if sid in ARCHIVED:
                stamp = ARCHIVED[sid]
                self.assertEqual(url.hostname, 'web.archive.org')
                self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
                self.assertLess(stamp, '20260907')
                self.assertEqual((extract['original_url'], source['original_url']),
                                 (source['original_url'], re.sub(r'^(https?://[^/]+):80/', r'\1/', source['url'].split('id_/', 1)[1])))
                self.assertNotIn(':80', source['original_url'])
                self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
                self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            else:
                self.assertIn(url.hostname, OTHER_HOSTS)
                self.assertNotIn('original_url', source)
                self.assertNotIn('archive_capture_utc', extract)
                if url.hostname == 'archive.gazettes.africa':
                    self.assertIn('Cloudflare', source['scope_note'])
                if url.hostname == 'www.parliament.gov.za':
                    self.assertIn('Last-Modified 2 March 2026', source['scope_note'])
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in NEW_SOURCES}, {'web.archive.org'} | OTHER_HOSTS)
        self.assertEqual({cid: (row['attested_on'], row['event_kind']) for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual({cid for cid, row in self.rows.items() if row['attested_on'] is None}, set(UNDATED))
        # The four re-imported responses are the same url and bytes as the CLAUDE-C01-09 sources, whose extracts keep
        # their own scope and rows unchanged.
        for new, old in REIMPORTS.items():
            old_extract = json.loads((research.ROOT / self.sources[old]['snapshot']['path']).read_text(encoding='utf-8'))
            self.assertEqual(self.sources[new]['url'], self.sources[old]['url'])
            self.assertEqual(RESPONSES[new], (old_extract['source_response_bytes'], old_extract['source_response_sha256']))
            self.assertIn(old, self.sources[new]['scope_note'])
            self.assertIn(self.sources[old]['snapshot']['path'], self.extracts[new]['bounded_scope'])
            self.assertFalse({r['claim_id'] for r in old_extract['rows']} & set(self.new_claims))
            self.assertIn('Head-of-state observations for CLAUDE-C01-09 only', old_extract['bounded_scope'])

    def test_secondary_leads_and_per_request_pages_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST_URL.search(url), sid)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url, sid)
        for source in self.packet['sources']:
            for marker in ('sanews', 'news24', 'wikipedia', 'dirco.gov.za', 'former-principals'):
                self.assertNotIn(marker, source['url'], source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'sahistory', 'britannica', 'news24', 'sanews'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('sanews.gov.za', 'news24.com', 'wikipedia.org', 'dirco.gov.za', 'former-principals', '50604_1',
                       '50510_1', '1154bc2c', '28af2ecb', '20260831043754', 'no-17332', 'NA-Procedural-Devs/30'):
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

        def role(packet, role_id=DP):
            return next(r for r in packet['institutions'][0]['roles'] if r['id'] == role_id)

        def holder(packet, index, role_id=DP):
            return role(packet, role_id)['holder_claims'][index]

        def anc_role(packet):
            return next(o for o in packet['organizations'] if o['id'] == ANC_ID)['roles'][0]

        validator_cases = [
            (lambda p: source(p, 'za_parliament_atc31_20230309')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'za_gazette_20261_19990702')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'za_presidency_history_list_deputy_presidents')['snapshot'].update(path=REPORT.as_posix()),
             'escapes'),
            (lambda p: holder(p, 11).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 2).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'za_mashatile_dp_in_office_20260830').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 2).update(until='1999-06-16'), 'Reversed historical interval'),
            (lambda p: holder(p, 4)['claim_ids'].append('za_motlanthe_sworn_in_deputy_president_caption_20090511'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('za_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor start used as an end (Mbeki)', lambda p: holder(p, 1).update(until='1999-06-17')),
            ('successor observation used as an end (Zuma)', lambda p: holder(p, 2).update(until='2005-06-22')),
            ('successor observation used as an end (Mlambo-Ngcuka)', lambda p: holder(p, 3).update(until='2008-10-21')),
            ('successor oath used as an end (Mbete)', lambda p: holder(p, 4).update(until='2009-05-11')),
            ('successor observation used as an end (Motlanthe)', lambda p: holder(p, 5).update(until='2014-05-30')),
            ('successor oath used as an end (Ramaphosa)', lambda p: holder(p, 6).update(until='2018-02-27')),
            ('successor oath used as an end (Mabuza 2019)', lambda p: holder(p, 8).update(until='2023-03-07')),
            ('reappointment used as an end (Mashatile 2023)', lambda p: holder(p, 9).update(until='2024-07-04')),
            ('retrospective list end used (de Klerk)', lambda p: holder(p, 0).update(until='1996-06-30')),
            ('release announcement used as an end (Zuma)', lambda p: holder(p, 2).update(until='2005-06-14')),
            ('end by reference used (Mlambo-Ngcuka)', lambda p: holder(p, 3).update(until='2008-09-25')),
            ('Assembly-seat resignation used as an end (Mabuza)', lambda p: holder(p, 8).update(until='2023-02-28')),
            ('announcement used as a start (Ramaphosa)', lambda p: holder(p, 6).update({'from': '2014-05-25'})),
            ('announcement used as a start (Mabuza 2019)', lambda p: holder(p, 8).update({'from': '2019-05-29'})),
            ('announcement used as a start (Mashatile 2024)', lambda p: holder(p, 10).update({'from': '2024-06-30'})),
            ('intention used as a start (Mbete)', lambda p: holder(p, 4).update({'from': '2008-09-25'})),
            ('nomination day used as the observation (Mbete)', lambda p: holder(p, 4).update(attested_on='2008-09-25')),
            ('scheduled oath used as a start (Mlambo-Ngcuka)', lambda p: holder(p, 3).update({'from': '2005-06-23'})),
            ('conflicting stated assumption used as a start (Mlambo-Ngcuka)',
             lambda p: holder(p, 3).update({'from': '2005-06-22', 'attested_on': None})),
            ('oath used as a start (Motlanthe)', lambda p: holder(p, 5).update({'from': '2009-05-11'})),
            ('retrospective list start used (Mbeki)', lambda p: holder(p, 1).update({'from': '1994-05-13'})),
            ('publication date used as the start (Zuma)', lambda p: holder(p, 2).update({'from': '1999-07-02'})),
            ('acting service added as a holder', lambda p: role(p)['holder_claims'].insert(2, {
                'name': 'Thabo Mbeki (Acting President)', 'attested_on': '1996-07-11', 'from': None, 'until': None,
                'sources': ['za_infogov_executive_deputy_president_rdp_statement_19960418'],
                'claim_ids': ['za_mbeki_executive_deputy_president_rdp_statement_19960418']})),
            ('interim designation added as a holder', lambda p: role(p)['holder_claims'].insert(5, {
                'name': 'Baleka Mbete', 'attested_on': '2008-09-25', 'from': None, 'until': None,
                'sources': ['za_presidency_cabinet_list_20080925_deputy_president'],
                'claim_ids': ['za_mbete_deputy_president_cabinet_list_20080925']})),
            ('continuation claim cited by a holder', lambda p: (
                holder(p, 1)['claim_ids'].append('za_mbeki_executive_deputy_president_rdp_statement_19960418'),
                holder(p, 1)['sources'].append('za_infogov_executive_deputy_president_rdp_statement_19960418'))),
            ('scheduled oath cited by a holder', lambda p: (
                holder(p, 3)['claim_ids'].append('za_mlambo_ngcuka_swearing_in_scheduled_20050623'),
                holder(p, 3)['sources'].append('za_dfa_presidency_swearing_in_advisory_20050623'))),
            ('Deputy President added to za_president_election', lambda p: role(p, PR)['holder_claims'].append(
                copy.deepcopy(holder(p, 6)))),
            ('Deputy President claim moved onto the President role', lambda p: (
                role(p, PR)['claim_ids'].append('za_ramaphosa_deputy_president_logb_letter_20140530'),
                role(p, PR)['sources'].append('za_parliament_atc6_20140609'))),
            ('President holder added to the Deputy President role', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, 6, PR)))),
            ('ANC holder added to the Deputy President role', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(anc_role(p)['holder_claims'][9]))),
            ('Deputy President role copied to the ANC', lambda p: next(
                o for o in p['organizations'] if o['id'] == ANC_ID)['roles'].append(copy.deepcopy(role(p)))),
            ('Deputy President role moved to a second institution', lambda p: p['institutions'].append(
                dict(copy.deepcopy(p['institutions'][0]), id='za_deputy_presidency', roles=[p['institutions'][0]['roles'].pop()]))),
            ('Deputy President given a head-of-state kind', lambda p: role(p).update(kind='head_of_state')),
            ('second Deputy President role', lambda p: p['institutions'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='za_executive_deputy_president'))),
            ('Deputy President role removed', lambda p: p['institutions'][0]['roles'].pop()),
            ('retrospective list row given a date', lambda p: claim(
                p, 'za_presidency_list_deputy_president_de_klerk_19940513_19960630').update(attested_on='1996-06-30')),
            ('month-only profile given a day', lambda p: claim(
                p, 'za_presidency_profile_mbete_deputy_since_september_2008').update(attested_on='2008-09-01')),
            ('receipt collapsed into the letter', lambda p: claim(
                p, 'za_mabuza_assembly_resignation_received_20230228').update(attested_on='2023-03-01')),
            ('holder order changed', lambda p: role(p)['holder_claims'].reverse()),
        ]
        dp_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                dp_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = dict.fromkeys([f'{n:02d}' for n in range(1, 11)], 'Accepted in part')
        decisions['03'] = decisions['06'] = 'Accepted'
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| ZA-DP-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 10)] + [f'B{n}' for n in range(1, 12)] + [f'C{n}' for n in range(1, 11)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal|Declined)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('e2d1a1ec', '2df4a0a6', 'claude/c01-za-16', 'claude/c01-za-09', 'research-index.json',
                     'test_south_africa_research_s10h.py', 'test_south_africa_heads_of_state_c01_09.py',
                     'test_south_africa_anc_presidents_c01_16.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'south-africa-deputy-presidents-1994-2026-21.md', 'claude/c01-za-21', 'e2d1a1ec',
                     'claude/c01-za-16', 'test_south_africa_deputy_presidents_c01_21.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'SouthAfrica')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['role_observations'], country['source_claims']), (7, 342))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'SouthAfrica'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
