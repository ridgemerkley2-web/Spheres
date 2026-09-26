"""CLAUDE-C01-26: Soviet heads of government and Supreme Soviet chairs, 1990-1991.

The Supreme Soviet's election or approval, the release decree, the Supreme Soviet's agreement, no confidence, the
resignation statement, the suspension, the consent to arrest, the Congress's release and vote, acting or interim service
and presiding arrangements stay separate dated claims. Holders are in-office signatures or the approval, with
attested_on only: no from and no until. The CLAUDE-C01-05 su_president holders and every Russia holder are unchanged."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import parse_qs, urlsplit

import campaign_research as research


# The 31 sources this packet appends, in packet order (document dates ascending).
NEW_SOURCES = [
    'su_sprsfsr_1990_8_art59_19891224',
    'su_sprsfsr_1990_8_art60_19900112',
    'su_garf_exhibit_res_1362i_19900315',
    'su_ips_cm_res_525_19900526',
    'su_ips_cm_res_1177_19901124',
    'su_rada_law_1861i_19901226',
    'su_snd4_steno_vol3',
    'su_rada_res_1870i_19901227',
    'su_ips_cm_res_27_19910110',
    'su_izvestia_19910115_no13_p1',
    'su_km_rasp_943r_19910819',
    'su_sten_vs_bulletin1_19910826',
    'su_sten_vs_bulletin2_19910826',
    'su_ved_1991_35',
    'su_ved_1991_36',
    'su_snd5_bulletin5_19910904',
    'su_kou_rasp_23r_19910904',
    'su_garf_exhibit_law_2392i',
    'su_kou_rasp_25r_19910906',
    'su_ved_1991_37',
    'su_kou_rasp_62r_19911005',
    'su_ved_1991_41',
    'su_mek_rasp_2r_19911010',
    'su_mek_rasp_6r_19911112',
    'su_mgek_rasp_7r_19911115',
    'su_kou_post_53_19911123',
    'su_mgek_post_7_19911128',
    'su_rsfsr_res_2017i_19911212',
    'su_mgek_rasp_23r_19911217',
    'su_kou_rasp_212r_19911219',
    'su_rsfsr_ukaz_299_19911219',
]
# Original response identity recorded in each new extract: (bytes, sha256), downloaded identical by the dossier or the check
# that found the record, by the independent check, and twice by this packet at least 30 minutes apart.
RESPONSES = {
    'su_sprsfsr_1990_8_art59_19891224': (30424, 'd795d3dddc4e07e1e2de030f4750f532ddedbf5fe13f2b52d864c4343891a236'),
    'su_sprsfsr_1990_8_art60_19900112': (31779, '20bd55b2a49687ec3bbec06de07c224d410825f3faac3dcfad86cf054478d65f'),
    'su_garf_exhibit_res_1362i_19900315': (9423, 'e2ab870b81a665e2c6318fd1a4c754a2fa0f347f2337058675398db68713ad95'),
    'su_ips_cm_res_525_19900526': (52511, '889da9f719499ba58720b976c29f378d31a84e8ba9eabe9fded290c5c46c14c9'),
    'su_ips_cm_res_1177_19901124': (52149, '6ed687d8afc0bacf4ad9dad2302123b773070343eec55b4542311ffa935b0348'),
    'su_rada_law_1861i_19901226': (42384, 'c7919fd209edeb9dc9532796d93a7632c54b6beb9ad59149a9aedaa476705b35'),
    'su_snd4_steno_vol3': (12653342, '398d2fa58abd61bfb5e466841d3ba18de6aa0c9170443f73123a4df2a349b897'),
    'su_rada_res_1870i_19901227': (11254, 'a78b05aef067e23e578ce9663505461d33457e4fcde71d9fb971cda26ee6dc62'),
    'su_ips_cm_res_27_19910110': (20265, '09ae8fe92970994ed7e108f0da959a1242fcbe388fe681b59da1cba3f29f123f'),
    'su_izvestia_19910115_no13_p1': (171140, 'db9ec251483c04cfb54e78b02bb8d570115fb57b9586e7b375924b1eccea6847'),
    'su_km_rasp_943r_19910819': (6925, 'b967d16d8b4a5dd6cb145225e991d848b5d2cfefd8be425c8629d575f585e53a'),
    'su_sten_vs_bulletin1_19910826': (2252696, '849b9dae8c5d3580df911819ff466f9e6997e3119c326b1cf638b6e86239776a'),
    'su_sten_vs_bulletin2_19910826': (1530044, '22562b5be16890047e3f62b4cf253ca9daac0fa374ca3870a63bee62723fd5bd'),
    'su_ved_1991_35': (618372, '5a8c0630da633ac68ef5c0514c05107497a9e84cda82541d561bc22013c55a63'),
    'su_ved_1991_36': (1303663, '87abb4c154470c0681cd0867ef63119675b249d323372fdcbfe87a34ab075b6f'),
    'su_snd5_bulletin5_19910904': (1620327, '9ea2c89395488a0214c38345333b504b30542563ebfa1af0646975a6ff227a65'),
    'su_kou_rasp_23r_19910904': (7876, '62e4006ab45af626526d88b4df934cf3241480c121199d4ca064ac545535bedd'),
    'su_garf_exhibit_law_2392i': (14839, 'f6a4f3ea50f21498c838b8ba41b5f865e5b8d71b8f1d7d535a97ac407650c2e9'),
    'su_kou_rasp_25r_19910906': (7340, 'd29cc413bc8a3f32c63b1a0abab6a61ddd22b4056edc59682fe20ec8016629eb'),
    'su_ved_1991_37': (999248, '3e77c34e4da247391f08d019c29bb24d9c94d28eeb6debda1efb95944eb603b0'),
    'su_kou_rasp_62r_19911005': (8464, '00a8eed4e06d7aa612153eee364a450930dfcfe5c42a3571f6baefc3dc0f8e06'),
    'su_ved_1991_41': (556781, '91cf65718b9dceaeb8cb4b4c6ce4eccf337af45afa3cdb897f197c97bdff902e'),
    'su_mek_rasp_2r_19911010': (6843, 'cbf946f6962b6a4572161a20423ce9bc21fdec3c401bcc86a5f5d54ff5e5f378'),
    'su_mek_rasp_6r_19911112': (7853, 'f149257ed9ee134eeae4d9831ba3347bd65b1faccee9022c07d555616d5b11d7'),
    'su_mgek_rasp_7r_19911115': (7150, '244348192719301c1f5d4b42b58d522e5522c612d52628249de0b7b8569a09ac'),
    'su_kou_post_53_19911123': (21408, '222e799e295307c865b9b5de259e474ac8790da12df1f60c71f3a122f5b71431'),
    'su_mgek_post_7_19911128': (7557, '4a9457c3b64dbf64a5709a84a0c7d7d1d84db557ee5204eb99af240f5e76ced8'),
    'su_rsfsr_res_2017i_19911212': (25236, '4bfe7135976067e09fd1bf6d913f8f3a81df9ff87f8bf26df7db039c6ecfc1dc'),
    'su_mgek_rasp_23r_19911217': (6923, '168225eb75ccad8cc8cc0861222dc41bdb6d57428fc73babf198bc4d79accdeb'),
    'su_kou_rasp_212r_19911219': (8608, 'e09fff16bc5515685dd8c9ad93a83e6a585083304e81179e9c797c4795d165ab'),
    'su_rsfsr_ukaz_299_19911219': (24105, 'b73a9d55970f26cacdcb2292adf48506a1c57ea27f573b37d2856f480224bf7e'),
}
# Attached responses in extract order: the portal card, facsimile images, the byte-identical live file of a capture,
# corroborating publications and supplementary images; each downloaded identical twice by this packet.
ATTACHED = {
    'su_sprsfsr_1990_8_art59_19891224': [('facsimile', 37089, '55bda52a960e86dc79b42dff8381a59cfe7db60db28bab2d7004365f5930a892')],
    'su_sprsfsr_1990_8_art60_19900112': [('facsimile', 38198, '3ab7bcd66f9b5cc315d47d011d0d6bb779c4c2f925882f08f2812b62f404f9d2')],
    'su_garf_exhibit_res_1362i_19900315': [('facsimile', 157538, '543c9efb25dedcab267b6505135da07a35a9928bec24b6b74c4593226970aee4')],
    'su_ips_cm_res_525_19900526': [('card', 2936, '93cfc7cb1bbbcf8c9a5caba20b07e71de142bb6d25ae0e59929c85783196abee')],
    'su_snd4_steno_vol3': [('live', 12653342, '398d2fa58abd61bfb5e466841d3ba18de6aa0c9170443f73123a4df2a349b897')],
    'su_izvestia_19910115_no13_p1': [('supplementary', 161461, 'cf0204c6c9431629553ca7cc1e630022efb16f403a2d26a2483793802acbbe88')],
    'su_ved_1991_35': [('live', 618372, '5a8c0630da633ac68ef5c0514c05107497a9e84cda82541d561bc22013c55a63')],
    'su_ved_1991_36': [('live', 1303663, '87abb4c154470c0681cd0867ef63119675b249d323372fdcbfe87a34ab075b6f')],
    'su_snd5_bulletin5_19910904': [('live', 1620327, '9ea2c89395488a0214c38345333b504b30542563ebfa1af0646975a6ff227a65'), ('corroborating', 544323, '236b00bd04599034c2175b4b05492be88c1eaa077cba2b96179575e8221e77a4')],
    'su_garf_exhibit_law_2392i': [('facsimile', 157151, 'c32c9177fd1f5e0b8aceb0e1092414cc41c392a0f8e2419932a225dde0d2a5b7'), ('facsimile', 117260, '044b8701b377fc4c392f00e48dde11406a09d1f4d56c862e9a69260db3af46e7')],
}
# Every claim of this packet: (attested_on or None for a two-day period, event kind, observation, role or None, institution).
EVENTS = {
    'su_sprsfsr_59_ryzhkov_signs_as_chairman_19891224': ('1989-12-24', 'in_office_attestation', 'SU-GOV-01', 'su_government_head', 'su_government'),
    'su_sprsfsr_60_ryzhkov_signs_as_chairman_19900112': ('1990-01-12', 'in_office_attestation', 'SU-GOV-01', 'su_government_head', 'su_government'),
    'su_garf_1362i_gorbachev_elected_president_19900315': ('1990-03-15', 'election_resolution', 'SU-GOV-02', 'su_president', 'su_presidency'),
    'su_garf_1362i_lukyanov_signs_as_chairman_19900315': ('1990-03-15', 'in_office_attestation', 'SU-GOV-02', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ips_cm_525_ryzhkov_signs_as_chairman_19900526': ('1990-05-26', 'in_office_attestation', 'SU-GOV-01', 'su_government_head', 'su_government'),
    'su_ips_cm_1177_ryzhkov_signs_as_chairman_19901124': ('1990-11-24', 'in_office_attestation', 'SU-GOV-04', 'su_government_head', 'su_government'),
    'su_rada_1861i_cabinet_chapter_19901226': ('1990-12-26', 'procedure', 'SU-GOV-03', None, 'su_government'),
    'su_rada_1861i_supreme_soviet_chair_provisions_19901226': ('1990-12-26', 'procedure', 'SU-GOV-03', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_steno4_ryzhkov_heart_attack_reported_19901226': ('1990-12-26', 'illness_reported', 'SU-GOV-04', 'su_government_head', 'su_government'),
    'su_steno4_congress_telegram_to_ryzhkov_19901226': ('1990-12-26', 'congress_message_on_illness', 'SU-GOV-04', 'su_government_head', 'su_government'),
    'su_steno4_pavlov_reports_for_government_19901227': ('1990-12-27', 'report_on_behalf_of_government', 'SU-GOV-04', 'su_government_head', 'su_government'),
    'su_steno4_ryzhkov_health_report_19901227': ('1990-12-27', 'illness_reported', 'SU-GOV-04', 'su_government_head', 'su_government'),
    'su_law_1861i_premier_appointment_release_procedure_19901226': ('1990-12-26', 'procedure', 'SU-GOV-03', 'su_government_head', 'su_government'),
    'su_law_1861i_no_confidence_procedure_19901226': ('1990-12-26', 'procedure', 'SU-GOV-03', None, 'su_government'),
    'su_law_1862i_council_of_ministers_retains_powers_19901226': ('1990-12-26', 'continuation_of_powers', 'SU-GOV-03', None, 'su_government'),
    'su_rada_1870i_lukyanov_signs_as_chairman_19901227': ('1990-12-27', 'in_office_attestation', 'SU-GOV-02', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ips_cm_27_deputy_chairman_signs_19910110': ('1991-01-10', 'government_act_signed_by_deputy', 'SU-GOV-04', 'su_government_head', 'su_government'),
    'su_izv13_president_submission_recited_19910114': ('1991-01-14', 'nomination_recited', 'SU-GOV-05', 'su_government_head', 'su_government'),
    'su_izv13_vs_approves_pavlov_premier_19910114': ('1991-01-14', 'appointment_approval', 'SU-GOV-05', 'su_government_head', 'su_government'),
    'su_izv13_lukyanov_signs_as_vs_chair_19910114': ('1991-01-14', 'in_office_attestation', 'SU-GOV-02', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_km_943r_pavlov_signs_as_premier_19910819': ('1991-08-19', 'in_office_attestation', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_km_943r_general_direction_entrusted_19910819': ('1991-08-19', 'interim_direction_entrusted', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_sten_laptev_presiding_19910826': ('1991-08-26', 'presiding_officer_attestation', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_sten_lukyanov_statement_19910824': ('1991-08-24', 'resignation_statement', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_sten_request_to_president_reference_19910822': ('1991-08-22', 'request_reference', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_sten_statement_read_out_19910826': ('1991-08-26', 'resignation_statement_read_out', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_sten2_laptev_presiding_evening_19910826': ('1991-08-26', 'presiding_officer_attestation', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_sten2_presidium_removal_not_approved_no_quorum_19910826': ('1991-08-26', 'approval_not_voted_no_quorum', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ss_res_2360i_cabinet_confidence_agenda_19910826': ('1991-08-26', 'agenda_adopted', 'SU-GOV-06', None, 'su_government'),
    'su_ss_res_2361i_presidium_removal_reference_19910822': ('1991-08-22', 'removal_from_chairing_reference', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ss_res_2361i_approves_presidium_removal_19910826': ('1991-08-26', 'removal_from_chairing_approved', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ss_res_2361i_chamber_chairs_preside_19910826': ('1991-08-26', 'presiding_duties_assigned', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ss_res_2361i_resignation_statement_reference_19910826': ('1991-08-26', 'resignation_statement_reference', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ss_res_2361i_suspension_19910826': ('1991-08-26', 'suspension_of_duties', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_lukyanov_signs_res_2350i_19910819': ('1991-08-19', 'in_office_attestation', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_lukyanov_signs_presidium_2352i_19910821': ('1991-08-21', 'in_office_attestation', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_lukyanov_signs_presidium_2353i_2354i_19910822': ('1991-08-22', 'in_office_attestation', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ss_presidium_2354i_government_changes_agenda_19910822': ('1991-08-22', 'agenda_proposed', 'SU-GOV-06', None, 'su_government'),
    'su_ukaz_up2443_pavlov_released_19910822': ('1991-08-22', 'release_decree_referred_to_legislature', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_ukaz_up2443_criminal_case_stated_19910822': ('1991-08-22', 'criminal_proceedings_reported', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_ukaz_up2443_referred_to_vs_session_19910822': ('1991-08-22', 'referral_to_legislature', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_ukaz_up2444_gkchp_members_removed_19910822': ('1991-08-22', 'collective_removal_from_posts', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_ukaz_up2461_cabinet_confidence_question_19910824': ('1991-08-24', 'confidence_question_raised', 'SU-GOV-06', None, 'su_government'),
    'su_ukaz_up2461_committee_created_19910824': ('1991-08-24', 'committee_created', 'SU-GOV-07', None, 'su_government'),
    'su_ved35_presidium_under_chamber_chairs_19910821': (None, 'presiding_arrangement_reported', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_vs_2366i_agrees_to_pavlov_release_19910828': ('1991-08-28', 'parliamentary_consent_to_release', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_vs_2367i_no_confidence_in_cabinet_19910828': ('1991-08-28', 'no_confidence', 'SU-GOV-06', None, 'su_government'),
    'su_vs_2367i_committee_until_new_cabinet_19910828': ('1991-08-28', 'committee_created', 'SU-GOV-07', None, 'su_government'),
    'su_ss_res_2368i_lukyanov_arrest_consent_19910829': ('1991-08-29', 'consent_to_arrest', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_vs_2371i_pavlov_named_among_organizers_19910829': ('1991-08-29', 'retrospective_office_reference', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_ved36_lukyanov_styled_chairman_19910828': ('1991-08-28', 'in_office_styling_reported', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ved36_consent_to_pavlov_release_reported_19910828': ('1991-08-28', 'session_record_consent_to_release', 'SU-GOV-06', 'su_government_head', 'su_government'),
    'su_ved36_no_confidence_reported_19910828': ('1991-08-28', 'session_record_no_confidence', 'SU-GOV-06', None, 'su_government'),
    'su_ved36_laptev_closes_session_19910831': ('1991-08-31', 'chamber_chairman_statement_reported', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_snd5_lukyanov_release_vote_19910904': ('1991-09-04', 'congress_vote', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_kou_23r_silayev_signs_as_committee_head_19910904': ('1991-09-04', 'committee_act_signature', 'SU-GOV-07', None, 'su_government'),
    'su_garf_law_2392i_signed_original_19910905': ('1991-09-05', 'procedure', 'SU-GOV-09', None, 'su_supreme_soviet'),
    'su_kou_25r_former_cabinet_apparatus_continues_19910906': ('1991-09-06', 'continuation_of_apparatus', 'SU-GOV-07', None, 'su_government'),
    'su_cpd_res_2389i_lukyanov_released_19910904': ('1991-09-04', 'release_from_office', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_law_2392i_bicameral_structure_19910905': ('1991-09-05', 'procedure', 'SU-GOV-09', None, 'su_supreme_soviet'),
    'su_law_2392i_mek_chairman_procedure_19910905': ('1991-09-05', 'procedure', 'SU-GOV-07', None, 'su_government'),
    'su_law_2392i_transition_and_entry_into_force_19910905': ('1991-09-05', 'procedure', 'SU-GOV-09', None, 'su_supreme_soviet'),
    'su_ukaz_up2528_committee_performs_cabinet_functions_19910906': ('1991-09-06', 'interim_functions_assigned', 'SU-GOV-07', None, 'su_government'),
    'su_ved37_release_reported_19910904': ('1991-09-04', 'release_reported', 'SU-GOV-08', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_ved37_presidium_under_chamber_chairs_19910905': ('1991-09-05', 'presiding_arrangement_reported', 'SU-GOV-09', 'su_supreme_soviet_chair', 'su_supreme_soviet'),
    'su_kou_62r_guest_of_ussr_government_19911005': ('1991-10-05', 'acts_in_name_of_union_government', 'SU-GOV-07', None, 'su_government'),
    'su_ukaz_up2663_new_ss_first_sitting_moved_19911003': ('1991-10-03', 'sitting_scheduled', 'SU-GOV-09', None, 'su_supreme_soviet'),
    'su_mek_2r_silayev_signs_as_chairman_19911010': ('1991-10-10', 'committee_act_signature', 'SU-GOV-07', None, 'su_government'),
    'su_mek_6r_silayev_signs_as_chairman_19911112': ('1991-11-12', 'committee_act_signature', 'SU-GOV-07', None, 'su_government'),
    'su_mgek_7r_silayev_signs_as_chairman_19911115': ('1991-11-15', 'committee_act_signature', 'SU-GOV-07', None, 'su_government'),
    'su_gs13_abolition_of_union_ministries_19911114': ('1991-11-14', 'abolition_decided', 'SU-GOV-10', None, 'su_government'),
    'su_kou_53_announces_gs13_19911123': ('1991-11-23', 'decision_announced', 'SU-GOV-10', None, 'su_government'),
    'su_mgek_7_signed_premier_of_economic_community_19911128': ('1991-11-28', 'committee_act_signature', 'SU-GOV-07', None, 'su_government'),
    'su_rsfsr_2017i_recalls_deputy_groups_19911212': ('1991-12-12', 'delegation_recalled', 'SU-GOV-10', None, 'su_supreme_soviet'),
    'su_mgek_23r_latest_signature_19911217': ('1991-12-17', 'committee_act_signature', 'SU-GOV-10', None, 'su_government'),
    'su_kou_212r_latest_signature_19911219': ('1991-12-19', 'committee_act_signature', 'SU-GOV-10', None, 'su_government'),
    'su_rsfsr_ukaz_299_abolishes_committee_19911219': ('1991-12-19', 'abolition_decreed_by_republic', 'SU-GOV-10', None, 'su_government'),
}
PERIODS = {
    'su_ved35_presidium_under_chamber_chairs_19910821': {'from': '1991-08-21', 'through': '1991-08-22'},
}
# Exact new holders, in chronological order, as (name, attested_on, from, until), and the claims each rests on.
HOLDERS = {
    'su_government_head': [
        ('Николай Иванович Рыжков', '1990-01-12', None, None),
        ('Николай Иванович Рыжков', '1990-11-24', None, None),
        ('Валентин Сергеевич Павлов', '1991-01-14', None, None),
        ('Валентин Сергеевич Павлов', '1991-08-19', None, None),
    ],
    'su_supreme_soviet_chair': [
        ('Анатолий Иванович Лукьянов', '1990-03-15', None, None),
        ('Анатолий Иванович Лукьянов', '1991-08-22', None, None),
    ],
}
HOLDER_CLAIMS = {
    'su_government_head': [['su_sprsfsr_60_ryzhkov_signs_as_chairman_19900112'], ['su_ips_cm_1177_ryzhkov_signs_as_chairman_19901124'], ['su_izv13_vs_approves_pavlov_premier_19910114'], ['su_km_943r_pavlov_signs_as_premier_19910819']],
    'su_supreme_soviet_chair': [['su_garf_1362i_lukyanov_signs_as_chairman_19900315'], ['su_lukyanov_signs_presidium_2353i_2354i_19910822']],
}
# The CLAUDE-C01-05 su_president holders, unchanged.
PRESIDENT_HOLDERS = [
    ('Mikhail Gorbachev', '1990-03-20', None, None, ['su_gorbachev_president_letter_19900320']),
    ('Mikhail Gorbachev', '1991-12-25', None, None, ['su_telcon_gorbachev_title_19911225']),
]
# SHA-256 of every Russia role and holder (name, attested_on, from, until) at the base (CLAUDE-C01-19); russia.json is unchanged.
RUSSIA_HOLDERS_SHA256 = '8575fda3b3069ed498377c3b66df29822c33a47cd4c7ead9fe07e30f7edc27cd'
ARCHIVED = {
    'su_garf_exhibit_res_1362i_19900315': '20191207080438',
    'su_rada_law_1861i_19901226': '20250531192616',
    'su_snd4_steno_vol3': '20250718143642',
    'su_rada_res_1870i_19901227': '20250531195947',
    'su_ved_1991_35': '20211204065955',
    'su_ved_1991_36': '20250820135020',
    'su_snd5_bulletin5_19910904': '20240901125154',
    'su_garf_exhibit_law_2392i': '20260415035300',
}
HTTP_READ = {
    'su_ips_cm_res_525_19900526': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102010231&page=1&rdk=0',
    'su_ips_cm_res_1177_19901124': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102084027&page=1&rdk=0',
    'su_ips_cm_res_27_19910110': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102010316&page=1&rdk=0',
    'su_km_rasp_943r_19910819': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012314&page=1&rdk=0',
    'su_kou_rasp_23r_19910904': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012407&page=1&rdk=0',
    'su_kou_rasp_25r_19910906': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012430&page=1&rdk=0',
    'su_kou_rasp_62r_19911005': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102012665&page=1&rdk=0',
    'su_mek_rasp_2r_19911010': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102119002&page=1&rdk=0',
    'su_mek_rasp_6r_19911112': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102119005&page=1&rdk=0',
    'su_mgek_rasp_7r_19911115': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013103&page=1&rdk=0',
    'su_kou_post_53_19911123': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013213&page=1&rdk=0',
    'su_mgek_post_7_19911128': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013298&page=1&rdk=0',
    'su_rsfsr_res_2017i_19911212': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013526&page=1&rdk=0',
    'su_mgek_rasp_23r_19911217': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013609&page=1&rdk=0',
    'su_kou_rasp_212r_19911219': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102119006&page=1&rdk=0',
    'su_rsfsr_ukaz_299_19911219': 'http://pravo.gov.ru/proxy/ips/?doc_itself=&nd=102013665&page=1&rdk=0',
}
LIVE = {
    'su_sten_vs_bulletin1_19910826': 'https://sten.vs.sssr.su/12/6/1.pdf',
    'su_sten_vs_bulletin2_19910826': 'https://sten.vs.sssr.su/12/6/2.pdf',
    'su_ved_1991_37': 'https://vedomosti.sssr.su/1991/37.pdf',
    'su_ved_1991_41': 'https://vedomosti.sssr.su/1991/41.pdf',
}
IMAGES = ['su_sprsfsr_1990_8_art59_19891224', 'su_sprsfsr_1990_8_art60_19900112']
PDF_SOURCES = ['su_snd4_steno_vol3', 'su_sten_vs_bulletin1_19910826', 'su_sten_vs_bulletin2_19910826', 'su_ved_1991_35', 'su_ved_1991_36', 'su_snd5_bulletin5_19910904', 'su_ved_1991_37', 'su_ved_1991_41']
USER_AGENT = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36'
DECISIONS = {'01': 'Accepted in part', '02': 'Accepted in part', '03': 'Accepted in part', '04': 'Accepted in part', '05': 'Accepted', '06': 'Accepted in part', '07': 'Accepted in part', '08': 'Accepted', '09': 'Accepted in part', '10': 'Accepted in part'}

GOV, GOV_ROLE, SS, SS_ROLE, PRES_ROLE = 'su_government', 'su_government_head', 'su_supreme_soviet', 'su_supreme_soviet_chair', 'su_president'
GOV_TITLE = 'Председатель Совета Министров СССР / Премьер-министр СССР — Head of the Union government'
GOV_NAME = 'Совет Министров СССР / Кабинет Министров СССР — Council of Ministers of the USSR (from 1991 the Cabinet of Ministers of the USSR)'
TITLES = {GOV_ROLE: GOV_TITLE, SS_ROLE: 'Chairman of the USSR Supreme Soviet', PRES_ROLE: 'President of the USSR', None: None}
RYZH, PAVL, LUK, GORB = 'Николай Иванович Рыжков', 'Валентин Сергеевич Павлов', 'Анатолий Иванович Лукьянов', 'Михаил Сергеевич Горбачев'
NAMES = {RYZH, PAVL, LUK, GORB}
# The vocabulary of event kinds. Only the two holder kinds may feed a holder observation, and only the claims pinned in
# HOLDER_CLAIMS do: earlier, intermediate and pre-period signatures stay role claims.
HOLDER_KINDS = {'in_office_attestation', 'appointment_approval'}
ACTING_KINDS = {'interim_direction_entrusted', 'government_act_signed_by_deputy',
                'presiding_officer_attestation', 'presiding_arrangement_reported', 'presiding_duties_assigned',
                'chamber_chairman_statement_reported'}
END_KINDS = {'release_decree_referred_to_legislature', 'parliamentary_consent_to_release', 'session_record_consent_to_release',
             'collective_removal_from_posts', 'removal_from_chairing_reference', 'removal_from_chairing_approved',
             'approval_not_voted_no_quorum', 'suspension_of_duties', 'resignation_statement', 'resignation_statement_reference',
             'resignation_statement_read_out', 'request_reference', 'consent_to_arrest', 'release_from_office', 'release_reported',
             'congress_vote', 'retrospective_office_reference', 'in_office_styling_reported', 'criminal_proceedings_reported',
             'referral_to_legislature', 'illness_reported', 'congress_message_on_illness', 'report_on_behalf_of_government'}
BODY_KINDS = {'procedure', 'continuation_of_powers', 'agenda_adopted', 'agenda_proposed', 'confidence_question_raised',
              'committee_created', 'no_confidence', 'session_record_no_confidence', 'committee_act_signature',
              'continuation_of_apparatus', 'interim_functions_assigned', 'acts_in_name_of_union_government', 'sitting_scheduled',
              'abolition_decided', 'decision_announced', 'delegation_recalled', 'abolition_decreed_by_republic'}
CONTEXT_KINDS = {'election_resolution', 'nomination_recited'}
VOCABULARY = HOLDER_KINDS | ACTING_KINDS | END_KINDS | BODY_KINDS | CONTEXT_KINDS
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in {c for ids in HOLDER_CLAIMS.values() for group in ids for c in group})
# Days a holder must never start or end on: a successor's approval or signature, a release decree, the agreement with it, a
# suspension, a consent to arrest, the Congress's release, an illness report and every other dated claim of this packet.
NEVER_BOUNDARY = {v[0] for v in EVENTS.values() if v[0]} | {'1990-03-14', '1991-12-25', '1991-12-26'}
TEMPTING_ENDS = ('su_steno4_ryzhkov_heart_attack_reported_19901226', 'su_izv13_vs_approves_pavlov_premier_19910114',
                 'su_ukaz_up2443_pavlov_released_19910822', 'su_vs_2366i_agrees_to_pavlov_release_19910828',
                 'su_ss_res_2361i_suspension_19910826', 'su_ss_res_2368i_lukyanov_arrest_consent_19910829',
                 'su_cpd_res_2389i_lukyanov_released_19910904', 'su_snd5_lukyanov_release_vote_19910904',
                 'su_vs_2367i_no_confidence_in_cabinet_19910828', 'su_rsfsr_ukaz_299_abolishes_committee_19911219')
# Dossier identifiers, keys and kinds replaced after the checks (duplicates, caption-only, renamed or split); none may appear.
STALE = ('su_garf_referendum_res_lukyanov_autograph_19901224', 'su_rada_1861i_cabinet_of_ministers_created_19901226',
         'su_rada_1861i_prime_minister_procedure_19901226', 'su_ukaz_up2461_confidence_question_19910824',
         'su_ukaz_up2461_committee_headed_by_silayev_19910824', 'su_ss_res_2367i_cabinet_no_confidence_19910828',
         'su_ss_res_2367i_committee_interim_19910828', 'su_vs_session_report_confidence_item_19910828', 'su_ved_1991_35_scan',
         'su_ved_1991_36_scan', 'su_garf_exhibit_referendum_res_19901224', '"acting_service_designation"',
         '"resignation_request_reference"', '"dismissal"', '"committee_head_designation"', 'supporting_claims', 'holder_name_normalized',
         'role_title_printed', 'Лукьянов Анатолий Иванович', 'Павлов Валентин Сергеевич', 'ГОРБАЧЕВ Михаил Сергеевич')
# Leads (secondary, commercial, crowd-sourced, news, transcriptions, caption-only or signed-URL records): never an identity URL.
LEAD_URL_MARKERS = ('garant', 'wikisource', 'wikipedia', 'aprel.org', 'naukaprava', 'izvestija.sssr.su', 'rusneb', 'prlib',
                    'consultant', 'sten.sr.vs.sssr.su', 'sten.vs.sssr.su/13/', 'sssr.su/1991-12.pdf', 'bigenc', 'cyberleninka',
                    'economics.kiev.ua', '09-32', '/nodes/', 'yandex.ru/archive/catalog/91392bb5-0841-4352-8d7f-55d76c362683/2',
                    '720a0706', 'type=original', 'vedomosti.sssr.su/1991/52', 'vedomosti.sssr.su/1990', 'vedomosti.sssr.su/1991/1/',
                    'nd=102010350', 'nd=102011538', 'nd=102012325', 'nd=102010393')
# URL fragments that mark a response generated per request, a cache-busting query, a signed URL, an unresolved or non-raw
# capture, or a growing search or listing; never allowed in a recorded identity.
VOLATILE_URL = re.compile(r'([?&](cb|_cb|_chk|nocache|exp|sig|token)=|sessid|PHPSESSID|ysclid|/web/\d{4}id_/|/web/\d{14}/|'
                          r'/cdx/|/search|a7date=|firstlast&lstsize=1000)')
REPORT = research.RESEARCH / 'ussr-government-and-supreme-soviet-1990-1991-26.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-26.md'


def roles_of(packet):
    return {r['id']: (e, r) for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles']}


def c26_invariants(ussr, russia):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in ussr['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in ussr['sources'] for c in s['claims']}
    insts = {e['id']: e for e in ussr['institutions']}
    assert list(insts) == ['su_presidency', 'su_congress_peoples_deputies', SS, GOV], list(insts)
    gov = insts[GOV]
    assert (gov['name'], gov['kind'], gov['lifecycle']['status']) == (GOV_NAME, 'executive_institution', 'unknown')
    assert (gov['lifecycle']['from'], gov['lifecycle']['until']) == (None, None)
    assert gov['jurisdiction']['automatic_successor_mapping'] is False and gov['represented_party_ids'] == []
    assert [r['id'] for r in gov['roles']] == [GOV_ROLE]
    assert [r['id'] for r in insts[SS]['roles']] == [SS_ROLE]
    assert (insts[SS]['lifecycle']['from'], insts[SS]['lifecycle']['until']) == (None, None)
    kinds = {}
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                for r in entry['roles']:
                    kinds.setdefault(r['kind'], []).append(r['id'])
    assert kinds['head_of_government'] == ['ru_government_chairman', GOV_ROLE], 'exactly one Union head-of-government role'
    assert kinds['head_of_state'] == ['ru_president', PRES_ROLE]
    roles = roles_of(ussr)
    # The CLAUDE-C01-05 presidency holders and the existing chair holder are unchanged; every Russia holder is unchanged.
    got = [(h['name'], h['attested_on'], h['from'], h['until'], h['claim_ids']) for h in roles[PRES_ROLE][1]['holder_claims']]
    assert got == PRESIDENT_HOLDERS, got
    got = [(h['name'], h.get('attested_on'), h['from'], h['until'], h['claim_ids']) for h in roles[SS_ROLE][1]['holder_claims'][:1]]
    assert got == [('Mikhail Gorbachev (signature: M. Gorbachev)', '1990-03-14', None, None, ['su_gorbachev_chair_signature_19900314'])], got
    ru = [(r['id'], [(h['name'], h.get('attested_on'), h['from'], h['until']) for h in r['holder_claims'] if isinstance(h, dict)])
          for g in ('organizations', 'institutions') for e in russia[g] for r in e['roles']]
    assert hashlib.sha256(json.dumps(ru, ensure_ascii=False).encode('utf-8')).hexdigest() == RUSSIA_HOLDERS_SHA256
    # Exact holders on the two roles, in chronological order; each rests on its own claim, dated that day; no from, no until.
    for role_id in (GOV_ROLE, SS_ROLE):
        role = roles[role_id][1]
        holders = role['holder_claims'][1:] if role_id == SS_ROLE else role['holder_claims']
        assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS[role_id], role_id
        assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS[role_id], role_id
        days = [h['attested_on'] for h in role['holder_claims']]
        assert days == sorted(days), role_id
        for h in holders:
            assert h['from'] is None and h['until'] is None, h['name']
            assert h['name'] in NAMES and 'Acting' not in h['name'] and 'Исполняющ' not in h['name'], h['name']
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert all(cid in role['claim_ids'] for cid in h['claim_ids']), h['name']
            assert claims[h['claim_ids'][0]]['attested_on'] == h['attested_on'], h['name']
            assert h['sources'] == [owner[cid] for cid in h['claim_ids']], h['name']
            assert EVENTS[h['claim_ids'][0]][1] in HOLDER_KINDS and EVENTS[h['claim_ids'][0]][3] == role_id, h['name']
    # Every claim of this packet is cited by exactly its own role or institution and by nothing else in either packet.
    ours = set(EVENTS)
    expected = {GOV_ROLE: [], SS_ROLE: [], PRES_ROLE: [], GOV: [], SS: []}
    for cid, (_, _, _, role_id, inst) in EVENTS.items():
        expected[role_id or inst].append(cid)
    assert roles[GOV_ROLE][1]['claim_ids'] == expected[GOV_ROLE]
    assert roles[SS_ROLE][1]['claim_ids'][1:] == expected[SS_ROLE]
    assert roles[PRES_ROLE][1]['claim_ids'][7:] == expected[PRES_ROLE]
    assert gov['claim_ids'] == expected[GOV]
    assert insts[SS]['claim_ids'][2:] == expected[SS]
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                cited = set(entry['claim_ids'])
                allowed = set(expected.get(entry['id'], []))
                assert not (cited & ours) - allowed, entry['id']
                for r in entry['roles']:
                    cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    assert not (cited & ours) - set(expected.get(r['id'], [])), r['id']
    # Acting, interim and presiding service and the interim committees name no holder.
    for cid, (_, kind, _, role_id, inst) in EVENTS.items():
        if kind in ACTING_KINDS or role_id is None:
            assert not any(cid in h['claim_ids'] for rid in (GOV_ROLE, SS_ROLE) for h in roles[rid][1]['holder_claims']), cid
    # Tempting ends stay dated claims and never become an until.
    for cid in TEMPTING_ENDS:
        assert claims[cid]['attested_on'] in NEVER_BOUNDARY, cid
    for rid in (GOV_ROLE, SS_ROLE, PRES_ROLE):
        for h in roles[rid][1]['holder_claims']:
            assert not {h['from'], h['until']} & NEVER_BOUNDARY, (rid, h['name'])
    # Distinct events stay distinct and in order.
    for earlier, later in (('su_izv13_president_submission_recited_19910114', 'su_izv13_vs_approves_pavlov_premier_19910114'),
                           ('su_ukaz_up2443_pavlov_released_19910822', 'su_vs_2366i_agrees_to_pavlov_release_19910828'),
                           ('su_ss_res_2361i_presidium_removal_reference_19910822', 'su_ss_res_2361i_approves_presidium_removal_19910826'),
                           ('su_ss_res_2361i_suspension_19910826', 'su_ss_res_2368i_lukyanov_arrest_consent_19910829'),
                           ('su_ss_res_2368i_lukyanov_arrest_consent_19910829', 'su_cpd_res_2389i_lukyanov_released_19910904'),
                           ('su_ukaz_up2461_cabinet_confidence_question_19910824', 'su_vs_2367i_no_confidence_in_cabinet_19910828')):
        assert claims[earlier]['attested_on'] <= claims[later]['attested_on'], (earlier, later)
        assert earlier != later
    assert claims['su_snd5_lukyanov_release_vote_19910904']['attested_on'] == claims['su_cpd_res_2389i_lukyanov_released_19910904']['attested_on']
    for cid in ours:
        if claims[cid].get('attested_on'):
            assert claims[cid]['attested_on'] <= research.CUTOFF, cid


class UssrGovernmentSupremeSovietTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8')
        cls.russia_raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.packet, cls.russia = json.loads(cls.raw), json.loads(cls.russia_raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.roles = roles_of(cls.packet)
        cls.gov = next(e for e in cls.packet['institutions'] if e['id'] == GOV)
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'USSR', 'Russia'}, {'USSR': set(), 'Russia': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), (31, 77))
        self.assertEqual([s['id'] for s in self.packet['sources'][10:]], NEW_SOURCES)
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (41, 98, 5, 6))
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], list(EVENTS))
        dates = [self.sources[sid]['document_date'] for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        self.assertEqual({v[1] for v in EVENTS.values()}, VOCABULARY)
        self.assertEqual(len(VOCABULARY), sum(map(len, (HOLDER_KINDS, ACTING_KINDS, END_KINDS, BODY_KINDS, CONTEXT_KINDS))))
        self.assertEqual({v[2] for v in EVENTS.values()}, {f'SU-GOV-{n:02d}' for n in range(1, 11)})
        self.assertEqual(re.findall(r'^### (SU-GOV-\d\d)\b', self.report, re.M), [f'SU-GOV-{n:02d}' for n in range(1, 11)])
        texts = [self.raw] + [(research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8') for sid in NEW_SOURCES]
        for stale in STALE:
            for text in texts:
                self.assertNotIn(stale, text, stale)

    def test_holders_are_exactly_as_intended(self):
        c26_invariants(self.packet, self.russia)
        for role_id in (GOV_ROLE, SS_ROLE):
            holders = self.roles[role_id][1]['holder_claims']
            for h in (holders[1:] if role_id == SS_ROLE else holders):
                self.assertTrue(h['note'] and h['uncertainty'], h['name'])
                self.assertRegex(h['uncertainty'], r'(from is null|no until|No until|no end is set)', h['name'])
                for cid in h['claim_ids']:
                    row = self.rows[cid]
                    self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (h['name'], role_id, TITLES[role_id]), cid)
                    self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
        for cid in NEVER_HOLDER:
            self.assertFalse(any(cid in h['claim_ids'] for rid in (GOV_ROLE, SS_ROLE, PRES_ROLE)
                                 for h in self.roles[rid][1]['holder_claims']), cid)
        # Rows: one canonical name per person with the printed form beside it; named non-holders only in persons_named.
        for cid, row in self.rows.items():
            self.assertNotIn('name', row)
            self.assertIn(row['holder_name'], NAMES | {None}, cid)
            self.assertEqual('printed_name' in row, row['holder_name'] is not None, cid)
            self.assertEqual(row['role_title'], TITLES[row['role_id']], cid)
            kind = row['event_kind']
            if kind in ACTING_KINDS | BODY_KINDS or row['role_id'] is None:
                self.assertIsNone(row['holder_name'], cid)
            if kind in ACTING_KINDS:
                self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)never a holder|claims? only', cid)
        for cid in ('su_steno4_ryzhkov_heart_attack_reported_19901226', 'su_steno4_congress_telegram_to_ryzhkov_19901226',
                    'su_steno4_pavlov_reports_for_government_19901227', 'su_steno4_ryzhkov_health_report_19901227'):
            self.assertEqual(self.rows[cid]['role_title_note'], 'The passage prints no office title for him.')
        self.assertEqual({cid for cid, row in self.rows.items() if 'role_title_note' in row},
                         {'su_steno4_ryzhkov_heart_attack_reported_19901226', 'su_steno4_congress_telegram_to_ryzhkov_19901226',
                          'su_steno4_pavlov_reports_for_government_19901227', 'su_steno4_ryzhkov_health_report_19901227'})
        # Silayev and the interim committees are never a holder and never on the head-of-government role.
        for cid, row in self.rows.items():
            if 'Силаев' in ' '.join(row.get('persons_named', [])) or 'Силаев' in row['text']:
                self.assertIsNone(row['role_id'], cid)
                self.assertIsNone(row['holder_name'], cid)
        self.assertEqual(self.rows['su_km_943r_general_direction_entrusted_19910819']['persons_named'],
                         ['т. Догужиева В. Х. (genitive; initials and surname only)'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        for rid in (GOV_ROLE, SS_ROLE, PRES_ROLE):
            for h in self.roles[rid][1]['holder_claims']:
                self.assertIsNone(h['from'], (rid, h['name']))
                self.assertIsNone(h['until'], (rid, h['name']))
        for cid in TEMPTING_ENDS:
            self.assertRegex(self.claims[cid]['uncertainty'],
                             r'(?i)(no until|not an end|not used as an end|no day on which|states no|not a boundary|no .*end)', cid)
        for cid in ('su_ukaz_up2443_pavlov_released_19910822', 'su_cpd_res_2389i_lukyanov_released_19910904'):
            self.assertIn('an integrator ruling is requested', self.claims[cid]['uncertainty'])
        self.assertIn('no effect clause', self.claims['su_izv13_vs_approves_pavlov_premier_19910114']['uncertainty'])
        role = self.roles[GOV_ROLE][1]
        for phrase in ('no holder has from or until', 'claims only, never holders', 'procedure only, never a date',
                       'earliest and latest reviewed attestation'):
            self.assertIn(phrase, role['scope_note'])
        chair = self.roles[SS_ROLE][1]
        self.assertIn('not acting service as Chairman', chair['scope_note'])
        self.assertIn('creates no Chairman of the Supreme Soviet', chair['scope_note'])
        self.assertEqual([u.split(':')[0] for u in self.gov['coverage']['unresolved']],
                         ['SU-GOV-01', 'SU-GOV-03', 'SU-GOV-04', 'SU-GOV-05', 'SU-GOV-06', 'SU-GOV-07', 'SU-GOV-10', 'Hosting'])
        ss = next(e for e in self.packet['institutions'] if e['id'] == SS)
        self.assertEqual([u.split(':')[0] for u in ss['coverage']['unresolved'][-4:]], ['SU-GOV-02', 'SU-GOV-08', 'SU-GOV-09', 'SU-GOV-10'])
        self.assertEqual(sum('CLAUDE-C01-26' in u for u in self.packet['coverage']['unresolved']), 1)
        pres = next(e for e in self.packet['institutions'] if e['id'] == 'su_presidency')
        self.assertTrue(pres['coverage']['unresolved'][-1].startswith('CLAUDE-C01-26: '))
        # Constitution and statute texts carry procedure kinds only.
        for cid in ('su_rada_1861i_cabinet_chapter_19901226', 'su_rada_1861i_supreme_soviet_chair_provisions_19901226',
                    'su_law_1861i_premier_appointment_release_procedure_19901226', 'su_law_1861i_no_confidence_procedure_19901226',
                    'su_law_2392i_bicameral_structure_19910905', 'su_law_2392i_transition_and_entry_into_force_19910905',
                    'su_law_2392i_mek_chairman_procedure_19910905', 'su_garf_law_2392i_signed_original_19910905'):
            self.assertEqual(EVENTS[cid][1], 'procedure', cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'(?i)procedure only', cid)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual((extract['access_method'], extract['published_date'], extract['document_date']),
                             (source['access_method'], source['published_date'], source['document_date']))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-26', '2026-09-26'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid], sid)
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertRegex(extract['stability_check'], r'This packet: re-downloaded at 2026-09-26T\d\d:\d\d:\d\dZ and again at 2026-09-26T')
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-26 only', extract['bounded_scope'])
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/ussr-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            got = [('card', extract['portal_card_response']['bytes'], extract['portal_card_response']['sha256'])] if 'portal_card_response' in extract else []
            got += [('facsimile', r['bytes'], r['sha256']) for r in extract.get('facsimile_responses', [])]
            got += [('live', extract['live_file_response']['bytes'], extract['live_file_response']['sha256'])] if 'live_file_response' in extract else []
            got += [('corroborating', r['bytes'], r['sha256']) for r in extract.get('corroborating_responses', [])]
            got += [('supplementary', r['bytes'], r['sha256']) for r in extract.get('supplementary_responses', [])]
            self.assertEqual(got, ATTACHED.get(sid, []), sid)
            # Rows repeat the packet claims exactly, in order, keyed by claim_id.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row.get('attested_on'), row.get('period')),
                                 (claim['text'], claim['locator'], claim.get('attested_on'), claim.get('period')))
        self.assertEqual({cid: (row.get('attested_on'), row['event_kind'], row['review_observation'], row['role_id'], row['observation_id'])
                          for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual({cid for cid, row in self.rows.items() if 'period' in row}, set(PERIODS))
        for cid, period in PERIODS.items():
            self.assertEqual((self.claims[cid]['period'], self.claims[cid]['precision']), (period, 'day_range'))
            self.assertNotIn('attested_on', self.claims[cid])
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual((url.scheme, url.hostname), ('https', 'web.archive.org'))
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(source['url'].split('id_/', 1)[1], source['original_url'])
            self.assertEqual((extract['source_response_url'], extract['original_url']), (source['url'], source['original_url']))
            self.assertIn('recorded identity is the uncompressed body', extract['provenance_note'])
            self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
            for row in extract.get('facsimile_responses', []) + extract.get('corroborating_responses', []):
                self.assertRegex(row['url'], r'^https://web\.archive\.org/web/(\d{14})id_/http')
                self.assertLess(re.search(r'/web/(\d{14})id_/', row['url']).group(1), '20260907')
        for sid, http in HTTP_READ.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(source['url'], http.replace('http://', 'https://', 1))
            self.assertEqual(extract['source_response_url'], http)
            self.assertIn('port 443', extract['provenance_note'])
        for sid, url in LIVE.items():
            self.assertEqual((self.sources[sid]['url'], self.extracts[sid]['source_response_url']), (url, url))
            self.assertIn('stored file, not one generated per request', self.extracts[sid]['provenance_note'])
        self.assertEqual(set(ARCHIVED) | set(HTTP_READ) | set(LIVE) | set(IMAGES) | {'su_izvestia_19910115_no13_p1'}, set(NEW_SOURCES))
        for sid in IMAGES:
            self.assertIn('/nodes/', self.extracts[sid]['provenance_note'])
            self.assertIn('locator only', self.extracts[sid]['provenance_note'])
        # The Izvestia page is a dynamic rendering: its request, its limits and its immutable preview are recorded.
        izv = self.extracts['su_izvestia_19910115_no13_p1']
        self.assertIn(USER_AGENT, izv['request_note'])
        self.assertIn('403', izv['request_note'])
        self.assertIn('dynamic HTML rendering', izv['provenance_note'])
        self.assertIn('OlmOZleFDzl7N-_VvEEGR', izv['provenance_note'])
        self.assertNotIn('a765e1e7', json.dumps(izv))
        # PDFs record the pages read; the scans' non-official host is stated on every such source.
        for sid in NEW_SOURCES:
            if self.sources[sid]['source_type'].endswith('_non_official_host'):
                self.assertIn('non-official', self.sources[sid]['publisher'], sid)
        self.assertEqual({sid for sid in NEW_SOURCES if self.extracts[sid]['visual_review']['pdf_pages_one_based']}, set(PDF_SOURCES))

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        urls = []
        for sid in NEW_SOURCES:
            extract = self.extracts[sid]
            urls += [self.sources[sid]['url'], extract['source_response_url'], self.sources[sid].get('original_url') or '']
            if 'portal_card_response' in extract:
                urls.append(extract['portal_card_response']['url'])
            if 'live_file_response' in extract:
                urls.append(extract['live_file_response']['url'])
            for key in ('facsimile_responses', 'corroborating_responses', 'supplementary_responses'):
                urls += [r['url'] for r in extract.get(key, [])]
        urls = [u for u in urls if u]
        for url in urls:
            self.assertIsNone(VOLATILE_URL.search(url), url)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url, (marker, url))
            query = parse_qs(urlsplit(url).query, keep_blank_values=True)
            if 'list_itself' in query:
                self.assertEqual(query['a7type'], ['4'], url)
                self.assertEqual(query['a7from'], query['a7to'], url)
                self.assertIn('a8', query, url)
        # Every vedomosti.sssr.su reference in the packet is a scanned issue PDF, never the HTML transcription.
        for path in re.findall(r'vedomosti\.sssr\.su(/[^"\s]*)?', self.raw):
            self.assertRegex(path, r'^/1991/\d+\.pdf$')
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('base.garant.ru/6334493', 'base.garant.ru/6336275', 'Закон_СССР_от_20.03.1991_№_2033-I', '09-32',
                       'vedomosti.sssr.su/1991/52/', 'aprel.org/d04up2.pdf', 'izvestija.sssr.su/1991/201mv.pdf'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_separation_from_the_presidency_russia_and_other_packets(self):
        for sid in NEW_SOURCES:
            self.assertTrue(sid.startswith('su_'))
            self.assertNotIn(f'"{sid}"', self.russia_raw)
        for cid in EVENTS:
            self.assertTrue(cid.startswith('su_'))
            self.assertNotIn(f'"{cid}"', self.russia_raw)
        self.assertIn('not the RSFSR Council of Ministers', self.gov['jurisdiction']['note'].replace('Not', 'not'))
        self.assertIn('ru_government', self.gov['jurisdiction']['note'])
        # Republic acts about Union bodies are filed here as claims, never as a mapping.
        for cid in ('su_rsfsr_2017i_recalls_deputy_groups_19911212', 'su_rsfsr_ukaz_299_abolishes_committee_19911219'):
            self.assertIsNone(EVENTS[cid][3])
            self.assertIn('republic', self.claims[cid]['uncertainty'])

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'ussr.json').read_bytes()
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

        def role(packet, role_id=GOV_ROLE):
            return roles_of(packet)[role_id][1]

        def holder(packet, index, role_id=GOV_ROLE):
            return role(packet, role_id)['holder_claims'][index]

        validator_cases = [
            (lambda p: source(p, 'su_ips_cm_res_1177_19901124')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'su_ved_1991_36')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 3).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'su_snd5_lukyanov_release_vote_19910904').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 1).update({'from': '1990-11-24', 'until': '1990-05-26'}), 'Reversed historical interval'),
            (lambda p: holder(p, 2)['claim_ids'].append('su_km_943r_pavlov_signs_as_premier_19910819'), 'cited source'),
            (lambda p: source(p, 'su_ips_cm_res_27_19910110').update(url=HTTP_READ['su_ips_cm_res_27_19910110']), 'Invalid public source URL'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def added_holder(name, day, sid, cid, role_id=GOV_ROLE):
            return lambda p: role(p, role_id)['holder_claims'].append({'name': name, 'attested_on': day, 'from': None, 'until': None,
                                                                      'sources': [sid], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'})

        invariant_cases = [
            ("successor's approval used as an end (Ryzhkov)", lambda p: holder(p, 1).update(until='1991-01-14')),
            ("successor's signature used as an end (the 1990-03-14 chair)", lambda p: holder(p, 0, SS_ROLE).update(until='1990-03-15')),
            ('illness report used as an end (Ryzhkov)', lambda p: holder(p, 1).update(until='1990-12-26')),
            ('release decree used as an end (Pavlov)', lambda p: holder(p, 3).update(until='1991-08-22')),
            ('agreement with the release used as an end (Pavlov)', lambda p: holder(p, 3).update(until='1991-08-28')),
            ('suspension used as an end (Lukyanov)', lambda p: holder(p, 2, SS_ROLE).update(until='1991-08-26')),
            ('consent to arrest used as an end (Lukyanov)', lambda p: holder(p, 2, SS_ROLE).update(until='1991-08-29')),
            ("the Congress's release used as an end (Lukyanov)", lambda p: holder(p, 2, SS_ROLE).update(until='1991-09-04')),
            ('approval date used as from (Pavlov)', lambda p: holder(p, 2).update({'from': '1991-01-14', 'attested_on': None})),
            ('election date used as from (Lukyanov)', lambda p: holder(p, 1, SS_ROLE).update({'from': '1990-03-15', 'attested_on': None})),
            ('nomination date used as the attested day', lambda p: holder(p, 2).update(attested_on='1991-01-13')),
            ('interim direction added as a holder (Doguzhiev)', added_holder('Догужиев В. Х.', '1991-08-19', 'su_km_rasp_943r_19910819', 'su_km_943r_general_direction_entrusted_19910819')),
            ('deputy signature added as a holder (Voronin)', added_holder('Л. ВОРОНИН', '1991-01-10', 'su_ips_cm_res_27_19910110', 'su_ips_cm_27_deputy_chairman_signs_19910110')),
            ('interim committee added as a holder (Silayev)', added_holder('Силаев И. С.', '1991-08-28', 'su_ved_1991_36', 'su_vs_2367i_committee_until_new_cabinet_19910828')),
            ('chamber chairman added as a holder (Laptev)', added_holder('И. Д. Лаптев', '1991-08-26', 'su_sten_vs_bulletin1_19910826', 'su_sten_laptev_presiding_19910826', SS_ROLE)),
            ('intermediate signature promoted to a holder (Ryzhkov 525)', added_holder(RYZH, '1990-05-26', 'su_ips_cm_res_525_19900526', 'su_ips_cm_525_ryzhkov_signs_as_chairman_19900526')),
            ('pre-period signature promoted to a holder (Ryzhkov 1989)', lambda p: role(p)['holder_claims'].insert(0, {
                'name': RYZH, 'attested_on': '1989-12-24', 'from': None, 'until': None, 'sources': ['su_sprsfsr_1990_8_art59_19891224'],
                'claim_ids': ['su_sprsfsr_59_ryzhkov_signs_as_chairman_19891224'], 'note': 'x', 'uncertainty': 'x'})),
            ('cross-role holder (a chair holder on the government role)', lambda p: role(p)['holder_claims'].append(copy.deepcopy(holder(p, 1, SS_ROLE)))),
            ('cross-role holder (a Premier on the chair role)', lambda p: role(p, SS_ROLE)['holder_claims'].append(copy.deepcopy(holder(p, 2)))),
            ('cross-institution holder (a Premier on the USSR Presidency)', lambda p: role(p, PRES_ROLE)['holder_claims'].append(copy.deepcopy(holder(p, 2)))),
            ('cross-role claim (the chair signature on the government role)', lambda p: role(p)['claim_ids'].append('su_izv13_lukyanov_signs_as_vs_chair_19910114')),
            ('body claim moved onto the role (the interim committee)', lambda p: role(p)['claim_ids'].append('su_vs_2367i_committee_until_new_cabinet_19910828')),
            ('presidency holder changed', lambda p: holder(p, 0, PRES_ROLE).update(attested_on='1990-03-15')),
            ('existing chair holder changed', lambda p: holder(p, 0, SS_ROLE).update(until='1990-03-15')),
            ('an until on any holder', lambda p: holder(p, 0).update(until='1990-12-31')),
            ('second Union head-of-government role', lambda p: next(e for e in p['institutions'] if e['id'] == SS)['roles'][0].update(kind='head_of_government')),
            ('role removed', lambda p: next(e for e in p['institutions'] if e['id'] == GOV)['roles'].pop()),
            ('institution removed', lambda p: p['institutions'].pop()),
            ('lifecycle given an end', lambda p: next(e for e in p['institutions'] if e['id'] == GOV)['lifecycle'].update(until='1991-12-19')),
            ('successor mapping to Russia', lambda p: next(e for e in p['institutions'] if e['id'] == GOV)['jurisdiction'].update(automatic_successor_mapping=True)),
            ('holder dated by a claim of another day', lambda p: holder(p, 0).update(claim_ids=['su_ips_cm_525_ryzhkov_signs_as_chairman_19900526'], sources=['su_ips_cm_res_525_19900526'])),
            ('events collapsed out of order', lambda p: claim(p, 'su_vs_2366i_agrees_to_pavlov_release_19910828').update(attested_on='1991-08-21')),
        ]
        c26_invariants(self.packet, self.russia)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration)):
                c26_invariants(mutated(change), self.russia)
        # A Russia holder changed, or a Russia role citing a claim of this packet, fails too.
        russia = copy.deepcopy(self.russia)
        next(r for e in russia['institutions'] for r in e['roles'] if r['id'] == 'ru_government_chairman')['holder_claims'][0]['until'] = '1996-08-10'
        with self.assertRaises(AssertionError):
            c26_invariants(self.packet, russia)
        russia = copy.deepcopy(self.russia)
        next(r for e in russia['institutions'] for r in e['roles'] if r['id'] == 'ru_government_chairman')['claim_ids'].append('su_km_943r_pavlov_signs_as_premier_19910819')
        with self.assertRaises(AssertionError):
            c26_invariants(self.packet, russia)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for number, decision in DECISIONS.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| SU-GOV-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 7)] + [f'B{n}' for n in range(1, 14)] + [f'C{n}' for n in range(1, 13)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Declined|Resolved by removal)\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('claude/c01-ru-19', '6f4ef2c2', 'research-index.json', 'test_ussr_research_s10h.py',
                     'test_ussr_russia_transition_c01_05.py', 'test_russia_heads_of_government_c01_19.py', 'test_campaign_census',
                     'vedomosti.sssr.su', 'su_president'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'ussr-government-and-supreme-soviet-1990-1991-26.md', 'claude/c01-su-26', 'e4fec74d',
                     'claude/c01-ru-19', 'test_ussr_government_supreme_soviet_c01_26.py', 'pending Codex acceptance',
                     'test_russia_heads_of_government_c01_19.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'USSR')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations'], country['source_claims'],
                          country['mapping_pending']), (4, 6, 98, 5))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'USSR'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
