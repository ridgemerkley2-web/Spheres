"""CLAUDE-C01-23: French presidents, 1990-2026, keep election rounds, the Constitutional Council's proclamation, its
publication, the investiture, a stated start of term, an outer limit and any stated end apart, state a start or an end only
where a source does, and reach france.json only through the importer's supplement merge."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import import_cnccfp_census as importer


# Original response identity recorded in each extract: (bytes, sha256). Every new source is reproducible.
RESPONSES = {
    'fr_cc_decision_88_60_pdr_19880511': (49425, '342c6945a560f27b9f06ae8aea6630e4f991ac86874693d3f330b1a51f41fa7d'),
    'fr_elysee_mitterrand_investiture_address_19880521': (161665, '346618ddb0ed0e8d288829ea5e7cd9dcf4efa00bda75cab932093461e9b4eb47'),
    'fr_elysee_mitterrand_voeux_corps_diplomatique_19900103': (228525, '150ceede3572e52430b2165f7c76936c0981c95608596929b6e3d602a228be1e'),
    'fr_elysee_mitterrand_kohl_press_conference_19900104': (294994, '3bdd2bff6a363d64ca61cca3eca6179dcd1f87543a65e294de5de45b495e2735'),
    'fr_an_jo_debats_19900827': (1491105, '765b7c0919dcfd1351e030c1f36b3192f2d15c8484bdff258f4d807c1dc031db'),
    'fr_elysee_mitterrand_accepts_balladur_resignation_19950510': (61756, 'cc30316909cfebe65bb0eb97a7122010bd8378c6257355156e776e2df2057804'),
    'fr_elysee_mitterrand_congratulates_chirac_19950507': (59231, 'de59b30cddba5099b3ef0ec43584f7317b9964c8fb876f1dd420bc3e5c2c3fbc'),
    'fr_cc_decision_95_81_pdr_19950512': (58698, '9c2c8dc15ef0a9bb9768664ccd6f489ed5d10c85780311f1f492ca7c7bb962b3'),
    'fr_elysee_mitterrand_letter_19950516': (188485, '093c7d82e51f51ee4c183f566d343e95661bd78ceb0dc394e144a59e063aeba0'),
    'fr_elysee_chirac_address_elysee_19950517': (142308, '034e3bc470008764ff1bcc2d89b46bf409392faf25e9fec19cb8f66d3a1e07d3'),
    'fr_elysee_chirac_armed_forces_message_19950517': (160080, '8937749586ea402755a0bd10f10872f82dce1194811289e6d47638c51ce03c6f'),
    'fr_an_jo_debats_19950519': (44865, 'b4bfce4a6b11eacd4f88d91cae1826f02716d13a053b7337dd81e89e6096754d'),
    'fr_elysee_chirac_reelection_declaration_20020505': (170755, 'e23141dbfdc4bab52e47f216ba6797e7efe15bbc1129544e55002c7874eaf2b7'),
    'fr_cc_proclamation_20020508': (13315, 'c0c0267723ffaf05c5c226785afc5b95a33f4e158229fe191172e592b3c8b9ef'),
    'fr_cc_decision_requests_20020509': (7865, 'bc10c20cd357cd990ab87d4a44f993017c6150766a97efa851f5edb994a36917'),
    'fr_elysee_chirac_investiture_declaration_20020516': (215488, '0bb0cc984eac7edad7f74bd44febc37074bb58522316c45412cd84cad4e53f6a'),
    'fr_cc_decision_2007_141_pdr': (22743, 'd74dbb04768f16ff112162ff4d4a4f67ed40f582dc9b76025268cb8572f2a181'),
    'fr_cc_debre_communication_20070510': (39241, '5acfc124a4ee2361814503287d9b0ee2e8b0c7a78d4b58692d99cab23a4f9828'),
    'fr_jorf_proclamation_20070511': (101812, 'c5371165cac3a7bebfc0823739a7f93a29e98a4107578f8b69380a4f34859918'),
    'fr_cc_debre_investiture_address_20070516': (15108, '891028cb791b4be6dcbc1b3ecd1df721005892512704a83ab4c0a72e0c1c5f56'),
    'fr_cc_investiture_page_2007': (44200, '2778609bc16232bb99bebf37bc57e9adfb3d5611d5ca99597f90dc17ecc95258'),
    'fr_cc_sarkozy_installation_speech_20070516': (231349, '95ba3192fd30e8bdbba949f2d7a2def61d96a2a3b88081f99d2f95ea7a421583'),
    'fr_elysee_investiture_sarkozy': (247821, '6c9c86b3489f6fe42616fcd244c4d6298142afb5649bda5e4725a3a77af021af'),
    'fr_elysee_chirac_declaration_20070515': (193314, 'e8ddca84536fec3a413ab3767e347dd597f8642e2c868753cf09bcf1ce36c87b'),
    'fr_elysee_chirac_biography': (233369, '061da13b6ad6423cbed63c5c16a6799eddcc7ef7fdb6deadf10843d36a1ae570'),
    'fr_cc_note_mai_2012_duree_des_mandats': (48899, '5750873809367f267d29c72c6480d4e19fbe74aaa8668ca4347dad339529c4e1'),
    'fr_cc_decision_2012_154_pdr': (24142, '3bad086522171386e47b7dd634e551001bc34ab395c8d481dc3de5ad79f94dc0'),
    'fr_cc_debre_communication_20120510': (31915, '96892ad19da2387ffd21bc717b06e09ae88a8b3524dc11f3885b3b8ca5e1447f'),
    'fr_jorf_proclamation_20120511': (130955, '2ad59e111fbfcb1e5f49e6f904806a18aca5ec0744ab461b77cdaf8e90ebf6f7'),
    'fr_cc_debre_investiture_address_20120515': (9865, 'f410fddf900598bb2156170d5065ed09315a03228d3fef0d10626b4b0c5d0719'),
    'fr_elysee_investiture_hollande': (221593, 'bfe0451958ce14edc13d01a8463ba2050cff083039e1588991a296dedb5890ad'),
    'fr_elysee_hollande_declaration_20120515': (203678, 'b218534b505f0fbf827489ea83fc239ece0b4fc25b9032124645f131d7d7b029'),
    'fr_jorf_decree_prime_minister_20120515': (95789, 'd91ae102773f02324e2ca81f6f51a251e46ec6943c3d4489d412bffc620878f8'),
    'fr_cc_decision_2017_171_pdr': (46596, '0b5f7d99109fd75453e603d7f05d3cd42aacb68528d35cfc70054b996f0a1a79'),
    'fr_cc_fabius_proclamation_statement_20170510': (18102, '049e18b5e536b14683bd186244c570fdcc3b011d16e450c9a89cc513de092fd3'),
    'fr_jorf_proclamation_20170511': (103631, '0616f940d3009d3f6be425cf41586f0b89bd118d87576e2682f041069f7022fc'),
    'fr_cc_dossier_documentaire_2017': (161471, '72e4bb37912f73bc6bfe3399c04f83657104ee86124cbf7f58dee22ab14c6fce'),
    'fr_cc_fabius_investiture_address_20170514': (48044, '25d5117f463403e2f7b63539e6610667c93a698a2394c17746f0bebd9348c283'),
    'fr_cc_rapport_activite_2017': (4686844, 'ef5a5144037f7d6942bfc66905c22af13b7fd779471f38556071fb47ba67f547'),
    'fr_elysee_investiture_macron': (267860, '240cc9757e95507b49ff1f1da5933fa79228668765be80b703e5f4490eff72e3'),
    'fr_elysee_passation_20170514': (185153, '43ebb93cfe8df94d38d1418803bcefe033438c30a040331504a9d3fe303c90f2'),
    'fr_elysee_hollande_interview_20170510': (562249, 'b8477b9ba22b85e7870c10c5d5a56f58b21228ef5692de1ed96f931098b00fb3'),
    'fr_cc_2022_195_pdr_first_round_declaration': (80571, '2a71fddfb61ec37262619dc0c577318246a73a5a615726516ef6e3a85f05f71d'),
    'fr_cc_2022_197_pdr_proclamation_pdf': (161083, '70ea188084dadcd92fcd28b0a6c7d35f3f0c623f8345e34cc1aed82bf1759643'),
    'fr_jorf_proclamation_20220428': (111042, 'b468ddf92e3b0ef545ad7d143c6cb79fd2a3bbec4a873d1f89df51d589f347df'),
    'fr_elysee_investiture_20220507': (220883, '92512babbe078fda318cace93b49fba10199f4de44251c0bf6bd44b529978e6b'),
    'fr_cc_fabius_investiture_address_20220507': (45900, 'f67529f5cef749b8f9330f3566df2ef91a1a6a3843fe34e48f79341cbe32b6d0'),
    'fr_jorf_dila_opendata_20260904': (2473426, '5d8811542a11d9e5bcad7a14bb6cc218aea77d956323d820680afd148d02fa19'),
    'fr_jorf_dila_opendata_20260905': (4271518, 'e63c469405887a58d37a92b4dd96fbf0922198a630c85bcd840a68f33c61ae49'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (byte-stable id_ form, all made before the 7 September 2026 cutoff): id -> timestamp.
ARCHIVED = {
    'fr_cc_decision_88_60_pdr_19880511': '20210227223450',
    'fr_cc_decision_95_81_pdr_19950512': '20210228124617',
    'fr_elysee_mitterrand_letter_19950516': '20260215194604',
    'fr_cc_proclamation_20020508': '20020602063735',
    'fr_cc_decision_requests_20020509': '20020602063827',
    'fr_jorf_proclamation_20070511': '20250104095151',
    'fr_cc_investiture_page_2007': '20240405132235',
    'fr_elysee_investiture_sarkozy': '20260618045617',
    'fr_elysee_chirac_declaration_20070515': '20260216083233',
    'fr_elysee_chirac_biography': '20260618043407',
    'fr_cc_note_mai_2012_duree_des_mandats': '20250224042741',
    'fr_jorf_proclamation_20120511': '20210227042751',
    'fr_elysee_investiture_hollande': '20260618045558',
    'fr_elysee_hollande_declaration_20120515': '20260618082345',
    'fr_jorf_decree_prime_minister_20120515': '20240919075204',
    'fr_jorf_proclamation_20170511': '20210216045704',
    'fr_cc_fabius_investiture_address_20170514': '20241208030244',
    'fr_elysee_investiture_macron': '20260618043729',
    'fr_elysee_passation_20170514': '20260217204752',
    'fr_elysee_hollande_interview_20170510': '20201024091759',
    'fr_cc_2022_195_pdr_first_round_declaration': '20250210041806',
    'fr_cc_2022_197_pdr_proclamation_pdf': '20230127073042',
    'fr_jorf_proclamation_20220428': '20220428153847',
    'fr_elysee_investiture_20220507': '20260618065523',
    'fr_cc_fabius_investiture_address_20220507': '20220507090054',
}
LIVE_HOSTS = {'www.conseil-constitutionnel.fr', 'www.elysee.fr', 'archives.assemblee-nationale.fr', 'echanges.dila.gouv.fr'}
PDF_PAGES = {
    'fr_elysee_mitterrand_investiture_address_19880521': [1, 2],
    'fr_elysee_mitterrand_voeux_corps_diplomatique_19900103': [1],
    'fr_elysee_mitterrand_kohl_press_conference_19900104': [1, 2],
    'fr_an_jo_debats_19900827': [1, 3],
    'fr_elysee_mitterrand_accepts_balladur_resignation_19950510': [1],
    'fr_elysee_mitterrand_congratulates_chirac_19950507': [1],
    'fr_elysee_chirac_address_elysee_19950517': [1, 2],
    'fr_elysee_chirac_armed_forces_message_19950517': [2],
    'fr_an_jo_debats_19950519': [1, 2],
    'fr_elysee_chirac_reelection_declaration_20020505': [1, 2],
    'fr_elysee_chirac_investiture_declaration_20020516': [1, 2],
    'fr_cc_decision_2007_141_pdr': [1, 3, 4],
    'fr_cc_debre_communication_20070510': [1],
    'fr_cc_debre_investiture_address_20070516': [1],
    'fr_cc_sarkozy_installation_speech_20070516': [1],
    'fr_cc_decision_2012_154_pdr': [1, 4],
    'fr_cc_debre_communication_20120510': [1],
    'fr_cc_debre_investiture_address_20120515': [1],
    'fr_cc_decision_2017_171_pdr': [1, 2, 6, 7],
    'fr_cc_fabius_proclamation_statement_20170510': [1, 2],
    'fr_cc_dossier_documentaire_2017': [1, 8, 11, 12],
    'fr_cc_rapport_activite_2017': [12, 15],
    'fr_cc_2022_197_pdr_proclamation_pdf': [1, 2, 8, 9],
}
INSTITUTION, ROLE = 'fr_presidency', 'fr_president'
# Exact holder observations, in chronological order: (name, attested_on, from, until).
HOLDERS = [
    ('François Mitterrand', '1990-01-04', None, None),
    ('Jacques Chirac', '1995-05-17', None, None),
    ('Jacques Chirac', None, '2002-05-17', None),
    ('Nicolas Sarkozy', None, '2007-05-16', None),
    ('François Hollande', None, '2012-05-15', None),
    ('Emmanuel Macron', None, '2017-05-14', None),
    ('Emmanuel Macron', None, '2022-05-14', None),
]
HOLDER_CLAIMS = [
    ['fr_mitterrand_kohl_press_conference_attestation_19900104', 'fr_mitterrand_signs_convocation_decree_as_president_19900822',
     'fr_mitterrand_message_to_parliament_read_19900827'],
    ['fr_chirac_assumes_office_statement_19950517', 'fr_chirac_armed_forces_message_as_president_19950517'],
    ['fr_cc_2002_states_effect_from_20020508'],
    ['fr_cc_debre_2007_from_this_day_20070516', 'fr_sarkozy_takes_office_statement_20070516'],
    ['fr_cc_debre_2012_from_this_day_20120515', 'fr_cc_note_links_debre_address_hollande_investiture',
     'fr_hollande_invested_statement_20120515', 'fr_jorf_hollande_signs_pm_appointment_decree_20120515'],
    ['fr_cc_fabius_2017_takes_office_now_20170514', 'fr_macron_says_ending_term_began_20170514'],
    ['fr_cc_2022_states_effect_from_20220427', 'fr_cc_fabius_2022_second_mandate_begins_20220507',
     'fr_jorf_decree_signed_macron_20260903', 'fr_jorf_decree_signed_macron_20260904'],
]
# The claim each `from` rests on, and the French words by which it states the day.
FROM_BASIS = {2: ('fr_cc_2002_states_effect_from_20020508', 'à compter du 17 mai 2002 à 0 heure'),
              3: ('fr_cc_debre_2007_from_this_day_20070516', 'A compter de ce jour'),
              4: ('fr_cc_debre_2012_from_this_day_20120515', 'A compter de ce jour, 15 mai 2012'),
              5: ('fr_cc_fabius_2017_takes_office_now_20170514', 'où vous prenez vos hautes fonctions'),
              6: ('fr_cc_2022_states_effect_from_20220427', 'à compter du 14 mai 2022 à 0 heure')}
# Holders whose basis names nobody: identified by a vote total printed both there and in a named decision row.
VOTES = {3: ('18.983.138', 'fr_cc_debre_2007_investiture_ceremony_20070516', 'fr_cc_2007_second_round_20070505_20070506', '18 983 138'),
         4: ('18 000 668', 'fr_cc_debre_2012_investiture_ceremony_20120515', 'fr_cc_2012_second_round_20120505_20120506', '18 000 668'),
         5: ('20 743 128', 'fr_cc_fabius_2017_investiture_ceremony_20170514', 'fr_cc_2017_second_round_20170506_20170507', '20 743 128')}
SURNAMES = {'François Mitterrand': 'mitterrand', 'Jacques Chirac': 'chirac', 'Nicolas Sarkozy': 'sarkozy',
            'François Hollande': 'hollande', 'Emmanuel Macron': 'macron'}
# Every new claim's (attested_on, event_kind, review observation), exactly.
EVENTS = {
    'fr_cc_proclaims_mitterrand_19880511': ('1988-05-11', 'result_proclamation', 'FR-PRES-01'),
    'fr_cc_88_60_states_mandate_effect_19880511': ('1988-05-11', 'stated_start_of_term', 'FR-PRES-01'),
    'fr_mitterrand_investiture_ceremony_19880521': ('1988-05-21', 'investiture_ceremony', 'FR-PRES-01'),
    'fr_mitterrand_assumes_second_septennat_statement_19880521': ('1988-05-21', 'holder_assumption_statement', 'FR-PRES-01'),
    'fr_mitterrand_diplomatic_corps_new_year_19900103': ('1990-01-03', 'in_office_attestation_archive_heading', 'FR-PRES-01'),
    'fr_mitterrand_kohl_press_conference_attestation_19900104': ('1990-01-04', 'in_office_attestation', 'FR-PRES-01'),
    'fr_mitterrand_signs_convocation_decree_as_president_19900822': ('1990-08-22', 'in_office_signature_as_president', 'FR-PRES-01'),
    'fr_mitterrand_message_to_parliament_read_19900827': ('1990-08-27', 'in_office_attestation', 'FR-PRES-01'),
    'fr_mitterrand_accepts_pm_resignation_19950510': ('1995-05-10', 'in_office_act_archive_heading', 'FR-PRES-01'),
    'fr_mitterrand_congratulates_chirac_election_19950507': ('1995-05-07', 'election_acknowledgement', 'FR-PRES-02'),
    'fr_cc_1995_cites_mitterrand_proclamation_19880511': ('1988-05-11', 'proclamation_reference', 'FR-PRES-01'),
    'fr_cc_proclaims_chirac_19950512': ('1995-05-12', 'result_proclamation', 'FR-PRES-02'),
    'fr_mitterrand_cessation_latest_bound_19950512': ('1995-05-12', 'cessation_latest_bound_stated', 'FR-PRES-08'),
    'fr_mitterrand_handover_announced_19950516': ('1995-05-16', 'handover_announced_by_holder', 'FR-PRES-08'),
    'fr_chirac_address_cites_election_day_19950507': ('1995-05-07', 'election_reference', 'FR-PRES-02'),
    'fr_chirac_assumes_office_statement_19950517': ('1995-05-17', 'holder_assumption_statement', 'FR-PRES-02'),
    'fr_chirac_predecessor_departure_statement_19950517': ('1995-05-17', 'predecessor_departure_statement', 'FR-PRES-08'),
    'fr_chirac_armed_forces_message_as_president_19950517': ('1995-05-17', 'in_office_attestation', 'FR-PRES-02'),
    'fr_chirac_letter_to_assembly_as_president_19950518': ('1995-05-18', 'in_office_attestation', 'FR-PRES-02'),
    'fr_chirac_reelection_night_declaration_20020505': ('2002-05-05', 'election_acceptance_statement', 'FR-PRES-03'),
    'fr_cc_2002_cites_chirac_proclamation_19950512': ('1995-05-12', 'proclamation_reference', 'FR-PRES-02'),
    'fr_cc_proclaims_chirac_20020508': ('2002-05-08', 'result_proclamation', 'FR-PRES-03'),
    'fr_cc_2002_states_effect_from_20020508': ('2002-05-08', 'stated_start_of_term', 'FR-PRES-03'),
    'fr_cc_20020509_cites_proclamation_20020508': ('2002-05-08', 'proclamation_reference', 'FR-PRES-03'),
    'fr_chirac_investiture_ceremony_20020516': ('2002-05-16', 'investiture_ceremony', 'FR-PRES-03'),
    'fr_cc_2007_cites_chirac_proclamation_20020508': ('2002-05-08', 'proclamation_reference', 'FR-PRES-03'),
    'fr_cc_2007_second_round_20070505_20070506': (None, 'election_round', 'FR-PRES-04'),
    'fr_cc_2007_proclamation_sarkozy_20070510': ('2007-05-10', 'result_proclamation', 'FR-PRES-04'),
    'fr_cc_2007_chirac_cessation_bound_20070510': ('2007-05-10', 'cessation_latest_bound_stated', 'FR-PRES-08'),
    'fr_cc_2007_results_arrested_session_20070510': ('2007-05-10', 'result_declaration_oral', 'FR-PRES-04'),
    'fr_cc_2007_mandate_begins_by_20070510': ('2007-05-10', 'term_start_upper_bound', 'FR-PRES-04'),
    'fr_jorf_publishes_2007_proclamation_20070511': ('2007-05-11', 'result_publication', 'FR-PRES-04'),
    'fr_cc_debre_2007_investiture_ceremony_20070516': ('2007-05-16', 'investiture_ceremony', 'FR-PRES-04'),
    'fr_cc_debre_2007_from_this_day_20070516': ('2007-05-16', 'start_stated_at_investiture', 'FR-PRES-04'),
    'fr_cc_2007_investiture_ceremony_at_1100_20070516': ('2007-05-16', 'investiture_ceremony', 'FR-PRES-04'),
    'fr_sarkozy_takes_office_statement_20070516': ('2007-05-16', 'holder_assumption_statement', 'FR-PRES-04'),
    'fr_elysee_sarkozy_received_by_chirac_20070516': ('2007-05-16', 'handover_meeting', 'FR-PRES-04'),
    'fr_elysee_sarkozy_investiture_ceremony_20070516': ('2007-05-16', 'investiture_ceremony', 'FR-PRES-04'),
    'fr_elysee_sarkozy_investiture_record_signed_20070516': ('2007-05-16', 'investiture_record_signed', 'FR-PRES-04'),
    'fr_chirac_handover_announced_20070515': ('2007-05-15', 'handover_announced_by_holder', 'FR-PRES-08'),
    'fr_elysee_bio_chirac_left_elysee_20070516': (None, 'retrospective_timeline', 'FR-PRES-08'),
    'fr_cc_note_mitterrand_second_mandate_due_end_19950521': ('1995-05-21', 'computed_term_expiry_retrospective', 'FR-PRES-08'),
    'fr_cc_note_chirac_1995_passation_19950517': ('1995-05-17', 'transfer_of_powers_retrospective', 'FR-PRES-02'),
    'fr_cc_note_chirac_first_mandate_end_20020517': ('2002-05-17', 'computed_term_expiry_retrospective', 'FR-PRES-08'),
    'fr_cc_note_passation_sarkozy_20070516': ('2007-05-16', 'transfer_of_powers_retrospective', 'FR-PRES-04'),
    'fr_cc_note_sarkozy_mandate_expiry_20120516': ('2012-05-16', 'computed_term_expiry_retrospective', 'FR-PRES-08'),
    'fr_cc_note_links_debre_address_hollande_investiture': (None, 'document_link_title', 'FR-PRES-05'),
    'fr_cc_2012_cites_sarkozy_proclamation_20070510': ('2007-05-10', 'proclamation_reference', 'FR-PRES-04'),
    'fr_cc_2012_second_round_20120505_20120506': (None, 'election_round', 'FR-PRES-05'),
    'fr_cc_2012_proclamation_hollande_20120510': ('2012-05-10', 'result_proclamation', 'FR-PRES-05'),
    'fr_cc_2012_sarkozy_cessation_bound_20120510': ('2012-05-10', 'cessation_latest_bound_stated', 'FR-PRES-08'),
    'fr_cc_2012_results_arrested_session_20120510': ('2012-05-10', 'result_declaration_oral', 'FR-PRES-05'),
    'fr_cc_2012_mandate_begins_by_20120510': ('2012-05-10', 'term_start_upper_bound', 'FR-PRES-05'),
    'fr_jorf_publishes_2012_proclamation_20120511': ('2012-05-11', 'result_publication', 'FR-PRES-05'),
    'fr_cc_debre_2012_investiture_ceremony_20120515': ('2012-05-15', 'investiture_ceremony', 'FR-PRES-05'),
    'fr_cc_debre_2012_from_this_day_20120515': ('2012-05-15', 'start_stated_at_investiture', 'FR-PRES-05'),
    'fr_elysee_hollande_received_by_sarkozy_20120515': ('2012-05-15', 'handover_meeting', 'FR-PRES-05'),
    'fr_elysee_hollande_investiture_ceremony_20120515': ('2012-05-15', 'investiture_ceremony', 'FR-PRES-05'),
    'fr_hollande_invested_statement_20120515': ('2012-05-15', 'holder_assumption_statement', 'FR-PRES-05'),
    'fr_jorf_hollande_signs_pm_appointment_decree_20120515': ('2012-05-15', 'in_office_signature_as_president', 'FR-PRES-05'),
    'fr_cc_2017_cites_hollande_proclamation_20120510': ('2012-05-10', 'proclamation_reference', 'FR-PRES-05'),
    'fr_cc_2017_second_round_20170506_20170507': (None, 'election_round', 'FR-PRES-06'),
    'fr_cc_2017_proclamation_macron_20170510': ('2017-05-10', 'result_proclamation', 'FR-PRES-06'),
    'fr_cc_2017_hollande_cessation_bound_20170510': ('2017-05-10', 'cessation_latest_bound_stated', 'FR-PRES-08'),
    'fr_cc_fabius_proclaims_macron_20170510': ('2017-05-10', 'result_declaration_oral', 'FR-PRES-06'),
    'fr_cc_fabius_2017_hollande_cessation_bound_20170510': ('2017-05-10', 'cessation_latest_bound_stated', 'FR-PRES-08'),
    'fr_jorf_publishes_2017_proclamation_20170511': ('2017-05-11', 'result_publication', 'FR-PRES-06'),
    'fr_cc_dossier2017_row_1988': (None, 'retrospective_list_row', 'FR-PRES-01'),
    'fr_cc_dossier2017_row_1995': (None, 'retrospective_list_row', 'FR-PRES-02'),
    'fr_cc_dossier2017_row_2002': (None, 'retrospective_list_row', 'FR-PRES-03'),
    'fr_cc_dossier2017_row_2007': (None, 'retrospective_list_row', 'FR-PRES-04'),
    'fr_cc_dossier2017_row_2012': (None, 'retrospective_list_row', 'FR-PRES-05'),
    'fr_cc_dossier2017_2017_ceremony_pending_20170510': ('2017-05-10', 'ceremony_pending_at_issue', 'FR-PRES-06'),
    'fr_cc_fabius_2017_investiture_ceremony_20170514': ('2017-05-14', 'investiture_ceremony', 'FR-PRES-06'),
    'fr_cc_fabius_2017_takes_office_now_20170514': ('2017-05-14', 'start_stated_at_investiture', 'FR-PRES-06'),
    'fr_cc_rapport2017_officially_invested_20170514': ('2017-05-14', 'investiture_ceremony_retrospective', 'FR-PRES-06'),
    'fr_cc_rapport2017_transfer_of_powers_20170514': ('2017-05-14', 'transfer_of_powers_retrospective', 'FR-PRES-06'),
    'fr_elysee_macron_received_by_hollande_20170514': ('2017-05-14', 'handover_meeting', 'FR-PRES-06'),
    'fr_elysee_macron_2017_investiture_ceremony_20170514': ('2017-05-14', 'investiture_ceremony', 'FR-PRES-06'),
    'fr_elysee_passation_announced_20170514': ('2017-05-14', 'handover_announced', 'FR-PRES-06'),
    'fr_hollande_handover_announced_20170510': ('2017-05-10', 'handover_announced_by_holder', 'FR-PRES-08'),
    'fr_2022_first_round_held_20220410': (None, 'election_round', 'FR-PRES-07'),
    'fr_cc_first_round_results_declared_20220413': ('2022-04-13', 'result_declaration_first_round', 'FR-PRES-07'),
    'fr_cc_2022_cites_macron_proclamation_20170510': ('2017-05-10', 'proclamation_reference', 'FR-PRES-06'),
    'fr_2022_second_round_held_20220424': (None, 'election_round', 'FR-PRES-07'),
    'fr_cc_proclaims_macron_president_20220427': ('2022-04-27', 'result_proclamation', 'FR-PRES-07'),
    'fr_cc_2022_states_effect_from_20220427': ('2022-04-27', 'stated_start_of_term', 'FR-PRES-07'),
    'fr_jorf_publishes_2022_proclamation_20220428': ('2022-04-28', 'result_publication', 'FR-PRES-07'),
    'fr_macron_says_ending_term_began_20170514': ('2017-05-14', 'term_start_retrospective_statement', 'FR-PRES-06'),
    'fr_macron_investiture_second_term_20220507': ('2022-05-07', 'investiture_ceremony', 'FR-PRES-07'),
    'fr_fabius_proclaims_results_at_investiture_20220507': ('2022-05-07', 'ceremonial_proclamation_at_investiture', 'FR-PRES-07'),
    'fr_cc_fabius_2022_investiture_ceremony_20220507': ('2022-05-07', 'investiture_ceremony', 'FR-PRES-07'),
    'fr_cc_fabius_2022_second_mandate_begins_20220507': ('2022-05-07', 'stated_start_of_term', 'FR-PRES-07'),
    'fr_jorf_decree_signed_macron_20260903': ('2026-09-03', 'in_office_signature_as_president', 'FR-PRES-10'),
    'fr_jorf_decree_signed_macron_20260904': ('2026-09-04', 'in_office_signature_as_president', 'FR-PRES-10'),
}
# Election rounds held over two days carry a period, never an attested_on.
PERIODS = {
    'fr_cc_2007_second_round_20070505_20070506': ('2007-05-05', '2007-05-06'),
    'fr_cc_2012_second_round_20120505_20120506': ('2012-05-05', '2012-05-06'),
    'fr_cc_2017_second_round_20170506_20170507': ('2017-05-06', '2017-05-07'),
    'fr_2022_first_round_held_20220410': ('2022-04-09', '2022-04-10'),
    'fr_2022_second_round_held_20220424': ('2022-04-23', '2022-04-24'),
}
# Retrospective table rows, the timeline entry and the link title carry no structured date at all.
UNDATED = ('fr_cc_dossier2017_row_1988', 'fr_cc_dossier2017_row_1995', 'fr_cc_dossier2017_row_2002',
           'fr_cc_dossier2017_row_2007', 'fr_cc_dossier2017_row_2012', 'fr_elysee_bio_chirac_left_elysee_20070516',
           'fr_cc_note_links_debre_address_hollande_investiture')
# Event kinds that may be cited by a holder; every other kind never is.
HOLDER_KINDS = {'stated_start_of_term', 'start_stated_at_investiture', 'holder_assumption_statement', 'in_office_attestation',
                'in_office_signature_as_president', 'term_start_retrospective_statement', 'document_link_title'}
# Kinds that are never a boundary or a holder date: rounds, proclamations, publications, ceremonies, bounds, announcements,
# retrospective statements and heading-only items.
NEVER_KINDS = {'election_round', 'election_acknowledgement', 'election_reference', 'election_acceptance_statement',
               'result_proclamation', 'result_declaration_oral', 'result_declaration_first_round', 'result_publication',
               'proclamation_reference', 'investiture_ceremony', 'ceremonial_proclamation_at_investiture',
               'investiture_record_signed', 'handover_meeting', 'investiture_ceremony_retrospective',
               'cessation_latest_bound_stated', 'term_start_upper_bound', 'handover_announced_by_holder', 'handover_announced',
               'predecessor_departure_statement', 'transfer_of_powers_retrospective', 'computed_term_expiry_retrospective',
               'retrospective_list_row', 'retrospective_timeline', 'ceremony_pending_at_issue',
               'in_office_attestation_archive_heading', 'in_office_act_archive_heading'}
NEVER_HOLDER = tuple(cid for cid, (_, kind, _) in EVENTS.items() if kind in NEVER_KINDS)
# Dates that are never any holder's attested_on, start or end: rounds, proclamations, publications, the ceremonies that
# precede a start, announcements, outer limits, computed expiries, continuation letters, the effect day of a term that began
# before the period and an inferred 2022 end.
NEVER_HOLDER_DATE = {'1988-05-08', '1988-05-11', '1988-05-21', '1990-01-03', '1995-05-07', '1995-05-10', '1995-05-12',
                     '1995-05-16', '1995-05-18', '1995-05-21', '2002-05-05', '2002-05-08', '2002-05-09', '2002-05-16',
                     '2007-05-05', '2007-05-06', '2007-05-10', '2007-05-11', '2007-05-15', '2012-05-05', '2012-05-06',
                     '2012-05-10', '2012-05-11', '2012-05-16', '2017-05-06', '2017-05-07', '2017-05-10', '2017-05-11',
                     '2022-04-09', '2022-04-10', '2022-04-13', '2022-04-23', '2022-04-24', '2022-04-27', '2022-04-28',
                     '2022-05-07', '2022-05-13'}
STARTS = [('Jacques Chirac', '2002-05-17'), ('Nicolas Sarkozy', '2007-05-16'), ('François Hollande', '2012-05-15'),
          ('Emmanuel Macron', '2017-05-14'), ('Emmanuel Macron', '2022-05-14')]
# Leads (news, other renderings, dropped duplicates and gzip-served or wrong captures): never an identity URL.
LEAD_URL_MARKERS = ('wikipedia', 'europe1', 'lapresse', 'senat.fr', 'elysee.fr/recherche', 'elysee-module-8256',
                    'elysee-module-8260', 'elysee-module-27176', 'sp-module-2234', '20260804110044', '20230126145839',
                    'JORFTEXT000045667581', 'JORFTEXT000000465545', '2007141pdr.htm', '2012154PDR.htm', '2017171PDR.htm',
                    '2022197PDR.htm', 'doc_proclam_2007', 'dossier-presse', 'cahier23', 'les-suites-de-la-proclamation',
                    'bilan-du-second-tour', '2007142PDR', '2002111PDR', 'compte-rendu-du-conseil-des-ministres',
                    'les-presidents-de-la-republique', 'l-investiture-des-presidents-de-la-republique',
                    'discours-d-investiture-du-president-de-la-republique', 'sur-les-priorites-de-sa-presidence',
                    '20020508.pdf', 'jorf/jo/2022/04/28/0099', 'node/297', '1989-1990-ordinaire2')
# URL fragments that mark a response generated per request, a cache-busting query, a signed or session URL, an unresolved or
# non-raw capture, or a growing search or listing page; never allowed in a recorded identity.
VOLATILE_URL = re.compile(r'([?&](cb|_|_cb|nocache|token|sig|exp|s|q|query|page)=|jsessionid|PHPSESSID|/recherche|/search|'
                          r'/cdx/|/api/|/web/\d{4}id_/|/web/\d{14}/|/OPENDATA/JORF/?$|Freemium)', re.I)
# Dossier claim ids renamed, merged or dropped per the checks and this packet's reconciliation: none may remain.
STALE = ('fr_mitterrand_mandate_takes_effect_19880521', 'fr_chirac_proclaimed_from_20020517',
         'fr_chirac_proclaimed_from_mitterrand_cessation_19950512', 'fr_mitterrand_announces_handover_19950517',
         'fr_chirac_announces_transfer_of_powers_20070516', 'fr_cc_chirac_cessation_latest_20070516',
         'fr_cc_sarkozy_cessation_latest_20120515', 'fr_cc_hollande_cessation_latest_20170514',
         'fr_cc_2007_term_start_upper_bound_20070510', 'fr_cc_2012_term_start_upper_bound_20120510',
         'fr_cc_2017_takes_office_upper_bound_20170510', 'fr_cc_fabius_takes_office_upper_bound_20170510',
         'fr_elysee_macron_first_mandate_began_20170514', 'fr_elysee_sarkozy_installation_address_20070516',
         'fr_cc_note_sarkozy_mandate_expiry_statement_20120516', 'fr_cc_2012_recital_sarkozy_took_office_20120510',
         'fr_cc_2017_recital_hollande_took_office_20170510', 'fr_cc_proclamation_effective_from_20220514',
         'fr_cc_2022_proclamation_published_jorf_20220428', 'fr_cc_2022_197_pdr_proclamation_html',
         'fr_cc_2007_141_pdr_proclamation', 'fr_cc_2012_154_pdr_proclamation', 'fr_cc_2017_171_pdr_proclamation',
         'fr_elysee_mitterrand_letter_end_of_septennat_19950516', 'fr_cc_dossier2017_row_2007_ceremony_20070516',
         'fr_cc_dossier2017_row_2012_ceremony_20120515', 'attested_period')
REPORT = research.RESEARCH / 'france-presidents-1990-2026-23.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-23.md'
EMPTY_SUPPLEMENT = b'{"sources": [], "institutions": [], "coverage_unresolved": []}'
# SHA-256 of france.json before this packet; an empty supplement must reproduce it exactly.
PRE_SUPPLEMENT_SHA256 = '6ac88df4bfc04728352b644ec419355bec519bb3ee8932dcf1df25b2964b112c'


def presidency_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError or IndexError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert [e['id'] for e in packet['institutions']] == [INSTITUTION], 'exactly one presidency institution'
    presidency = packet['institutions'][0]
    assert presidency['kind'] == 'executive_institution' and presidency['name'] == 'Présidence de la République'
    assert presidency['represented_party_ids'] == [] and presidency['reconciled_organization_id'] is None
    assert presidency['lifecycle']['status'] == 'unknown'
    assert presidency['lifecycle']['from'] is None and presidency['lifecycle']['until'] is None
    assert [(r['id'], r['kind'], r['title']) for r in presidency['roles']] == [(ROLE, 'head_of_state', 'Président de la République')]
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles'] if r['kind'] == 'head_of_state']
    assert heads == [ROLE], 'no other head-of-state role'
    role = presidency['roles'][0]
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    for h in holders:
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_HOLDER_DATE, h['name']
        assert h['until'] is None, 'no source states the day any President left office'
        assert (h['attested_on'] is None) != (h['from'] is None), 'a holder is dated by a start or by an observation'
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'] and cid in presidency['claim_ids'], cid
    assert [(h['name'], h['from']) for h in holders if h['from']] == STARTS
    # Each start is the day its basis claim states; each observation is the day of its claims.
    for index, (cid, words) in FROM_BASIS.items():
        assert holders[index]['claim_ids'][0] == cid, index
        assert words in claims[cid]['text'], cid
        if claims[cid].get('attested_on') not in (None, holders[index]['from']):
            assert claims[cid]['attested_on'] < holders[index]['from'], cid
    chirac_1995 = holders[1]
    assert all(claims[cid]['attested_on'] == chirac_1995['attested_on'] for cid in chirac_1995['claim_ids'])
    # Holders rest on an address that names nobody only with a matching vote total in a named decision row.
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    for index, (printed, ceremony, round_claim, decision_total) in VOTES.items():
        basis = holders[index]['claim_ids'][0]
        assert claim_source[ceremony] == claim_source[basis] and printed in claims[ceremony]['text'], index
        assert decision_total in claims[round_claim]['text'], index
        assert printed in holders[index]['note'], index
    # Two-day rounds keep their period, undated rows carry no structured date, and events keep their own days.
    for cid, (start, end) in PERIODS.items():
        assert claims[cid]['period'] == {'from': start, 'through': end} and 'attested_on' not in claims[cid], cid
    for cid in UNDATED:
        assert 'attested_on' not in claims[cid] and 'period' not in claims[cid], cid
    for cid, (day, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # No organization, and no other entry, cites this institution's claims or sources.
    new_sources, new_claims = set(RESPONSES), set(EVENTS)
    for entry in packet['organizations']:
        assert not set(entry['claim_ids']) & new_claims and not set(entry['sources']) & new_sources, entry['id']
        assert entry['roles'] == [] and entry['represented_party_ids'] == [], entry['id']
    assert set(role['claim_ids']) == new_claims and set(role['sources']) == new_sources


class FrancePresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.csv = (importer.ROOT / importer.RAW).read_bytes()
        cls.raw = (research.ROOT / research.RESEARCH / 'france.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.supplement_bytes = (importer.ROOT / importer.SUPPLEMENT).read_bytes()
        cls.supplement = json.loads(cls.supplement_bytes)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.presidency = cls.packet['institutions'][0]
        cls.role = cls.presidency['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'France'}, {'France': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_packet_is_the_importer_output_and_the_supplement_changes_nothing_else(self):
        built = importer.build(self.csv)
        self.assertEqual(self.raw, json.dumps(built, ensure_ascii=False, indent=2) + '\n')
        self.assertEqual(json.dumps(importer.build(self.csv), ensure_ascii=False), json.dumps(built, ensure_ascii=False))
        data = (research.ROOT / research.RESEARCH / 'france.json').read_bytes()
        self.assertNotIn(b'\r', data)
        # The same build with an empty supplement is the unchanged CNCCFP packet; the real one only adds to it.
        base = importer.build(self.csv, supplement=EMPTY_SUPPLEMENT)
        # Byte for byte the France packet as it was before CLAUDE-C01-23 (the committed file at 8761e45c).
        text = json.dumps(base, ensure_ascii=False, indent=2) + '\n'
        self.assertEqual(hashlib.sha256(text.encode('utf-8')).hexdigest(), PRE_SUPPLEMENT_SHA256)
        self.assertEqual(base['institutions'], [])
        self.assertEqual(self.packet['organizations'], base['organizations'])
        self.assertEqual(len(base['organizations']), 635)
        count = len(base['sources'])
        self.assertEqual(self.packet['sources'][:count], base['sources'])
        self.assertEqual(self.packet['sources'][count:], self.supplement['sources'])
        self.assertEqual(self.packet['institutions'], self.supplement['institutions'])
        unresolved = base['coverage'].pop('unresolved')
        coverage = dict(self.packet['coverage'])
        self.assertEqual(coverage.pop('unresolved'), unresolved + self.supplement['coverage_unresolved'])
        self.assertEqual(coverage, base['coverage'])
        self.assertEqual({k: v for k, v in self.packet.items() if k not in ('sources', 'institutions', 'coverage')},
                         {k: v for k, v in base.items() if k not in ('sources', 'institutions', 'coverage')})
        # The supplement is LF, indent=2, ensure_ascii=False, with exactly the three merged keys.
        self.assertNotIn(b'\r', self.supplement_bytes)
        self.assertEqual(self.supplement_bytes.decode('utf-8'), json.dumps(self.supplement, indent=2, ensure_ascii=False) + '\n')
        self.assertEqual(list(self.supplement), ['sources', 'institutions', 'coverage_unresolved'])
        self.assertEqual(importer.SUPPLEMENT.parent.name, 'supplements')
        self.assertNotIn(importer.SUPPLEMENT.parent, [research.RESEARCH])

    def test_supplement_merge_fails_loudly(self):
        good = json.loads(self.supplement_bytes)

        def encoded(change):
            value = copy.deepcopy(good)
            change(value)
            return json.dumps(value, ensure_ascii=False).encode('utf-8')

        cases = [
            (b'{"sources": [', 'Malformed France supplement'),
            (b'\xff\xfe', 'Malformed France supplement'),
            (b'[]', 'expected exactly'),
            (encoded(lambda v: v.pop('coverage_unresolved')), 'expected exactly'),
            (encoded(lambda v: v.update(organizations=[])), 'expected exactly'),
            (encoded(lambda v: v.update(sources={})), 'entries'),
            (encoded(lambda v: v['sources'][0].pop('claims')), 'entries'),
            (encoded(lambda v: v['sources'][0]['claims'].append('not a claim')), 'entries'),
            (encoded(lambda v: v['coverage_unresolved'].append(' ')), 'entries'),
            (encoded(lambda v: v['institutions'].append({'name': 'no id'})), 'entries'),
            (encoded(lambda v: v['sources'][0].update(id='fr_cnccfp_2024')), 'source id collision'),
            (encoded(lambda v: v['sources'].append(copy.deepcopy(v['sources'][0]))), 'source id collision'),
            (encoded(lambda v: v['sources'][1]['claims'][0].update(id='fr_cnccfp_identity_scope')), 'claim id collision'),
            (encoded(lambda v: v['sources'][1]['claims'][0].update(id=v['sources'][0]['claims'][0]['id'])), 'claim id collision'),
            (encoded(lambda v: v['institutions'][0].update(id='fr_cnccfp_5')), 'entry id collision'),
            (encoded(lambda v: v['sources'][0]['snapshot'].update(path='Cargo.toml')), 'outside research/sources'),
            (encoded(lambda v: v['sources'][0]['snapshot'].update(
                path='docs/campaign-certification/C01/research/sources/../france.json')), 'outside research/sources'),
            (encoded(lambda v: v['sources'][0].pop('snapshot')), 'outside research/sources'),
            (encoded(lambda v: v['sources'][0]['snapshot'].update(
                path='docs/campaign-certification/C01/research/sources/france-missing-review.json')), 'missing or not a file'),
            (encoded(lambda v: v['sources'][0]['snapshot'].update(
                path='docs/campaign-certification/C01/research/sources')), 'missing or not a file'),
        ]
        for raw, message in cases:
            with self.subTest(message=message, raw=raw[:40]), self.assertRaisesRegex(ValueError, message):
                importer.build(self.csv, supplement=raw)
        original = importer.SUPPLEMENT
        try:
            importer.SUPPLEMENT = original.with_name('missing-france.json')
            with self.assertRaisesRegex(ValueError, 'Missing France supplement'):
                importer.build(self.csv)
        finally:
            importer.SUPPLEMENT = original
        # The non-recursive packet glob never reads the supplement as a country packet.
        packets = [p.relative_to(research.ROOT).as_posix() for p in (research.ROOT / research.RESEARCH).glob('*.json')]
        self.assertNotIn(importer.SUPPLEMENT.as_posix(), packets)
        self.assertIn(importer.DEST.as_posix(), packets)

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (49, 95))
        self.assertEqual([s['id'] for s in self.packet['sources']][-49:], NEW_SOURCES)
        self.assertEqual(len(ids['entries']), 636)
        self.assertEqual(ids['roles'], {ROLE})
        self.assertEqual(set(self.new_claims), set(EVENTS))
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual({self.rows[cid]['event_kind'] for cid in holder_claims} - HOLDER_KINDS, set())
        self.assertEqual({kind for _, kind, _ in EVENTS.values()} - NEVER_KINDS - HOLDER_KINDS, set())
        for cid in self.new_claims:
            self.assertIn(cid, self.presidency['claim_ids'])
            self.assertIn(cid, self.role['claim_ids'])
        self.assertEqual(self.presidency['claim_ids'], self.new_claims)
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.presidency['sources'], NEW_SOURCES)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        observations = re.findall(r'^### (FR-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'FR-PRES-{n:02d}' for n in range(1, 11)])
        # FR-PRES-09 (interim presidency) records the absence of any statement, so it owns no row.
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'FR-PRES-{n:02d}' for n in range(1, 11)} - {'FR-PRES-09'})
        for stale in STALE:
            self.assertNotIn(stale, self.raw, stale)
            self.assertNotIn(stale, json.dumps(self.extracts, ensure_ascii=False), stale)

    def test_coverage_hours_preserve_explicit_midnight_effects(self):
        supplement = json.loads(self.supplement_bytes)
        notes = ' '.join(supplement['institutions'][0]['coverage']['unresolved'])
        self.assertNotIn('no source fixes the hour at which any term began', notes)
        self.assertIn('2002 and 2022', notes)
        self.assertIn('0 heure', notes)
        self.assertIn('2007, 2012 and 2017', notes)

    def test_holders_are_exactly_as_intended(self):
        presidency_invariants(self.packet)
        for index, holder in enumerate(self.role['holder_claims']):
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected_sources = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected_sources:
                    expected_sources.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
            surname = SURNAMES[holder['name']]
            named = [cid for cid in holder['claim_ids'] if self.rows[cid]['holder_name']]
            self.assertTrue(named, holder['name'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertEqual(row['role_id'], ROLE, cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                if row['holder_name'] is None:
                    self.assertIn(index, VOTES, cid)
                else:
                    self.assertIn(surname, row['holder_name'].casefold(), cid)
            self.assertIn('No end', holder['uncertainty'], holder['name'])
            if holder['from']:
                self.assertTrue(holder['note'].startswith('From '), holder['name'])
            else:
                self.assertIn('No start', holder['uncertainty'], holder['name'])
        # The holder's own "assumes today" statement never carries a start on its own (1995), and the outer limits,
        # announcements and retrospective statements never carry an end.
        for cid in ('fr_mitterrand_cessation_latest_bound_19950512', 'fr_cc_2007_chirac_cessation_bound_20070510',
                    'fr_cc_2012_sarkozy_cessation_bound_20120510', 'fr_cc_2017_hollande_cessation_bound_20170510',
                    'fr_cc_fabius_2017_hollande_cessation_bound_20170510'):
            self.assertIn("au plus tard", self.claims[cid]['text'], cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'never an `until`', cid)
        for cid in ('fr_mitterrand_handover_announced_19950516', 'fr_chirac_handover_announced_20070515',
                    'fr_hollande_handover_announced_20170510'):
            self.assertIn('never an `until`', self.claims[cid]['uncertainty'], cid)
            self.assertIn("Seul le prononcé fait foi", self.claims[cid]['uncertainty'], cid)
        for cid in ('fr_cc_note_mitterrand_second_mandate_due_end_19950521', 'fr_cc_note_chirac_first_mandate_end_20020517',
                    'fr_cc_note_sarkozy_mandate_expiry_20120516'):
            self.assertIn('etrospective', self.claims[cid]['uncertainty'] + self.sources[self.claim_source[cid]]['scope_note'])
        self.assertIn('never makes a start', self.claims['fr_chirac_assumes_office_statement_19950517']['uncertainty'])
        self.assertIn('never an `until`', self.claims['fr_chirac_predecessor_departure_statement_19950517']['uncertainty'])
        scope = self.role['scope_note']
        for text in ('supports but never makes a start', 'never ends', 'procedure only, never a date',
                     'catalogue labels', 'No interim presidency', 'claims only'):
            self.assertIn(text, scope, text)

    def test_distinct_events_keep_distinct_days(self):
        claims = self.claims
        # Round, proclamation, publication, investiture and start stay separate, each on its own day.
        chains = [
            ('fr_cc_proclaims_chirac_20020508', 'fr_chirac_investiture_ceremony_20020516'),
            ('fr_cc_2007_proclamation_sarkozy_20070510', 'fr_jorf_publishes_2007_proclamation_20070511',
             'fr_cc_debre_2007_from_this_day_20070516'),
            ('fr_cc_2012_proclamation_hollande_20120510', 'fr_jorf_publishes_2012_proclamation_20120511',
             'fr_cc_debre_2012_from_this_day_20120515'),
            ('fr_cc_2017_proclamation_macron_20170510', 'fr_jorf_publishes_2017_proclamation_20170511',
             'fr_cc_fabius_2017_takes_office_now_20170514'),
            ('fr_cc_first_round_results_declared_20220413', 'fr_cc_proclaims_macron_president_20220427',
             'fr_jorf_publishes_2022_proclamation_20220428', 'fr_macron_investiture_second_term_20220507'),
        ]
        for chain in chains:
            days = [claims[cid]['attested_on'] for cid in chain]
            self.assertEqual(days, sorted(set(days)), chain)
        for rounds, proclamation in (('fr_cc_2007_second_round_20070505_20070506', 'fr_cc_2007_proclamation_sarkozy_20070510'),
                                     ('fr_cc_2012_second_round_20120505_20120506', 'fr_cc_2012_proclamation_hollande_20120510'),
                                     ('fr_cc_2017_second_round_20170506_20170507', 'fr_cc_2017_proclamation_macron_20170510'),
                                     ('fr_2022_second_round_held_20220424', 'fr_cc_proclaims_macron_president_20220427')):
            self.assertLess(claims[rounds]['period']['through'], claims[proclamation]['attested_on'])
        # The 2002 and 2022 investitures precede the stated starts; the proclamations are dated 12 May 1995 and 8 May 2002.
        self.assertLess(claims['fr_chirac_investiture_ceremony_20020516']['attested_on'], '2002-05-17')
        self.assertLess(claims['fr_macron_investiture_second_term_20220507']['attested_on'], '2022-05-14')
        self.assertEqual(claims['fr_cc_proclaims_chirac_19950512']['attested_on'], '1995-05-12')
        self.assertEqual(claims['fr_cc_proclaims_chirac_20020508']['attested_on'], '2002-05-08')
        self.assertIn('not 9 May', claims['fr_cc_proclaims_chirac_20020508']['uncertainty'])
        self.assertIn('not 11 May', claims['fr_cc_proclaims_chirac_19950512']['uncertainty'])
        # Prospective statements are dated by the day made, retrospective ones by the day they state.
        for cid in ('fr_cc_88_60_states_mandate_effect_19880511', 'fr_cc_2002_states_effect_from_20020508',
                    'fr_cc_2022_states_effect_from_20220427'):
            self.assertIn('dated by the day it was made', claims[cid]['uncertainty'], cid)
        self.assertIn('dated by the day it states', claims['fr_macron_says_ending_term_began_20170514']['uncertainty'])
        # The conflicting 11:00 of 2007 is recorded, and only the day is used.
        self.assertIn('conflicts with', claims['fr_cc_2007_investiture_ceremony_at_1100_20070516']['uncertainty'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source['published_date'])
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-27', '2026-09-27'))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('same byte count and SHA-256', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['rights_note'], source['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith(('primary_', 'presidency_')) and source['scope_note'])
            snapshot = source['snapshot']
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/france-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            # Rows repeat the packet claims exactly, in order, keyed by claim_id; no row has a bare 'name' key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on'], row.get('period')),
                                 (claim['text'], claim['locator'], claim.get('attested_on'), claim.get('period')))
                self.assertEqual((row['observation_id'], row['role_id']), (INSTITUTION, ROLE))
                self.assertNotIn('name', row)
                self.assertTrue(row['holder_name'] is None or row['holder_name'].strip())
                self.assertTrue(row['role_title'].startswith('Président de la République'), row['claim_id'])
                self.assertEqual((row['attested_on'], row['event_kind'], row['review_observation']), EVENTS[row['claim_id']])
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'])
                          for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertTrue(source['url'].replace(':80/', '/').endswith(source['original_url'].split('://', 1)[1]), sid)
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('served without Content-Encoding', extract['provenance_note'])
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED]
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others}, LIVE_HOSTS)
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertIn('cache-busting query', self.extracts[sid]['provenance_note'])
            self.assertIn("curl's default User-Agent", self.extracts[sid]['provenance_note'])
            if 'elysee.fr' in self.sources[sid]['url']:
                self.assertIn('wkhtmltopdf', self.sources[sid]['scope_note'])
                self.assertIn('Seul le prononcé fait foi', self.sources[sid]['scope_note'])
                self.assertIn('catalogue labels', self.sources[sid]['scope_note'])
            if 'dila.gouv.fr' in self.sources[sid]['url']:
                self.assertIn('Retention caveat', self.sources[sid]['scope_note'])

    def test_leads_and_volatile_responses_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        for sid in NEW_SOURCES:
            for url in (self.sources[sid]['url'], self.sources[sid].get('original_url') or ''):
                self.assertIsNone(VOLATILE_URL.search(url), url)
            self.assertFalse(urlsplit(self.sources[sid]['url']).query, sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'europe1', 'lapresse', 'britannica', 'larousse'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        attempted = self.section('Sources attempted')
        for marker in ('europe1', 'lapresse', 'elysee-module-8256', 'elysee-module-8260', 'sp-module-2234', '20260804110044',
                       '20230126145839', '2007141pdr.htm', 'doc_proclam_2007', 'dossier-presse', 'cahier23',
                       'les-suites-de-la-proclamation', '20020508.pdf', 'jorf/jo/2022/04/28/0099', 'senat.fr'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        # Records that could not be retrieved, and a wrong identifier, are reported as attempts, never as sources.
        for marker in ('JORFTEXT000045667581', 'JORFTEXT000000465545', 'legifrance.gouv.fr', 'not bypassed'):
            self.assertIn(marker, attempted, marker)
            self.assertNotIn(marker, added, marker)

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
            (lambda p: source(p, 'fr_cc_rapport_activite_2017')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'fr_an_jo_debats_19900827')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'fr_cc_proclamation_20020508')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 6).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'fr_jorf_decree_signed_macron_20260904').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 3).update(until='2007-05-15'), 'Reversed historical interval'),
            (lambda p: holder(p, 1)['claim_ids'].append('fr_cc_proclaims_chirac_19950512'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('fr_does_not_exist'), 'Unknown'),
            (lambda p: source(p, 'fr_cc_proclamation_20020508').update(url='http://www.conseil-constitutionnel.fr/decision/2002/20020508.htm'),
             'Invalid public source URL'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('announced handover used as an end (Mitterrand)', lambda p: holder(p, 0).update(until='1995-05-17')),
            ('outer limit used as an end (Mitterrand)', lambda p: holder(p, 0).update(until='1995-05-21')),
            ('pre-period effect day used as a start (Mitterrand)', lambda p: holder(p, 0).update({'from': '1988-05-21', 'attested_on': None})),
            ('own same-day statement used as a start (Chirac 1995)', lambda p: holder(p, 1).update({'from': '1995-05-17', 'attested_on': None})),
            ('proclamation used as a start (Chirac 1995)', lambda p: holder(p, 1).update({'from': '1995-05-12', 'attested_on': None})),
            ('proclamation used as the observation (Chirac 1995)', lambda p: holder(p, 1).update(attested_on='1995-05-12')),
            ('computed expiry used as an end (Chirac 1995)', lambda p: holder(p, 1).update(until='2002-05-17')),
            ('investiture used as a start (Chirac 2002)', lambda p: holder(p, 2).update({'from': '2002-05-16'})),
            ('successor start used as an end (Chirac 2002)', lambda p: holder(p, 2).update(until='2007-05-16')),
            ('announcement used as an end (Chirac 2002)', lambda p: holder(p, 2).update(until='2007-05-15')),
            ('proclamation used as a start (Sarkozy)', lambda p: holder(p, 3).update({'from': '2007-05-10'})),
            ('computed expiry used as an end (Sarkozy)', lambda p: holder(p, 3).update(until='2012-05-16')),
            ('publication used as a start (Hollande)', lambda p: holder(p, 4).update({'from': '2012-05-11'})),
            ('successor start used as an end (Hollande)', lambda p: holder(p, 4).update(until='2017-05-14')),
            ('inferred end of the first term (Macron)', lambda p: holder(p, 5).update(until='2022-05-13')),
            ('investiture used as a start (Macron 2022)', lambda p: holder(p, 6).update({'from': '2022-05-07'})),
            ('attestation used as a boundary (Macron 2022)', lambda p: holder(p, 6).update(attested_on='2026-09-04')),
            ('election claim cited by a holder', lambda p: holder(p, 6)['claim_ids'].append('fr_2022_second_round_held_20220424')),
            ('ceremony claim cited by a holder', lambda p: holder(p, 2)['claim_ids'].append('fr_chirac_investiture_ceremony_20020516')),
            ('proclamation claim cited by a holder', lambda p: holder(p, 5)['claim_ids'].append('fr_cc_fabius_proclaims_macron_20170510')),
            ('retrospective row cited by a holder', lambda p: holder(p, 1)['claim_ids'].append('fr_cc_note_chirac_1995_passation_19950517')),
            ('table row given a date', lambda p: claim(p, 'fr_cc_dossier2017_row_1995').update(attested_on='1995-05-17')),
            ('round collapsed into the proclamation', lambda p: claim(p, 'fr_cc_2012_second_round_20120505_20120506').update(
                attested_on='2012-05-10')),
            ('prospective statement re-dated to the day it fixes', lambda p: claim(p, 'fr_cc_2002_states_effect_from_20020508').update(
                attested_on='2002-05-17')),
            ('interim holder added', lambda p: role(p)['holder_claims'].insert(1, {
                'name': 'Alain Poher', 'attested_on': '1995-05-16', 'from': None, 'until': None,
                'sources': ['fr_elysee_mitterrand_letter_19950516'], 'claim_ids': ['fr_mitterrand_handover_announced_19950516']})),
            ('holders reordered', lambda p: role(p)['holder_claims'].reverse()),
            ('holder removed', lambda p: role(p)['holder_claims'].pop(2)),
            ('second presidency institution', lambda p: p['institutions'].append(copy.deepcopy(p['institutions'][0]))),
            ('second head-of-state role', lambda p: p['organizations'][0]['roles'].append(dict(copy.deepcopy(role(p)), id='fr_other'))),
            ('role removed', lambda p: p['institutions'][0]['roles'].pop()),
            ('institution mapped to a party', lambda p: p['institutions'][0]['represented_party_ids'].append('France/fr_ps')),
            ('institution given a lifecycle end', lambda p: p['institutions'][0]['lifecycle'].update(until='2026-09-07')),
            ('organization citing a presidency claim', lambda p: p['organizations'][0]['claim_ids'].append(
                'fr_cc_proclaims_mitterrand_19880511')),
            ('organization citing a presidency source', lambda p: p['organizations'][0]['sources'].append('fr_an_jo_debats_19900827')),
            ('identity total removed from a holder note', lambda p: holder(p, 5).update(note='From 14 May 2017.')),
        ]
        presidency_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError)):
                presidency_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted in part', '02': 'Accepted in part', '03': 'Accepted', '04': 'Accepted',
                     '05': 'Accepted', '06': 'Accepted', '07': 'Accepted', '08': 'Accepted in part', '09': 'Accepted',
                     '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| FR-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 14)] + [f'B{n}' for n in range(1, 19)] + [f'C{n}' for n in range(1, 11)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Declined|Resolved by removal)')
        for heading in ('Response identities and stability checks', 'Date ledger', 'Sources attempted', 'Suggested next work orders',
                        'Integration notes'):
            self.assertIn(heading, self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('16e17c18', 'a33a8987', '8761e45c', 'research-index.json', 'test_campaign_research.py',
                     'test_import_cnccfp_census.py', 'supplements/france.json'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'france-presidents-1990-2026-23.md', 'claude/c01-fr-23', '16e17c18', 'a33a8987',
                     '8761e45c', 'test_france_presidents_c01_23.py', 'supplements/france.json'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'France')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (1, 1))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'France'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
