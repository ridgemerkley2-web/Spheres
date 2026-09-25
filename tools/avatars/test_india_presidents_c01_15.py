"""CLAUDE-C01-15: India's presidents, 1990-2026, keep the Election Commission's notices, the Returning Officer's declaration,
the oath administered by the Chief Justice, the stated assumption of office and any stated end apart, and state a start or
an end only where a source does."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_india_prime_ministers_c01_11 as c01_11


# Original response identity recorded in each extract: (bytes, sha256). Every new source is reproducible.
RESPONSES = {
    'in_gazette_ext_p2s3ii_no19_19900113':
        (95282, '4e10fbbe05c55af36f827c134c5ff851a7bce1aa681cc301ebd895692a35bacf'),
    'in_gazette_ext_eci_on49_19920610':
        (477596, '39d6f0914fa658d0b85a2f13d77980f20dcf9d5b1a96e4e325eafe7108b55f1b'),
    'in_ls_jpi_199209':
        (19494832, 'db63276d42ae2cf1b9670c317ec7fbd90fb4142fa9bb9bf7ef79c54d51e919f9'),
    'in_rb_former_president_venkataraman':
        (56372, 'c73d0fa3eb0c3d3fa971d2d6b0db6f64d0f92626d19de94d91add0f340be02a1'),
    'in_gazette_ext_p2s3ii_no463_19920717':
        (86875, 'c38ac9dff034d98dfd0ba64aa7b2fef4414e99172b2451a7b1939fd1e63ed0e0'),
    'in_gazette_ext_p2s3ii_no475_19920724':
        (63461, '9ccccde383954b6e4333ee638d6f95cac5d25c4ebccc9cd67e7d9a22da0e3939'),
    'in_gazette_ext_p2s3ii_no479_19920725':
        (136366, '5d56bc3ac541eb2d9b263af206346e39d0e7e7ad5af2e018674887247ec60a46'),
    'in_rb_former_president_sharma':
        (56354, 'f835ea08d9f6fad1b3ad372f6ed0956dafb5ba3191e5c2b61db88f1d10269158'),
    'in_gazette_ext_eci_on65_19970609':
        (544135, '373b8d88fe5ce6dd0299468dbadea5ea385e10929e0280de36619e15e7dc4055'),
    'in_gazette_ext_p2s3ii_no407_19970722':
        (60255, '06e047abc596a69b3083a3ab4a68e73d2e111a5f816db0f493102d094784b92b'),
    'in_gazette_ext_p2s3ii_no414_19970724':
        (60134, '77be3e9b8c0af5c616a55f435ffdbbb4482bdcacd6287ad5b07092a095c6d4bd'),
    'in_gazette_ext_p2s3ii_no416_19970725':
        (134579, 'b7ca86bd390b232c16f1c4351a73a2f103f5fd71271057793ecf1fdaaf58165c'),
    'in_ls_debates_19970725':
        (10823304, '90481cf5c86643105a9b67df2df7146aa8438b98cc0ea47febaa71fc0180e70f'),
    'in_rb_former_president_narayanan':
        (56405, '8eb27596d6d95d150a1fc0280c3e8219ed54c4bfaf0147257b5044926139fca4'),
    'in_gazette_ext_eci_on60_20020611':
        (488037, '8ae25a5c05fffd715ff6666843ce2cda8096ee524c5318c7ccb609d0388df87c'),
    'in_gazette_ext_no648_so763e_20020718':
        (52620, '7414cccc8e3ecfb4cb319194e7a3c1a56d4e8bff2608ac00c5dc3e2d0e817c23'),
    'in_rs_debates_20020718':
        (1486333, 'c7f43556fc4c49d1a8f79752261ecfd2e3d5ff5d2e77099ceae80aefc59b3b6c'),
    'in_poi_narayanan_congratulates_kalam_20020718':
        (2758, '622a91d47ab4a3f36235585731c021a86de51084aa72c71549ca50f7da2601de'),
    'in_poi_kalam_assumption_speech_20020725':
        (13348, '0f7bed08a917f2f338ccbc55d066c6c77a6638f7d6387f999985cbac08492e5b'),
    'in_poi_kalam_first_day_release_20020725':
        (7353, '0cb19059457be5f6166b6f3405fc10c5063ccd1fb59857c367f3fb0ed7513a73'),
    'in_gazette_ext_no723_so849e_20020813':
        (39931, '86135c39e1597fbb6dbe7bc9ac53b0d4e069f75c429eef6ff2c908f6a127de74'),
    'in_gazette_ext_eci_on85_20070616':
        (373904, 'cf0e3a33eebd0ae9b6c03c8314f5f50abaadaf2a0cdc67d728e0f29b6a091812'),
    'in_gazette_ext_no891_so1193e_20070721':
        (41704, 'e4c73073c7123d4c9d54c1376a7ead7fd3fd72825f613605daf6d053312f8f08'),
    'in_gazette_ext_no901_so1205e_20070724':
        (38987, 'c67af9716e5e1f68057ba5773c2f738b78c53f53c6676d013ced994a233f1366'),
    'in_gazette_ext_no903_so1207e_20070725':
        (128992, '0ae199eefcdb35e5d6070638fa72e85db6d8cf6f27a4194b33ce0fcf9139d8dd'),
    'in_poi_patil_assumption_speech_20070725':
        (10882, '685218e368e1f5fc32c965cada9c0a8b6f2a60c5cb2048b79d3f046448a4eea2'),
    'in_poi_swearing_in_page_patil':
        (2904, 'e7f97344f841ad19ab2c36cbc000642f006da9d30a35680003877667852a410e'),
    'in_pp_profile_former_president_patil':
        (9030, 'f2b67138e9048eec5369598b4e5e24b61f33b415927976a2cc2e6532c940e429'),
    'in_gazette_ext_eci_on45_20120616':
        (230151, 'd30202f8c8f0f57a685b984292ec2d0e3bc8d566c740c29a91cf7d1cd0717d57'),
    'in_gazette_ext_no1364_so1657e_20120722':
        (22147, 'a77e15dc1fb4ea3e8dcd9e66b52d93c824f34351b46de907959756ebb0f38780'),
    'in_gazette_ext_no1376_so1672e_20120724':
        (24435, '9718610dd0ea9fc32701e30a6c94a082f19f5541fa23cc2e24ad1268fb78f6c6'),
    'in_gazette_ext_no1389_so1687e_20120725':
        (52566, 'b233ae369a5c1bea3d7070e6d4c8f6ec3674cea9eff5e7b55e1ec0a5d2fb10b9'),
    'in_gazette_ext_no1425_so1724e_20120731':
        (17687, '10ba73725c43b5c17366ab4a6a8e58906059b334f652426a2a71f90dfdab04df'),
    'in_poi_pranab_assumption_speech_20120725':
        (12598, '0352665a7a57a3d7b66f6b8857b3af902f80f5e4db997484282299d40393d969'),
    'in_poi_swearing_in_page_pranab':
        (5595, 'e8a2e6fe6c17a90d793b3de2ff6475b66c93fe19870b5d173a11d987c4fb0e1f'),
    'in_gazette_ext_eci_on38_20170614':
        (1380182, '11fb2a53dd349f7f7101d38d1a90a9f92bdb969dc991857762b8955ed6bbd2e1'),
    'in_gazette_ext_so2282_declaration_20170720':
        (1266561, 'bce8741ce227ea19643b9068765a4654f7c66d12e0575df07b8e939ff9d314c6'),
    'in_gazette_ext_so2309_ceremony_20170724':
        (1265680, '27e1ef83a9e7b118aec453620477c0ddeb9608a8dd2b327b874cddcd528ccb64'),
    'in_pib_pranab_eve_of_demitting_office_20170724':
        (14661, '0b62ff975909eebb5d1fb3fa5ecfb1e0ca05605db917c5fed78b8264bf646233'),
    'in_rb_pranab_farewell_address_20170724':
        (94872, 'f6cb0331448eb3853382dec8b6f8264222fbc140dac25a1f1e7171b7a0809bfa'),
    'in_gazette_ext_so2318_assumption_20170725':
        (1356195, '7e0a5bbe67e01961938e41d4a820715b598399816c0d91cc47534a3e844f487e'),
    'in_rb_kovind_assumption_speech_20170725':
        (90535, '6643fae1fc1db4d35926f3ffd85cf38c7037555d0e14465a62fc8835f54d488b'),
    'in_pib_kovind_profile_20170725':
        (9699, '1670c76e15ce1fcbdf34a5145db19ae3e40e95339c893dce4a929f4ada75c425'),
    'in_pib_photo_20170725_oath':
        (56334, '710516928416ec4934a592e268e65e80b0a8bab2c90e1dd03cc770b7da0eff8f'),
    'in_gazette_ext_so2487_corrigendum_20170804':
        (1275649, '9098b6d29b80bb58dc6be4c076d20a8a3daf8fe3d77ebe55c5dc643e300cb045'),
    'in_gazette_ext_eci_on40_20220615':
        (1095857, '3ed805e2f26c7ac33c9052c02fefc879dcc15987d6c6656b6ef9c3a6ae5f7f79'),
    'in_gazette_ext_so3325_declaration_20220722':
        (1069378, '728485253cece6c57f1b95d6ffdb1b9fc2d119b798cab5a7a16ef452bf546587'),
    'in_gazette_ext_so3365_ceremony_20220724':
        (845105, '12303213746e18aee2de8c79007dab2f86ae048e387e813acd307af9272a59f1'),
    'in_pib_kovind_farewell_address_20220724':
        (114788, '485dbf2399e2544f1c0d11ec200a466f86d3ba91c36bb209ba7525fccf31b166'),
    'in_gazette_ext_so3373_assumption_20220725':
        (1151852, '20256cd28f8d5848b356a27806b352985e3f809f8c5d2f945feb89882386688c'),
    'in_rb_murmu_assumption_address_20220725':
        (67544, 'deaf87826586768663ec8956efcceab3781492768ccefe4b1c7af26f742ec22f'),
    'in_rb_photo_gallery_20220725':
        (72793, '956acf2c17783230a2560bf713b5703294866659ef64f7dadf1736da067f118d'),
    'in_pib_pmo_murmu_took_oath_20220725':
        (48168, '6d8b43fa00fd482010b5f8292d42c76dcaabbb5e5d1f31b907294038bdbb2dba'),
    'in_rb_murmu_mozambique_delegation_20220729':
        (42823, 'f281a1c5540a18c168bef6cb407f1da6652c4913e41cc7a23c973f460544eff1'),
    'in_gazette_ext_ordinance_2_2026_20260605':
        (329484, 'd3cc622530042291a5b8dcd6ebc6bca2366da8aaab596cccf8e0d3e2e4aba18e'),
    'in_rb_murmu_independence_day_address_20260814':
        (623615, 'aed4c60f6792dc2ea391cd02be29fa6dfe0fd3b3d61da3883d07c82996146d19'),
}
ARCHIVED = {
    'in_rb_former_president_venkataraman': '20260414052010',
    'in_rb_former_president_sharma': '20260414060905',
    'in_rb_former_president_narayanan': '20260414065240',
    'in_poi_narayanan_congratulates_kalam_20020718': '20020823090957',
    'in_poi_kalam_assumption_speech_20020725': '20020816131338',
    'in_poi_kalam_first_day_release_20020725': '20020823091738',
    'in_poi_patil_assumption_speech_20070725': '20070921001016',
    'in_poi_swearing_in_page_patil': '20080314145026',
    'in_pp_profile_former_president_patil': '20130101083749',
    'in_poi_pranab_assumption_speech_20120725': '20120809045746',
    'in_poi_swearing_in_page_pranab': '20120919193133',
    'in_pib_pranab_eve_of_demitting_office_20170724': '20170728203623',
    'in_rb_pranab_farewell_address_20170724': '20170724235959',
    'in_rb_kovind_assumption_speech_20170725': '20170725193433',
    'in_pib_kovind_profile_20170725': '20170728203039',
    'in_pib_photo_20170725_oath': '20170726040224',
    'in_pib_kovind_farewell_address_20220724': '20220724144120',
    'in_rb_murmu_assumption_address_20220725': '20220725075941',
    'in_rb_photo_gallery_20220725': '20220725133207',
    'in_pib_pmo_murmu_took_oath_20220725': '20220725082247',
    'in_rb_murmu_mozambique_delegation_20220729': '20220815221305',
}
MIRRORS = ('in_ls_jpi_199209',
           'in_gazette_ext_eci_on65_19970609',
           'in_ls_debates_19970725',
           'in_gazette_ext_eci_on60_20020611',
           'in_gazette_ext_eci_on85_20070616',
           'in_gazette_ext_eci_on45_20120616')
PDF_PAGES = {
    'in_gazette_ext_p2s3ii_no19_19900113': [1, 2],
    'in_gazette_ext_eci_on49_19920610': [1, 2],
    'in_ls_jpi_199209': [1, 2, 3, 6, 9, 19, 22, 23, 45, 46, 50, 51, 76],
    'in_gazette_ext_p2s3ii_no463_19920717': [1, 2],
    'in_gazette_ext_p2s3ii_no475_19920724': [1, 2],
    'in_gazette_ext_p2s3ii_no479_19920725': [1, 2, 3, 4],
    'in_gazette_ext_eci_on65_19970609': [2],
    'in_gazette_ext_p2s3ii_no407_19970722': [1, 2],
    'in_gazette_ext_p2s3ii_no414_19970724': [1, 2],
    'in_gazette_ext_p2s3ii_no416_19970725': [1, 2, 3, 4],
    'in_ls_debates_19970725': [8, 140, 157],
    'in_gazette_ext_eci_on60_20020611': [1, 2],
    'in_gazette_ext_no648_so763e_20020718': [1, 2],
    'in_rs_debates_20020718': [3, 256],
    'in_gazette_ext_no723_so849e_20020813': [1, 2],
    'in_gazette_ext_eci_on85_20070616': [1],
    'in_gazette_ext_no891_so1193e_20070721': [1],
    'in_gazette_ext_no901_so1205e_20070724': [1],
    'in_gazette_ext_no903_so1207e_20070725': [1, 2, 3, 4],
    'in_gazette_ext_eci_on45_20120616': [1, 2],
    'in_gazette_ext_no1364_so1657e_20120722': [1],
    'in_gazette_ext_no1376_so1672e_20120724': [1],
    'in_gazette_ext_no1389_so1687e_20120725': [1, 3, 4],
    'in_gazette_ext_no1425_so1724e_20120731': [1],
    'in_gazette_ext_eci_on38_20170614': [1, 2, 3, 5, 6, 7, 10, 11, 12],
    'in_gazette_ext_so2282_declaration_20170720': [1, 2],
    'in_gazette_ext_so2309_ceremony_20170724': [1],
    'in_gazette_ext_so2318_assumption_20170725': [1, 4, 5, 6, 7],
    'in_gazette_ext_so2487_corrigendum_20170804': [1],
    'in_gazette_ext_eci_on40_20220615': [1, 2],
    'in_gazette_ext_so3325_declaration_20220722': [1, 2],
    'in_gazette_ext_so3365_ceremony_20220724': [1, 2],
    'in_gazette_ext_so3373_assumption_20220725': [1, 2, 3, 4, 5, 6, 7],
    'in_gazette_ext_ordinance_2_2026_20260605': [1, 2, 3],
    'in_rb_murmu_independence_day_address_20260814': [1],
}
EVENTS = {
    'in_venkataraman_president_order_delhi_19900113': ('1990-01-13', 'in_office_attestation', 'IN-PRES-01'),
    'in_eci_venkataraman_term_due_to_expire_19920610': ('1992-06-10', 'term_expiry_scheduled', 'IN-PRES-01'),
    'in_eci_appoints_presidential_election_dates_19920610': ('1992-06-10', 'election_called', 'IN-PRES-02'),
    'in_mps_farewell_to_president_venkataraman_19920721': ('1992-07-21', 'farewell_address', 'IN-PRES-01'),
    'in_jpi_presidential_count_19920716': ('1992-07-16', 'result_count_reported', 'IN-PRES-02'),
    'in_cji_kania_administers_oath_to_sharma_19920725': ('1992-07-25', 'oath_of_office', 'IN-PRES-02'),
    'in_sharma_address_on_entering_office_19920725': ('1992-07-25', 'assumption_statement', 'IN-PRES-02'),
    'in_jpi_span_venkataraman_19870725_19920725': (None, 'retrospective_term_span', 'IN-PRES-01'),
    'in_rb_span_venkataraman_19870725_19920725': (None, 'retrospective_term_span', 'IN-PRES-01'),
    'in_ro_declares_shanker_dayal_sharma_elected_19920716': ('1992-07-16', 'result_declaration', 'IN-PRES-02'),
    'in_gazette_publishes_sharma_declaration_19920717': ('1992-07-17', 'result_publication', 'IN-PRES-02'),
    'in_sharma_assumption_ceremony_notified_19920724': ('1992-07-24', 'assumption_ceremony_scheduled', 'IN-PRES-02'),
    'in_sharma_takes_seat_as_president_19920725': ('1992-07-25', 'assumption_statement', 'IN-PRES-02'),
    'in_proclamation_sharma_entered_upon_office_19920725': ('1992-07-25', 'assumption_proclaimed', 'IN-PRES-02'),
    'in_rb_span_sharma_19920725_19970725': (None, 'retrospective_term_span', 'IN-PRES-02'),
    'in_eci_sharma_term_due_to_expire_19970609': ('1997-06-09', 'term_expiry_scheduled', 'IN-PRES-09'),
    'in_eci_appoints_presidential_election_dates_19970609': ('1997-06-09', 'election_called', 'IN-PRES-03'),
    'in_ro_declares_narayanan_elected_19970717': ('1997-07-17', 'result_declaration', 'IN-PRES-03'),
    'in_gazette_publishes_narayanan_declaration_19970722': ('1997-07-22', 'result_publication', 'IN-PRES-03'),
    'in_narayanan_assumption_ceremony_notified_19970724': ('1997-07-24', 'assumption_ceremony_scheduled', 'IN-PRES-03'),
    'in_narayanan_takes_seat_as_president_19970725': ('1997-07-25', 'assumption_statement', 'IN-PRES-03'),
    'in_proclamation_narayanan_entered_upon_office_19970725': ('1997-07-25', 'assumption_proclaimed', 'IN-PRES-03'),
    'in_narayanan_assumption_recalled_in_lok_sabha_19970725': ('1997-07-25', 'assumption_recalled_in_debate', 'IN-PRES-03'),
    'in_rb_span_narayanan_19970725_20020725': (None, 'retrospective_term_span', 'IN-PRES-03'),
    'in_eci_narayanan_term_due_to_expire_20020611': ('2002-06-11', 'term_expiry_scheduled', 'IN-PRES-09'),
    'in_ro_tripathi_declares_kalam_elected_20020718': ('2002-07-18', 'result_declaration', 'IN-PRES-04'),
    'in_law_ministry_publishes_kalam_declaration_20020718': ('2002-07-18', 'result_publication', 'IN-PRES-04'),
    'in_rs_informed_ro_declared_kalam_20020718': ('2002-07-18', 'result_announced_in_house', 'IN-PRES-04'),
    'in_rs_kalam_votes_reported_20020718': ('2002-07-18', 'result_figures_reported', 'IN-PRES-04'),
    'in_narayanan_congratulates_kalam_on_election_20020718': ('2002-07-18', 'election_acknowledged', 'IN-PRES-04'),
    'in_kalam_assumption_speech_20020725': ('2002-07-25', 'assumption_statement', 'IN-PRES-04'),
    'in_kalam_oath_taken_reported_20020725': ('2002-07-25', 'oath_of_office', 'IN-PRES-04'),
    'in_kalam_president_first_day_engagements_20020725': ('2002-07-25', 'in_office_attestation', 'IN-PRES-04'),
    'in_narayanan_styled_ex_president_20020725': ('2002-07-25', 'styled_former_president', 'IN-PRES-09'),
    'in_kalam_assumption_recited_in_corrigendum_20020725': ('2002-07-25', 'assumption_recited_in_corrigendum', 'IN-PRES-04'),
    'in_mha_corrects_kalam_hindi_name_20020813': ('2002-08-13', 'name_corrigendum', 'IN-PRES-04'),
    'in_eci_kalam_term_due_to_expire_20070616': ('2007-06-16', 'term_expiry_scheduled', 'IN-PRES-09'),
    'in_ro_achary_declares_patil_elected_20070721': ('2007-07-21', 'result_declaration', 'IN-PRES-05'),
    'in_law_ministry_publishes_patil_declaration_20070721': ('2007-07-21', 'result_publication', 'IN-PRES-05'),
    'in_mha_schedules_patil_assumption_ceremony_20070724': ('2007-07-24', 'assumption_ceremony_scheduled', 'IN-PRES-05'),
    'in_patil_takes_seat_as_president_20070725': ('2007-07-25', 'assumption_statement', 'IN-PRES-05'),
    'in_mha_proclaims_patil_entered_upon_office_20070725': ('2007-07-25', 'assumption_proclaimed', 'IN-PRES-05'),
    'in_patil_assumption_speech_20070725': ('2007-07-25', 'assumption_statement', 'IN-PRES-05'),
    'in_patil_swearing_in_recalled_caption_20070725': ('2007-07-25', 'swearing_in_recalled_retrospective', 'IN-PRES-05'),
    'in_pp_profile_patil_assumed_office_recalled_20070725': ('2007-07-25', 'assumption_recalled_retrospective', 'IN-PRES-05'),
    'in_eci_patil_term_due_to_expire_20120616': ('2012-06-16', 'term_expiry_scheduled', 'IN-PRES-09'),
    'in_ro_agnihotri_declares_pranab_elected_20120722': ('2012-07-22', 'result_declaration', 'IN-PRES-06'),
    'in_law_ministry_publishes_pranab_declaration_20120722': ('2012-07-22', 'result_publication', 'IN-PRES-06'),
    'in_mha_schedules_pranab_assumption_ceremony_20120724': ('2012-07-24', 'assumption_ceremony_scheduled', 'IN-PRES-06'),
    'in_pranab_takes_seat_as_president_20120725': ('2012-07-25', 'assumption_statement', 'IN-PRES-06'),
    'in_mha_proclaims_pranab_entered_upon_office_20120725': ('2012-07-25', 'assumption_proclaimed', 'IN-PRES-06'),
    'in_pranab_ls_membership_ceased_wef_20120725': ('2012-07-25', 'lok_sabha_membership_ceased_on_assumption', 'IN-PRES-06'),
    'in_pranab_assumption_speech_20120725': ('2012-07-25', 'assumption_statement', 'IN-PRES-06'),
    'in_pranab_refers_to_oath_taken_20120725': ('2012-07-25', 'oath_referred_in_address', 'IN-PRES-06'),
    'in_pranab_sworn_in_by_chief_justice_caption_20120725': ('2012-07-25', 'oath_of_office', 'IN-PRES-06'),
    'in_eci_pranab_term_due_to_expire_20170614': ('2017-06-14', 'term_expiry_scheduled', 'IN-PRES-09'),
    'in_ro_declares_kovind_elected_20170720': ('2017-07-20', 'result_declaration', 'IN-PRES-07'),
    'in_kovind_declaration_published_so2282_20170720': ('2017-07-20', 'result_publication', 'IN-PRES-07'),
    'in_kovind_assumption_ceremony_scheduled_20170724': ('2017-07-24', 'assumption_ceremony_scheduled', 'IN-PRES-07'),
    'in_pranab_on_eve_of_demitting_office_20170724': ('2017-07-24', 'end_of_term_anticipated', 'IN-PRES-09'),
    'in_pranab_farewell_address_20170724': ('2017-07-24', 'farewell_address', 'IN-PRES-09'),
    'in_kovind_takes_seat_as_president_20170725': ('2017-07-25', 'assumption_statement', 'IN-PRES-07'),
    'in_kovind_entered_upon_office_proclaimed_20170725': ('2017-07-25', 'assumption_proclaimed', 'IN-PRES-07'),
    'in_kovind_assumption_address_20170725': ('2017-07-25', 'assumption_statement', 'IN-PRES-07'),
    'in_kovind_assumed_charge_stated_20170725': ('2017-07-25', 'assumption_statement', 'IN-PRES-07'),
    'in_kovind_oath_administered_by_cji_khehar_20170725': ('2017-07-25', 'oath_of_office', 'IN-PRES-07'),
    'in_kovind_signs_register_after_swearing_in_20170725': ('2017-07-25', 'in_office_attestation', 'IN-PRES-07'),
    'in_pranab_styled_former_president_20170725': ('2017-07-25', 'styled_former_president', 'IN-PRES-09'),
    'in_kovind_name_corrigendum_20170804': ('2017-08-04', 'name_corrigendum', 'IN-PRES-07'),
    'in_eci_kovind_term_due_to_expire_20220615': ('2022-06-15', 'term_expiry_scheduled', 'IN-PRES-09'),
    'in_ro_declares_murmu_elected_20220721': ('2022-07-21', 'result_declaration', 'IN-PRES-08'),
    'in_murmu_declaration_published_so3325_20220722': ('2022-07-22', 'result_publication', 'IN-PRES-08'),
    'in_murmu_assumption_ceremony_scheduled_20220724': ('2022-07-24', 'assumption_ceremony_scheduled', 'IN-PRES-08'),
    'in_kovind_farewell_address_20220724': ('2022-07-24', 'farewell_address', 'IN-PRES-09'),
    'in_murmu_takes_seat_as_president_20220725': ('2022-07-25', 'assumption_statement', 'IN-PRES-08'),
    'in_murmu_entered_upon_office_proclaimed_20220725': ('2022-07-25', 'assumption_proclaimed', 'IN-PRES-08'),
    'in_murmu_assumption_address_20220725': ('2022-07-25', 'assumption_statement', 'IN-PRES-08'),
    'in_murmu_oath_administered_by_cji_ramana_20220725': ('2022-07-25', 'oath_of_office', 'IN-PRES-08'),
    'in_kovind_styled_former_president_20220725': ('2022-07-25', 'styled_former_president', 'IN-PRES-09'),
    'in_pmo_reports_murmu_took_oath_20220725': ('2022-07-25', 'oath_taken_reported', 'IN-PRES-08'),
    'in_rb_murmu_assumption_recalled_20220725': ('2022-07-25', 'assumption_recalled_retrospective', 'IN-PRES-08'),
    'in_murmu_receives_mozambique_delegation_20220729': ('2022-07-29', 'in_office_attestation', 'IN-PRES-08'),
    'in_murmu_promulgates_income_tax_ordinance_20260605': ('2026-06-05', 'in_office_attestation', 'IN-PRES-10'),
    'in_murmu_independence_day_eve_address_20260814': ('2026-08-14', 'in_office_attestation', 'IN-PRES-10'),
}
NEW_SOURCES = list(RESPONSES)
ORIGINAL_SOURCES = ('in_eci_national_parties_20240323', 'in_eci_state_parties_20240323')
INSTITUTION = 'in_presidency'
PRES = 'in_president'
T_PRES = 'President of India'
OFFICIAL_HOSTS = {'egazette.gov.in', 'cms.rajyasabha.nic.in', 'www.presidentofindia.gov.in'}
V, S, N, K = 'R. Venkataraman', 'Shankar Dayal Sharma', 'K. R. Narayanan', 'A. P. J. Abdul Kalam'
P, M, RK, DM = 'Pratibha Devisingh Patil', 'Pranab Mukherjee', 'Ram Nath Kovind', 'Droupadi Murmu'

# Exact holder observations of in_president: (name, attested_on, from, until), in chronological order. No end is stated.
HOLDERS = [
    (V, '1990-01-13', None, None),
    (S, None, '1992-07-25', None),
    (N, None, '1997-07-25', None),
    (K, None, '2002-07-25', None),
    (P, None, '2007-07-25', None),
    (M, None, '2012-07-25', None),
    (RK, None, '2017-07-25', None),
    (DM, None, '2022-07-25', None),
]
HOLDER_CLAIMS = [
    ['in_venkataraman_president_order_delhi_19900113'],
    ['in_sharma_takes_seat_as_president_19920725', 'in_proclamation_sharma_entered_upon_office_19920725',
     'in_cji_kania_administers_oath_to_sharma_19920725', 'in_sharma_address_on_entering_office_19920725'],
    ['in_narayanan_takes_seat_as_president_19970725', 'in_proclamation_narayanan_entered_upon_office_19970725'],
    ['in_kalam_assumption_speech_20020725', 'in_kalam_oath_taken_reported_20020725',
     'in_kalam_president_first_day_engagements_20020725'],
    ['in_patil_takes_seat_as_president_20070725', 'in_mha_proclaims_patil_entered_upon_office_20070725',
     'in_patil_assumption_speech_20070725'],
    ['in_pranab_takes_seat_as_president_20120725', 'in_mha_proclaims_pranab_entered_upon_office_20120725',
     'in_pranab_assumption_speech_20120725', 'in_pranab_sworn_in_by_chief_justice_caption_20120725'],
    ['in_kovind_takes_seat_as_president_20170725', 'in_kovind_entered_upon_office_proclaimed_20170725',
     'in_kovind_assumption_address_20170725', 'in_kovind_assumed_charge_stated_20170725',
     'in_kovind_oath_administered_by_cji_khehar_20170725', 'in_kovind_signs_register_after_swearing_in_20170725'],
    ['in_murmu_takes_seat_as_president_20220725', 'in_murmu_entered_upon_office_proclaimed_20220725',
     'in_murmu_assumption_address_20220725', 'in_murmu_oath_administered_by_cji_ramana_20220725',
     'in_murmu_receives_mozambique_delegation_20220729', 'in_murmu_promulgates_income_tax_ordinance_20260605',
     'in_murmu_independence_day_eve_address_20260814'],
]
# Claims that must never feed a holder observation.
RESULTS = (  # election schedules, the Returning Officer's declarations, their publication and reports of the result
    'in_eci_appoints_presidential_election_dates_19920610', 'in_jpi_presidential_count_19920716',
    'in_ro_declares_shanker_dayal_sharma_elected_19920716', 'in_gazette_publishes_sharma_declaration_19920717',
    'in_eci_appoints_presidential_election_dates_19970609', 'in_ro_declares_narayanan_elected_19970717',
    'in_gazette_publishes_narayanan_declaration_19970722', 'in_ro_tripathi_declares_kalam_elected_20020718',
    'in_law_ministry_publishes_kalam_declaration_20020718', 'in_rs_informed_ro_declared_kalam_20020718',
    'in_rs_kalam_votes_reported_20020718', 'in_narayanan_congratulates_kalam_on_election_20020718',
    'in_ro_achary_declares_patil_elected_20070721', 'in_law_ministry_publishes_patil_declaration_20070721',
    'in_ro_agnihotri_declares_pranab_elected_20120722', 'in_law_ministry_publishes_pranab_declaration_20120722',
    'in_ro_declares_kovind_elected_20170720', 'in_kovind_declaration_published_so2282_20170720',
    'in_ro_declares_murmu_elected_20220721', 'in_murmu_declaration_published_so3325_20220722')
PROSPECTIVE = (  # ceremony programmes and notices that a term is due to expire
    'in_sharma_assumption_ceremony_notified_19920724', 'in_narayanan_assumption_ceremony_notified_19970724',
    'in_mha_schedules_patil_assumption_ceremony_20070724', 'in_mha_schedules_pranab_assumption_ceremony_20120724',
    'in_kovind_assumption_ceremony_scheduled_20170724', 'in_murmu_assumption_ceremony_scheduled_20220724',
    'in_eci_venkataraman_term_due_to_expire_19920610', 'in_eci_sharma_term_due_to_expire_19970609',
    'in_eci_narayanan_term_due_to_expire_20020611', 'in_eci_kalam_term_due_to_expire_20070616',
    'in_eci_patil_term_due_to_expire_20120616', 'in_eci_pranab_term_due_to_expire_20170614',
    'in_eci_kovind_term_due_to_expire_20220615')
ENDINGS = (  # farewells, the eve of demitting office and same-day stylings of a predecessor as former President
    'in_mps_farewell_to_president_venkataraman_19920721', 'in_pranab_on_eve_of_demitting_office_20170724',
    'in_pranab_farewell_address_20170724', 'in_kovind_farewell_address_20220724',
    'in_narayanan_styled_ex_president_20020725', 'in_pranab_styled_former_president_20170725',
    'in_kovind_styled_former_president_20220725')
RETROSPECTIVE = (  # retrospective spans, later recollections and a member's statement in the House
    'in_jpi_span_venkataraman_19870725_19920725', 'in_rb_span_venkataraman_19870725_19920725',
    'in_rb_span_sharma_19920725_19970725', 'in_rb_span_narayanan_19970725_20020725',
    'in_narayanan_assumption_recalled_in_lok_sabha_19970725', 'in_patil_swearing_in_recalled_caption_20070725',
    'in_pp_profile_patil_assumed_office_recalled_20070725', 'in_rb_murmu_assumption_recalled_20220725')
CORROBORATION = (  # a corrigendum's recital, name corrigenda, a Lok Sabha seat notice, a reference to the oath, the PMO
    'in_kalam_assumption_recited_in_corrigendum_20020725', 'in_mha_corrects_kalam_hindi_name_20020813',
    'in_pranab_ls_membership_ceased_wef_20120725', 'in_pranab_refers_to_oath_taken_20120725',
    'in_kovind_name_corrigendum_20170804', 'in_pmo_reports_murmu_took_oath_20220725')
NEVER_HOLDER = RESULTS + PROSPECTIVE + ENDINGS + RETROSPECTIVE + CORROBORATION
SPANS = tuple(cid for cid in RETROSPECTIVE if '_span_' in cid)
# Event kinds that may bound or date a holder; every other kind never does. No source states an end, so no claim has an
# until kind; the kind is named so that a stated end, if one is ever recorded, is recognised and nothing else is.
FROM_KINDS = {'assumption_statement', 'assumption_proclaimed'}
UNTIL_KINDS = {'end_of_office_stated'}
ATTEST_KINDS = {'in_office_attestation', 'oath_of_office'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | ATTEST_KINDS
NEVER_KINDS = {'election_called', 'result_count_reported', 'result_declaration', 'result_publication',
               'result_announced_in_house', 'result_figures_reported', 'election_acknowledged',
               'assumption_ceremony_scheduled', 'term_expiry_scheduled', 'farewell_address', 'end_of_term_anticipated',
               'styled_former_president', 'retrospective_term_span', 'assumption_recalled_in_debate',
               'swearing_in_recalled_retrospective', 'assumption_recalled_retrospective',
               'assumption_recited_in_corrigendum', 'name_corrigendum', 'lok_sabha_membership_ceased_on_assumption',
               'oath_referred_in_address', 'oath_taken_reported'}
# Dates that are never any holder's attested_on, start or end: election notices, polls, declarations and publications,
# ceremony programmes, term-expiry days, farewells, corrigenda, the 1987 start in a retrospective span, and the Lok Sabha
# notice's own dates.
NEVER_HOLDER_DATE = {
    '1987-07-25', '1992-06-10', '1992-07-13', '1992-07-16', '1992-07-17', '1992-07-21', '1992-07-24', '1997-06-09',
    '1997-07-14', '1997-07-17', '1997-07-22', '1997-07-24', '2002-06-11', '2002-07-18', '2002-07-24', '2002-08-13',
    '2007-06-16', '2007-07-21', '2007-07-24', '2012-06-16', '2012-07-22', '2012-07-24', '2012-07-30', '2012-07-31',
    '2017-06-14', '2017-07-20', '2017-07-24', '2017-08-04', '2022-06-15', '2022-07-21', '2022-07-22', '2022-07-24'}
STARTS = [(h[0], h[2]) for h in HOLDERS if h[2]]
SURNAMES = {V: 'Venkataraman', S: 'Dayal Sharma', N: 'Narayanan', K: 'Kalam', P: 'Patil', M: 'Mukherjee', RK: 'Kovind',
            DM: 'Murmu'}
# The handoff's separation guard, per election: the declaration, the oath administered by the Chief Justice and the stated
# assumption of office are distinct claims with distinct kinds and days (no end is stated in any year).
SEPARATION = {
    1992: ('in_ro_declares_shanker_dayal_sharma_elected_19920716', ('in_cji_kania_administers_oath_to_sharma_19920725',),
           ('in_sharma_takes_seat_as_president_19920725', 'in_proclamation_sharma_entered_upon_office_19920725')),
    1997: ('in_ro_declares_narayanan_elected_19970717', (),
           ('in_narayanan_takes_seat_as_president_19970725', 'in_proclamation_narayanan_entered_upon_office_19970725')),
    2002: ('in_ro_tripathi_declares_kalam_elected_20020718', ('in_kalam_oath_taken_reported_20020725',),
           ('in_kalam_assumption_speech_20020725',)),
    2007: ('in_ro_achary_declares_patil_elected_20070721', (),
           ('in_patil_takes_seat_as_president_20070725', 'in_mha_proclaims_patil_entered_upon_office_20070725')),
    2012: ('in_ro_agnihotri_declares_pranab_elected_20120722', ('in_pranab_sworn_in_by_chief_justice_caption_20120725',),
           ('in_pranab_takes_seat_as_president_20120725', 'in_mha_proclaims_pranab_entered_upon_office_20120725')),
    2017: ('in_ro_declares_kovind_elected_20170720', ('in_kovind_oath_administered_by_cji_khehar_20170725',),
           ('in_kovind_takes_seat_as_president_20170725', 'in_kovind_entered_upon_office_proclaimed_20170725')),
    2022: ('in_ro_declares_murmu_elected_20220721', ('in_murmu_oath_administered_by_cji_ramana_20220725',),
           ('in_murmu_takes_seat_as_president_20220725', 'in_murmu_entered_upon_office_proclaimed_20220725')),
}
# Leads, secondary sources and records the checks declined or replaced: never a source URL.
LEAD_URL_MARKERS = ('wikipedia', 'ramnathkovind', 'E-0576-1990-0004-24040', 'E-0718-1987-0382-31802', 'in.gazette.e.1987',
                    'relid=85462', 'relid=85503', 'relid=29331', 'relid=29379', 'relid=85501', 'relid=29367',
                    'relid=168957', 'relid=168956', 'relid=168868', 'PRID=1844557', 'photoid=111499', '20170727221652', '20220920170715',
                    'presidential_elec2002', 'mypresidentialye', 'in.gazette.e.1997.1680', 'ID_181_25071997',
                    'former-presidents', 'sp250717', 'r18072002', 'r25072002', 'pressrel_july02', 'pr409',
                    'sp250712.pdf', '272639', 'presidentofindia.gov.in/shri-', 'eparlib.nic.in.3167', 'photo-gallery.htm?')
# URL patterns of responses generated per request, cache-busting queries or session state: never a recorded identity.
PER_REQUEST_PATTERNS = ('cb=', 'reviewcb', 'form_build_id', 'token=', 'X-Amz-', 'Signature=', 'sessionid', 'ASPSESSION',
                        '__VIEWSTATE', 'error.aspx', 'ajax_page_data', 'view-dom-id', 'boomerang')
# Dossier ids renamed or withdrawn by the checks; none may remain in the packet or its extracts.
STALE_IDS = ('in_venkataraman_signs_presidents_act_19900110', 'in_gazette_ext_p2s1_no4_19900110',
             'in_eci_presidential_election_notification_19920610', 'in_kalam_assumption_of_office_notified_20020725',
             'in_patil_sworn_in_by_chief_justice_caption_20070725', 'in_pranab_ceases_ls_member_on_assuming_office_20120725',
             'in_rb_states_murmu_assumed_office_20220725', 'in_rb_kovind_farewell_address_20220724')
# Event kinds of the three research parts that one vocabulary replaced; no row may use them.
OLD_KINDS = {'oath', 'oath_scheduled', 'retrospective_span', 'stated_assumption_of_office', 'assumption_of_office',
             'assumption_of_office_statement', 'assumption_of_office_recalled', 'assumption_of_office_proclaimed',
             'assumption_proclamation', 'assumption_address', 'scheduled_term_expiry_notice',
             'result_declaration_published', 'result_figures', 'oath_of_office_reported'}
REPORT = research.RESEARCH / 'india-presidents-1990-2026-15.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-15.md'


def load_rows(packet):
    rows = {}
    for sid in packet['institutions'][1]['sources']:
        source = next(s for s in packet['sources'] if s['id'] == sid)
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def president_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert [i['id'] for i in packet['institutions']] == ['in_prime_minister', INSTITUTION], \
        'the C01-11 prime-ministership, then exactly one presidency'
    office = packet['institutions'][1]
    assert office['kind'] == 'executive_institution' and office['lifecycle']['status'] == 'unknown'
    assert [r['id'] for r in office['roles']] == [PRES], 'exactly one role'
    role = office['roles'][0]
    assert (role['title'], role['kind']) == (T_PRES, 'head_of_state')
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
             if r['kind'] == 'head_of_state']
    assert heads == [PRES], 'no other head-of-state role'
    # No president is a holder of the prime-ministership, and no claim is shared between the two institutions.
    pm = packet['institutions'][0]
    assert not {h['name'] for r in pm['roles'] for h in r['holder_claims']} & set(SURNAMES), 'cross-institution holder'
    assert not set(pm['claim_ids']) & set(office['claim_ids']), 'claims shared across institutions'
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
            assert row['holder_name'] == name and row['role_title'] == T_PRES, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        attest_days = set().union(*(kinds.get(k, set()) for k in ATTEST_KINDS))
        # A start only where a source states the day office was assumed.
        assert ({holder['from']} == from_days) if holder['from'] else not from_days, (name, 'start')
        # An end only where a source states the day the office ended; no source reviewed states one.
        assert ({holder['until']} == until_days) if holder['until'] else not until_days, (name, 'end')
        if holder['attested_on']:
            assert holder['attested_on'] in attest_days, (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and (holder['from'] or holder['attested_on']) <= holder['until']
    # Result, prospective, farewell, retrospective and corroborating claims stay on the role, and no claim carries a period.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] not in HOLDER_KINDS, cid
    for cid in SPANS:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid
    for cid in office['claim_ids']:
        assert 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid


def president_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    president_rules(packet, rows)
    role = packet['institutions'][1]['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [h['until'] for h in role['holder_claims'] if h['until']] == []
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    # Distinct dated events stay distinct and keep their own days.
    for cid, (day, _kind, _obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class IndiaPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.office = cls.packet['institutions'][1]
        cls.role = cls.office['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows(cls.packet)
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'India'}, {'India': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (56, 84))
        pm_sources = self.packet['institutions'][0]['sources']
        self.assertEqual(pm_sources, c01_11.NEW_SOURCES)
        self.assertEqual([s['id'] for s in self.packet['sources']], list(ORIGINAL_SOURCES) + pm_sources + NEW_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (84, 2))
        self.assertEqual(len(self.packet['organizations']), 82)
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER)), (30, 54))
        self.assertEqual(set(EVENTS), set(self.new_claims))
        # Every new claim and source is cited by the presidency and its one role, and by no organization and not by the
        # prime-ministership.
        self.assertEqual(self.office['claim_ids'], self.new_claims)
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.office['sources'], NEW_SOURCES)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        for entry in self.packet['organizations'] + self.packet['institutions'][:1]:
            self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
            self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
            for role in entry['roles']:
                self.assertFalse(set(self.new_claims) & set(role['claim_ids']), role['id'])
        # At most ten observations, each reported and each carrying rows.
        observations = re.findall(r'^### (IN-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'IN-PRES-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'IN-PRES-{n:02d}' for n in range(1, 11)})
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract), stale)
        kinds = {row['event_kind'] for row in self.rows.values()}
        self.assertFalse(kinds & OLD_KINDS)
        self.assertEqual(kinds, (HOLDER_KINDS - UNTIL_KINDS) | NEVER_KINDS)

    def test_holders_are_exactly_as_intended(self):
        president_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected_sources = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected_sources:
                    expected_sources.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
        self.assertEqual(list(self.office), ['id', 'name', 'kind', 'represented_party_ids', 'reconciled_organization_id',
                                             'lifecycle', 'roles', 'sources', 'claim_ids', 'coverage'])
        self.assertEqual((self.office['name'], self.office['represented_party_ids']), ('President of India (office)', []))
        self.assertIsNone(self.office['reconciled_organization_id'])
        self.assertEqual(self.office['lifecycle'], {
            'status': 'unknown', 'from': None, 'until': None,
            'note': 'These observations do not establish founding, dissolution, legal continuity or exact terms of the institution.'})
        self.assertEqual(self.office['coverage']['status'], 'partial')
        self.assertEqual(self.office['coverage']['period'], {'from': '1990-01-01', 'through': '2026-09-07'})
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        for cid in NEVER_HOLDER:
            self.assertIn(self.rows[cid]['event_kind'], NEVER_KINDS, cid)
        for cid in self.new_claims:
            self.assertEqual((self.rows[cid]['role_title'], self.rows[cid]['role_id']), (T_PRES, PRES), cid)
        # Only the two election schedules name no one; every other row names the President concerned.
        self.assertEqual({cid for cid in self.new_claims if self.rows[cid]['holder_name'] is None},
                         {'in_eci_appoints_presidential_election_dates_19920610',
                          'in_eci_appoints_presidential_election_dates_19970609'})
        self.assertEqual({self.rows[cid]['holder_name'] for cid in self.new_claims} - {None}, set(SURNAMES))
        # The stacked C01-11 prime-minister holders are unchanged.
        pm_role = self.packet['institutions'][0]['roles'][0]
        self.assertEqual([(h['name'], h['attested_on'], h['from'], h['until']) for h in pm_role['holder_claims']],
                         c01_11.HOLDERS)
        self.assertEqual([h['claim_ids'] for h in pm_role['holder_claims']], c01_11.HOLDER_CLAIMS)

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims, rows = self.claims, self.rows
        holders = self.role['holder_claims']
        starts = {h['name']: h['from'] for h in holders}
        for cid, row in rows.items():
            if row['event_kind'] in FROM_KINDS:
                # A stated assumption: the Ministry of Home Affairs resolution ('takes ... seat'), its proclamation
                # ('has entered upon the said Office'), or a same-day address or profile on assumption of office.
                self.assertRegex(claims[cid]['text'], r'takes (his|her) seat|entered upon|assum', cid)
                self.assertEqual(row['attested_on'], starts[row['holder_name']], cid)
        for holder in holders:
            self.assertIsNone(holder['until'], holder['name'])
            self.assertTrue(re.search(r'No (start and no )?end', holder['uncertainty']), holder['name'])
            self.assertNotRegex(holder['uncertainty'] + holder['note'], r'(?i)until the \d+ \w+ \d{4} (swearing-in|oath)')
        self.assertFalse({row['event_kind'] for row in rows.values()} & UNTIL_KINDS)
        self.assertIn('earliest 1990 attestation pinned to a byte-stable response', holders[0]['note'])
        # Prospective expiry notices, farewells and stylings as former President are never ends.
        for cid in PROSPECTIVE:
            if rows[cid]['event_kind'] == 'term_expiry_scheduled':
                self.assertIn('due to expire', claims[cid]['text'], cid)
                self.assertIn('Prospective', claims[cid]['uncertainty'], cid)
            else:
                self.assertEqual(rows[cid]['event_kind'], 'assumption_ceremony_scheduled', cid)
                self.assertRegex(claims[cid]['text'], r'(?i)will take place', cid)
        self.assertEqual(sum(rows[cid]['event_kind'] == 'term_expiry_scheduled' for cid in PROSPECTIVE), 7)
        for cid in ENDINGS:
            self.assertRegex(claims[cid]['uncertainty'], r'no day on which (the|his) office ended|does not state the day',
                             cid)
        for cid in SPANS:
            self.assertEqual(rows[cid]['event_kind'], 'retrospective_term_span', cid)
            self.assertIn('no structured date is stored', claims[cid]['uncertainty'], cid)
        for cid in ('in_patil_swearing_in_recalled_caption_20070725', 'in_pp_profile_patil_assumed_office_recalled_20070725',
                    'in_rb_murmu_assumption_recalled_20220725'):
            self.assertIn('dated by the day it recalls', claims[cid]['uncertainty'], cid)
        self.assertIn("member's statement",
                      claims['in_narayanan_assumption_recalled_in_lok_sabha_19970725']['uncertainty'])
        # A constitution text never supplies a date: the Lok Sabha seat notice is corroboration only.
        self.assertIn('procedure only, never a date', claims['in_pranab_ls_membership_ceased_wef_20120725']['uncertainty'])
        for cid in ('in_mha_corrects_kalam_hindi_name_20020813', 'in_kovind_name_corrigendum_20170804'):
            self.assertIn('changes no date', claims[cid]['uncertainty'], cid)
        # The handoff's separation guard: declaration, oath and stated assumption stay distinct claims, kinds and days.
        for year, (declaration, oaths, assumptions) in SEPARATION.items():
            self.assertEqual(rows[declaration]['event_kind'], 'result_declaration', year)
            self.assertTrue(declaration.endswith(str(year)[:4]) or str(year) in declaration, year)
            for cid in oaths:
                self.assertEqual(rows[cid]['event_kind'], 'oath_of_office', cid)
            for cid in assumptions:
                self.assertIn(rows[cid]['event_kind'], FROM_KINDS, cid)
            self.assertEqual(len({declaration, *oaths, *assumptions}), 1 + len(oaths) + len(assumptions))
            self.assertLess(rows[declaration]['attested_on'], rows[assumptions[0]]['attested_on'])
            self.assertEqual({rows[cid]['attested_on'] for cid in oaths + assumptions}, {f'{year}-07-25'})
            self.assertNotIn(declaration, [c for ids_ in HOLDER_CLAIMS for c in ids_])
        # Oaths that name the Chief Justice name him in the claim.
        for cid, justice in (('in_cji_kania_administers_oath_to_sharma_19920725', 'Kania'),
                             ('in_kovind_oath_administered_by_cji_khehar_20170725', 'Khehar'),
                             ('in_murmu_oath_administered_by_cji_ramana_20220725', 'Ramana')):
            self.assertIn(justice, claims[cid]['text'], cid)
        # A Vice-President acting as President would be claims only: no holder is anyone but the eight Presidents.
        scope = self.role['scope_note']
        for text in ('acting as President would be claims only, never a holder', 'procedure only, never a date',
                     "a successor's assumption of office are never an end", 'no structured date is stored'):
            self.assertIn(text, scope)
        unresolved = self.office['coverage']['unresolved']
        self.assertTrue(unresolved[0].startswith('Presidents 1990-2026 (CLAUDE-C01-15'))
        self.assertTrue(unresolved[1].startswith('Stated starts:'))
        self.assertIn('Stated ends: none.', unresolved[1])
        self.assertTrue(unresolved[-1].startswith('Keep the head of state distinct from the head of government'))
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-15' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][-1].startswith('Presidents 1990-2026 (CLAUDE-C01-15)'))
        self.assertEqual(len(coverage['unresolved']), 9)
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-09-24', sid)
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('same byte count and SHA-256', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-15 only', extract['bounded_scope'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/india-'))
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
                self.assertEqual((row['observation_id'], row['role_id']), (INSTITUTION, PRES))
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'])
                          for cid, row in self.rows.items()}, EVENTS)
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
        for sid in MIRRORS:
            url = urlsplit(self.sources[sid]['url'])
            self.assertEqual((url.hostname, url.path.split('/')[1]), ('archive.org', 'download'), sid)
            self.assertTrue(self.extracts[sid]['mirror_of'], sid)
            self.assertIn('Internet Archive item', self.extracts[sid]['provenance_note'])
            self.assertNotIn('original_url', self.sources[sid])
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED and sid not in MIRRORS]
        self.assertEqual((len(ARCHIVED), len(MIRRORS), len(others)), (21, 6, 29))
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others}, OFFICIAL_HOSTS)
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
            self.assertIn("Official file served by the publisher's own host", self.extracts[sid]['provenance_note'])
        for sid in NEW_SOURCES:
            self.assertNotIn('source_response_content_encoding', self.extracts[sid])
        # Mirror-only parliamentary records say so.
        for sid in ('in_ls_jpi_199209', 'in_ls_debates_19970725'):
            self.assertIn('Mirror-only provenance', self.sources[sid]['scope_note'])

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for pattern in PER_REQUEST_PATTERNS:
                self.assertNotIn(pattern, url, url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if parts.hostname == 'web.archive.org':
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
            else:
                self.assertIn(parts.hostname, OFFICIAL_HOSTS | {'archive.org'}, url)
                # Stored files only: no query string, and the one presidentofindia.gov.in identity is a static PDF
                # attachment, never a per-request Drupal page.
                self.assertEqual(parts.query, '', url)
                self.assertTrue(parts.path.endswith('.pdf'), url)
                if parts.hostname == 'www.presidentofindia.gov.in':
                    self.assertRegex(parts.path, r'^/files/\d{4}-\d{2}/[A-Za-z0-9_-]+\.pdf$', url)
                    self.assertEqual(sid, 'in_rb_murmu_independence_day_address_20260814')
        # The live, per-request Drupal and ASP.NET pages are recorded only through fixed pre-cutoff archive captures.
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertNotRegex(url, r'^https://(www\.)?(presidentofindia\.nic\.in|pib\.|pmindia|archive\.pib|eci\.)', sid)
            if re.search(r'presidentofindia\.nic\.in|pib\.(nic|gov)\.in|pratibhapatil', url):
                self.assertIn(sid, ARCHIVED)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'businesstoday', 'nbcnews', 'encyclopaedia britannica', 'fbis'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('E-0576-1990-0004-24040', 'E-0718-1987-0382-31802', 'ramnathkovind', 'relid=85462',
                       'presidential_elec2002', 'PRID=1844557', 'mypresidentialye', 'wikipedia'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'india.json').read_bytes()
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
            return packet['institutions'][1]['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def cite(index, cid, **dates):
            def change(packet):
                holder(packet, index).update(dates)
                holder(packet, index)['claim_ids'].append(cid)
                sid = self.claim_source[cid]
                if sid not in holder(packet, index)['sources']:
                    holder(packet, index)['sources'].append(sid)
            return change

        validator_cases = [
            (lambda p: source(p, 'in_gazette_ext_so2318_assumption_20170725')['snapshot'].update(sha256='0' * 64),
             'checksum mismatch'),
            (lambda p: source(p, 'in_ls_jpi_199209')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'in_rb_photo_gallery_20220725')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 7).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 7).update({'from': '2026-09-08'}), 'exceeds cutoff'),
            (lambda p: holder(p, 0).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'in_murmu_independence_day_eve_address_20260814').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'in_rb_span_narayanan_19970725_20020725').update(
                period={'from': '1997-07-25', 'through': '2026-09-30'}), 'exceeds cutoff'),
            (lambda p: holder(p, 6).update(until='2017-01-01'), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('in_patil_takes_seat_as_president_20070725'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('in_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def extra_holder(name, day, cid):
            return {'name': name, 'attested_on': day, 'from': None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        def move_to_pm(packet):
            packet['institutions'][0]['roles'][0]['holder_claims'].append(role(packet)['holder_claims'].pop(7))

        rule_cases = [
            # A successor's assumption of office used as an end.
            ('successor start used as an end (Venkataraman)', lambda p: holder(p, 0).update(until='1992-07-25')),
            ('successor start cited as an end (Venkataraman)',
             cite(0, 'in_sharma_takes_seat_as_president_19920725', until='1992-07-25')),
            ('successor start used as an end (Sharma)', lambda p: holder(p, 1).update(until='1997-07-25')),
            ('successor start used as an end (Narayanan)', lambda p: holder(p, 2).update(until='2002-07-25')),
            ('successor start used as an end (Kalam)', lambda p: holder(p, 3).update(until='2007-07-25')),
            ('successor start used as an end (Patil)', lambda p: holder(p, 4).update(until='2012-07-25')),
            ('successor start used as an end (Mukherjee)', lambda p: holder(p, 5).update(until='2017-07-25')),
            ('successor oath cited as an end (Kovind)',
             cite(6, 'in_murmu_oath_administered_by_cji_ramana_20220725', until='2022-07-25')),
            # A prospective expiry, a farewell, a styling as former President or a retrospective span used as an end.
            ('term due to expire used as an end (Venkataraman)',
             cite(0, 'in_eci_venkataraman_term_due_to_expire_19920610', until='1992-07-24')),
            ('term due to expire used as an end (Kovind)', lambda p: holder(p, 6).update(until='2022-07-24')),
            ('farewell used as an end (Kovind)', cite(6, 'in_kovind_farewell_address_20220724', until='2022-07-24')),
            ('eve of demitting office used as an end (Mukherjee)',
             cite(5, 'in_pranab_on_eve_of_demitting_office_20170724', until='2017-07-24')),
            ('styling as former President used as an end (Mukherjee)',
             cite(5, 'in_pranab_styled_former_president_20170725', until='2017-07-25')),
            ('styling as ex-President used as an end (Narayanan)',
             cite(2, 'in_narayanan_styled_ex_president_20020725', until='2002-07-25')),
            ('retrospective span used as an end (Venkataraman)',
             cite(0, 'in_rb_span_venkataraman_19870725_19920725', until='1992-07-25')),
            ('retrospective span used as a start (Venkataraman)',
             lambda p: holder(p, 0).update({'attested_on': None, 'from': '1987-07-25'})),
            ('an end invented at the cutoff (Murmu)', lambda p: holder(p, 7).update(until='2026-09-07')),
            # An election, designation, publication or programme date used as a start.
            ('declaration used as a start (Sharma)', lambda p: holder(p, 1).update({'from': '1992-07-16'})),
            ('declaration cited as a start (Murmu)',
             cite(7, 'in_ro_declares_murmu_elected_20220721', **{'from': '2022-07-21'})),
            ('declaration used as a start (Kalam)', lambda p: holder(p, 3).update({'from': '2002-07-18'})),
            ('Gazette publication used as a start (Narayanan)', lambda p: holder(p, 2).update({'from': '1997-07-22'})),
            ('ceremony programme used as a start (Patil)', lambda p: holder(p, 4).update({'from': '2007-07-24'})),
            ('election schedule used as an observation (Venkataraman)',
             lambda p: holder(p, 0).update(attested_on='1992-06-10')),
            ('declaration cited by a holder (Kovind)', cite(6, 'in_ro_declares_kovind_elected_20170720')),
            ('ceremony programme cited by a holder (Mukherjee)', cite(5, 'in_mha_schedules_pranab_assumption_ceremony_20120724')),
            # Corroboration, recollections and reports that never feed a holder.
            ('Lok Sabha seat notice cited by a holder (Mukherjee)', cite(5, 'in_pranab_ls_membership_ceased_wef_20120725')),
            ('corrigendum recital cited by a holder (Kalam)',
             cite(3, 'in_kalam_assumption_recited_in_corrigendum_20020725')),
            ('recollection cited by a holder (Murmu)', cite(7, 'in_rb_murmu_assumption_recalled_20220725')),
            ('later caption cited by a holder (Patil)', cite(4, 'in_patil_swearing_in_recalled_caption_20070725')),
            ("member's statement cited by a holder (Narayanan)",
             cite(2, 'in_narayanan_assumption_recalled_in_lok_sabha_19970725')),
            ('PMO oath report cited by a holder (Murmu)', cite(7, 'in_pmo_reports_murmu_took_oath_20220725')),
            ('name corrigendum used as an observation (Kovind)',
             cite(6, 'in_kovind_name_corrigendum_20170804')),
            # Acting, interim or continuation service added as a holder.
            ('Vice-President acting as President added as a holder', lambda p: role(p)['holder_claims'].insert(
                5, extra_holder('Vice-President (acting President)', '2012-07-24',
                                'in_mha_schedules_pranab_assumption_ceremony_20120724'))),
            ('service on the eve of demitting office added as a holder (Mukherjee)', lambda p: role(p)['holder_claims']
             .insert(6, extra_holder(M, '2017-07-24', 'in_pranab_on_eve_of_demitting_office_20170724'))),
            ('farewell service added as a holder (Venkataraman)', lambda p: role(p)['holder_claims'].insert(
                1, extra_holder(V, '1992-07-21', 'in_mps_farewell_to_president_venkataraman_19920721'))),
            ('continuation after an expiry notice added as a holder (Kovind)', lambda p: role(p)['holder_claims'].insert(
                7, extra_holder(RK, '2022-06-15', 'in_eci_kovind_term_due_to_expire_20220615'))),
            # Cross-role and cross-institution holders.
            ('prime minister added as a president', lambda p: role(p)['holder_claims'].append(
                extra_holder('Narendra Modi', '2024-06-09', 'in_modi_appointed_pm_communique_20240609'))),
            ('prime-ministership claim cited by a president', cite(7, 'in_murmu_appoints_modi_pm_20240607')),
            ('president moved into the prime-ministership', move_to_pm),
            ('second head-of-state role on the prime-ministership', lambda p: p['institutions'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='in_president_2'))),
            ('head of state moved onto an organization', lambda p: p['organizations'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='in_president_org'))),
            ('second role', lambda p: p['institutions'][1]['roles'].append(dict(copy.deepcopy(role(p)), id='in_pres_2'))),
            ('second presidency', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][1]),
                                                                          id='in_presidency_2'))),
            ('span given a structured date', lambda p: claim(p, 'in_rb_span_sharma_19920725_19970725').update(
                attested_on='1997-07-25')),
            ('span stored as a structured period', lambda p: claim(p, 'in_jpi_span_venkataraman_19870725_19920725').update(
                attested_period={'from': '1987-07-25', 'through': '1992-07-25'})),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        president_rules(self.packet, self.rows)
        president_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    president_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError)):
                    president_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('in_sharma_assumption_ceremony_notified_19920724', '1992-07-25'),
                         ('in_ro_declares_murmu_elected_20220721', '2022-07-22'),
                         ('in_kovind_name_corrigendum_20170804', '2017-07-25'),
                         ('in_eci_kovind_term_due_to_expire_20220615', '2022-07-24'),
                         ('in_pranab_ls_membership_ceased_wef_20120725', '2012-07-30')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                president_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted in part', '02': 'Accepted', '03': 'Accepted in part', '04': 'Accepted in part',
                     '05': 'Accepted in part', '06': 'Accepted', '07': 'Accepted', '08': 'Accepted', '09': 'Unresolved',
                     '10': 'Accepted'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| IN-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 11)] + [f'B{n}' for n in range(1, 11)] + [f'C{n}' for n in range(1, 14)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal|Declined|Already recorded)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('f755794c', '538920f1', 'claude/c01-in-11', 'research-index.json', 'test_india_research_s10e.py',
                     'test_india_prime_ministers_c01_11.py', 'test_campaign_census'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'india-presidents-1990-2026-15.md', 'claude/c01-in-15', 'f755794c', '538920f1',
                     'test_india_presidents_c01_15.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (2, 2))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
