"""CLAUDE-C01-20: the Presidents of the Indian National Congress, 1990-2026, are one party role on the Indian National
Congress recognition observation, kept apart from the prime-ministership and the presidency both ways. Working Committee
decisions, AICC sessions, the Central Election Authority's notices and declarations, certificates, handovers,
resignations, farewells and the 2019-2022 interim arrangement stay separate claims, and every holder is a dated in-office
observation with no start or end, because no record reviewed states one."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_india_presidents_c01_15 as c01_15
import test_india_prime_ministers_c01_11 as c01_11

ORG = 'in_eci_20240323_np_05'
ROLE = 'in_inc_president'
TITLE = 'President of the Indian National Congress'
ORIGINAL_SOURCES = ('in_eci_national_parties_20240323', 'in_eci_state_parties_20240323')
REVIEW = [f'INC-PRES-{n:02d}' for n in range(1, 11)]

# Original response identity recorded in each extract: (bytes, sha256), in packet order. Every source is reproducible.
RESPONSES = {
    'in_rs_debate_19900829_sri_lanka':
        (293232, 'abaf31c2b040f0787605fa89f95e1ed4ee04a2c35d9011538d420597976f154d'),
    'in_rs_debate_19910304_surveillance':
        (398978, '208694149484595dd6462bf594700bd28dba7ee655f1a92cd61778bf15d2c0fb'),
    'in_rs_debate_19910305_surveillance_rajiv_gandhi':
        (1005620, '1f49c918c2aa76d53c74b23059ebaceac2fae27b371bb83061e7f3fffa9c5a84'),
    'in_inc_our_inspiration_rajiv_gandhi_20260805':
        (14404, 'e4a91eef61a6de615233f1e6032ac8246eeeb85e48cc68da73b626b7016fc17b'),
    'in_rs_debate_19910603_resolution_demise_rajiv_gandhi':
        (2225446, 'dcead9b072e2b12c44fd177df0be5a2aea4770d42b7ab82b57581c2c03147baf'),
    'in_rs_debate_19910604_security_discussion':
        (3611707, 'fdc07bbb07a5a012af474dfa66f889221e3af0a611b3bb382c8be261c030b968'),
    'in_rs_debate_19910731_budget':
        (5172112, '1461215dbfad67d3f857f75dd6e5a73263030715e193aec44441bc41551ccff3'),
    'in_ls_bio_pv_narasimha_rao_xi_lok_sabha_20141024':
        (4656, 'f57efb1c73aee41e683ff11c9fca52394ba51fde5de46cf0f818b2930888f25d'),
    'in_inc_past_president_pv_narasimha_rao_20221026':
        (1833, '297ee3cf049ae4914061883fdc2ae6880f7b903be583caf5a0be8bc21eca7bc3'),
    'in_inc_sessions_20260827':
        (3402, 'cf9de49af546df2e32f786a61642b76628731697a164d4d97e27f39a1b89aadc'),
    'in_rs_debate_19960715_remarks_against_congress_president':
        (120669, 'd9811540716d1ab6deb6f94b2a9aedcdbeef0e45e2be6756eac6cbb881469cd9'),
    'in_rs_debate_19960910_ministry_of_power':
        (623098, '6908fa9a830e2d2239af3e923ade37819833d1274ab5789a2ed9b0da94a7fcdd'),
    'in_aicc_timeline_20040606':
        (166957, '2b9f6e3b9110ae945a78d9fd1b0b01f140c4c54421d82274c1b8b5f093197f7a'),
    'in_inc_brief_history_1995_2005_20260731':
        (18108, 'e27b1786a0c26d8712c8293b97addb905e646c346eb825011eef0f3789844c6b'),
    'in_rs_member_sketches_k_20101005':
        (133305, '553d0fb78bbdcec1a726bd63a056063a65c8b810bf630d1d39307e37345bd6c0'),
    'in_ls_debates_19970411_premchandran':
        (20221, '33460a2efbe4972c878f24e56c8063a78608364ad88c7760b27119395eca6074'),
    'in_ls_debates_19970411_deve_gowda_reply':
        (20136, '75c34c45abbfec683275a548ac6466d044aa250a426e0165b5a478c376f63140'),
    'in_inc_past_president_sitaram_kesri_20221026':
        (849, 'a66372ee49ab30d9e577c2cea6fe17ffbb6b1820bd48b94fb54f983b4fa64eeb'),
    'in_inc_site_sonia_aicc_opening_remarks_19980406':
        (22666, '4c8dd4d2beafcbceb69488f4fa2d19308a8047129d0d4685f23f1bdaf06777d4'),
    'in_inc_site_congress_president_speech_index_2000':
        (6440, '60ca897738afcbe2c0b5b1bedd614c0e7079ea897638adb41816720206a66a48'),
    'in_inc_site_sonia_resignation_letter_19990515':
        (2082, '82e395f2eabb63814864b5135f573d23eb51921757756f758e9fe5f3cbc5d64e'),
    'in_inc_site_sonia_withdrawal_speech_19990525':
        (10341, 'debdf6e5a6e0235365b522dd2e9d810f08219c66bd1b825df86b26340bf0665a'),
    'in_inc_site_president_profile_2002':
        (22366, '6a5dcaa1c5bd9458839025465558cfed6f5a66828edd181862bada10b907c04d'),
    'in_congress_sandesh_directory_congress_president_2001':
        (13191, '2852d9ccfec222f22e7d31787fe78c23dbf683074b1c82f189415cff86239d76'),
    'in_inc_history_years_in_opposition_2015':
        (75804, 'b36e6bd67188882c982b22f3f50b427ce035027cc23d96330159a93b3a1ba958'),
    'in_inc_past_president_page_sonia_gandhi_20251230':
        (12523, 'd104e80a0785b5fd5d098ffb033b9ac1f53c9c576c2daf57a764d562f94e604b'),
    'in_congress_sandesh_nov2000_news_diary':
        (23965, '22a2938180a8de41d4bb87dc5569999ceacbaf00c99b7ab1d73827a1354943d1'),
    'in_congress_sandesh_nov2000_sonia_elected':
        (5352, '7036af0956c2ecac42570f37a6cabf8242cb27752430b718c511f461a2d102e4'),
    'in_congress_sandesh_dec2000_reports':
        (24398, '914f28a0604de001c14c85749d98659289a2090b397400aff627a651d7655102'),
    'in_congress_sandesh_jan2001_foundation_day_resolution':
        (8968, '52a6a7c4d0ae776bb88786fa20278eaf2347591b6ab7a4393676cdab79560b06'),
    'in_aicc_site_sonia_elected_president_20050528':
        (15410, '58f3e954518f202ee009f3d1afd799e8a1fca4ed05cca267c686b928b61972bd'),
    'in_aicc_site_sonia_unanimously_elected_20100903':
        (10930, '1fc311ed716897652484711111c0d7ab45c567e3a8da729eb38942e07a6b848c'),
    'in_aicc_certificate_of_election_congress_president_20100903':
        (324847, 'b350d70ca8a99c754a37417c88c4598815e75fba6b854035b5198f0718cb8f66'),
    'in_aicc_press_release_cea_composition_20170429':
        (67047, 'e6066095de4333887c24562f5b8e332cc7c4fea5158b428de023aade71ca5497'),
    'in_inc_cp_remarks_cwc_20170606':
        (117199, '3d185619a2e6923c0cb9db2b270532c0fd255692e79552c12f23be92377467c3'),
    'in_aicc_cwc_press_briefing_highlights_20170606':
        (175499, '4d828befaa9158518e330df029fd04b41f7676f5fe51629139ec507f2bad02cb'),
    'in_cea_organisational_elections_programme_20170606':
        (172872, '7c7380da01f4784e2534fe762a55d8eb476ba8eb4fed9d19b4bd8ee655d6adb8'),
    'in_cea_schedule_congress_president_election_20171120':
        (389967, 'a132096435c0774be23decf1297cede662f9a012e4e2e50b430efebc171ec6ab'),
    'in_cea_declaration_of_result_rahul_gandhi_20171211':
        (503514, '24a35772e0392ce0e2d60ca09e4bba6ddb974311f69ec80973d15ea7e6bb794a'),
    'in_aicc_cea_press_briefing_highlights_20171211':
        (76410, '5f1e0a24ca72e60c4a5a4abb00891ac3fd16e6339ba0dfeb943452a47372c783'),
    'in_aicc_highlights_president_elect_address_20171213':
        (177313, 'a3ea8d42d1580c23ffa05043470c1c8053ed550ca3c68801528edaaa0bd438fa'),
    'in_aicc_transcript_rahul_gandhi_speech_20171216':
        (91142, '644636ca14afcc3ca36cf23587642f3ad07a52f41a2833b15143271d5c2aa675'),
    'in_aicc_sonia_gandhi_outgoing_president_speech_20171216':
        (404244, '71f0307bbda5d9d7174bf5d5feb505f8de4b9272cb14b680a1d561eb9a80d7de'),
    'in_aicc_cwc_resolution_outgoing_president_20171222':
        (593188, '2f146cb795bd48a79c5cc79d939a09edd88b93c6cc0607d5a5fd3506d531bc99'),
    'in_inc_cp_opening_remarks_cwc_20171222':
        (60430, '8f05c0f26fd9e9255e34a89140586415c73a720024b5e0fdc72faf0326eb2771'),
    'in_inc_past_president_page_rahul_gandhi_20260727':
        (14648, 'f5a65218daa8bebaaf3e6930973717317a0c9cdd3c4de4c4954756dd7a3d404e'),
    'in_inc_cwc_resolution_20190525':
        (86805, '7752337145be583e62f8b1c1b8421ae50e55d5de5c389a0ec6c3d7080f9260a9'),
    'in_inc_press_release_listing_20190704':
        (119961, '400dd780d4ee1f34d5380f159f598e478d4ca783f6669cfef54c8c345487f3f4'),
    'in_inc_rahul_gandhi_statement_page_20190704':
        (40720, 'bc3e0706fcb4837968993929eed5bc83e9a6a1432a795772cdc49dde33d756c4'),
    'in_inc_cwc_media_byte_20190810':
        (179349, '19b69b4f24f150e420427eb12f30b5a5364bb480191de958aa93b4663787a9fc'),
    'in_inc_cwc_three_resolutions_20190810':
        (122977, '8038e45d57bfbf8bb5a6c49e0f414171c2922cde58f889a48e69db99a435053c'),
    'in_inc_cwc_evening_briefing_20190810':
        (197813, 'eec3eb165f5d996474a22db1f4f1f8db1d0416f0a22a87be6c48f1cfb27ff36f'),
    'in_inc_cp_speech_rajiv75_20190822':
        (164283, '83030903d3539cd2db32272623b31db291fac71fc48e1816a198564cd7325c04'),
    'in_inc_cwc_resolution_20200824':
        (191780, 'f1da0b6d2e0d73eca1b11e15a0d955f6ddf8adc4be751debe45bd7bb706b891d'),
    'in_inc_cwc_briefing_20200824':
        (216700, '45c5945dc3f58578f32352ae46f07d0a594e23723ca1b06b7d690969374c4e43'),
    'in_inc_cp_opening_remarks_cwc_20211016':
        (67874, '78db3be74602d99fb33715f97f2b2c4499c9b18928df3d8e81b8d352ffdfce55'),
    'in_inc_cwc_briefing_20211016':
        (232932, 'de388317ca148b30854f6b9609eb5de2ff93058cd95556bb7d1b8071a683b935'),
    'in_inc_cwc_briefing_20220828':
        (540492, 'b49484dd8ecd782d42e49a4f26aa11af205d3afa9f48df844cb2fffb193d7727'),
    'in_inc_cea_notification_20220922':
        (99159, 'bbda539eb2a4aaf762bc53cc26f2b71285bf312393d63c8abd13c48e04d8b803'),
    'in_inc_cea_media_bite_20221001':
        (426255, '30c62a88f620f46144cce63ce540e1de389fab632bd7c30c48d6dfa9a9324be7'),
    'in_inc_cea_briefing_20221008':
        (543641, '4450f67fa438eb30ddb769c339393a06ddf7e7e554fdf7ea3d63ce1cc74f76dd'),
    'in_inc_cea_briefing_20221017':
        (589150, '39db4153ccef9d96a03d5d1656dc8c7cdba22c41767a9ca7d9e977e9feaec53b'),
    'in_inc_cea_declaration_of_result_20221019':
        (116341, '8dd6c3fa5d32d12a76b96942bba38a07ab29d7d7e7d871cdaacb30d70890599e'),
    'in_inc_cea_chairman_briefing_20221019':
        (164451, 'dbc069acf2aa41c7b6a990b8c8cdc28b75b8871177809f7ae2f86b6196b14b8b'),
    'in_inc_president_elect_briefing_20221019':
        (159916, 'c4af13ab79e5a34ac983a1e9dc0e49e26dac17aeacd67e99acebd9a2f923ec9c'),
    'in_inc_maken_resolution_of_thanks_20221026':
        (416720, 'd75822ea2815b48b9dd170ca1aa761370177129cd5069aec0c791b77c2d7f048'),
    'in_inc_sonia_gandhi_speech_20221026':
        (397733, '7010df9ad90986bedcd14b97f767f6b6c23b0509069ceac19717b457ab637514'),
    'in_inc_kharge_address_20221026':
        (483792, '42c3b7196a5f9b041fbc145a8bf49174a6a6fa7d789bb570bd41e2e3a920a10b'),
    'in_inc_venugopal_address_20221026':
        (306332, '7bbd827a9a747544104ff70b5d85b594fcabe7a01f52457a9662b3d371400d41'),
    'in_inc_steering_committee_release_20221026':
        (2204253, '05ddf3adf84f7ab709045b9f7bf2d9c029e8321ca8e8ee94e777216036f7b278'),
    'in_inc_plenary_appreciation_resolution_20230225':
        (136762, '15b3c1e89c2ad3f55221ed81cba7c125e7ae79483bd9215a48cb8bc6241fbecf'),
    'in_inc_cp_media_bite_20260903':
        (452587, '372f5daa0ee8465a705c278658932a6712feb31537fd94f225dd6167807062c2'),
    'in_inc_dcc_odisha_release_20260907':
        (195798, 'b82a80812d47a8317db5c154b2c7ef660c34a33cc810c9561098eb1731cfce6b'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (id_ form), with their capture timestamps; all before the cutoff.
ARCHIVED = {
    'in_inc_our_inspiration_rajiv_gandhi_20260805': '20260805063316',
    'in_ls_bio_pv_narasimha_rao_xi_lok_sabha_20141024': '20141024095328',
    'in_inc_past_president_pv_narasimha_rao_20221026': '20221026054027',
    'in_inc_sessions_20260827': '20260827182834',
    'in_aicc_timeline_20040606': '20040606060953',
    'in_inc_brief_history_1995_2005_20260731': '20260731211640',
    'in_rs_member_sketches_k_20101005': '20101005160003',
    'in_ls_debates_19970411_premchandran': '20090411021147',
    'in_ls_debates_19970411_deve_gowda_reply': '20090411035934',
    'in_inc_past_president_sitaram_kesri_20221026': '20221026054009',
    'in_inc_site_sonia_aicc_opening_remarks_19980406': '20000919174701',
    'in_inc_site_congress_president_speech_index_2000': '20000823071618',
    'in_inc_site_sonia_resignation_letter_19990515': '20000919174547',
    'in_inc_site_sonia_withdrawal_speech_19990525': '20000919174542',
    'in_inc_site_president_profile_2002': '20021217173146',
    'in_congress_sandesh_directory_congress_president_2001': '20020301080401',
    'in_inc_history_years_in_opposition_2015': '20150109222022',
    'in_inc_past_president_page_sonia_gandhi_20251230': '20251230142802',
    'in_congress_sandesh_nov2000_news_diary': '20040121061136',
    'in_congress_sandesh_nov2000_sonia_elected': '20030908214817',
    'in_congress_sandesh_dec2000_reports': '20030505120610',
    'in_congress_sandesh_jan2001_foundation_day_resolution': '20030708083003',
    'in_aicc_site_sonia_elected_president_20050528': '20051027141716',
    'in_aicc_site_sonia_unanimously_elected_20100903': '20101206171524',
    'in_aicc_certificate_of_election_congress_president_20100903': '20101207011339',
    'in_inc_past_president_page_rahul_gandhi_20260727': '20260727145441',
    'in_inc_press_release_listing_20190704': '20190704080621',
    'in_inc_rahul_gandhi_statement_page_20190704': '20190704065137',
}
# Captures the archive stores and serves compressed: (content encoding, decoded bytes, decoded sha256).
ENCODED = {
    'in_inc_our_inspiration_rajiv_gandhi_20260805':
        ('gzip', 80039, 'b98fa15f76fe5b58ee09be613901d16c156a0d700e7bf4cf73b41fbfa0a3369b'),
    'in_inc_sessions_20260827':
        ('gzip', 22387, '91e175cabce0eaeec8adecae1e80015db52852ad186bed86274f81671dce3cc5'),
    'in_inc_brief_history_1995_2005_20260731':
        ('zstd', 98190, 'cbcbd20d41eddcf26197d31a013f28c0f458324672aa10a8ec24118b549ecc53'),
    'in_inc_past_president_page_sonia_gandhi_20251230':
        ('gzip', 67209, 'feca879777e88427e3182c7ba656c4d3f4affd00e33db0a6d2bb30863f473863'),
    'in_inc_past_president_page_rahul_gandhi_20260727':
        ('gzip', 77102, 'c084479fb5b823c9925d6aa066a8d6320f14db965b42bb35a581a2d15062770f'),
}
# Files on the Rajya Sabha Secretariat's debates store and on the Indian National Congress's own media store.
STORE = ('in_rs_debate_19900829_sri_lanka',
         'in_rs_debate_19910304_surveillance',
         'in_rs_debate_19910305_surveillance_rajiv_gandhi',
         'in_rs_debate_19910603_resolution_demise_rajiv_gandhi',
         'in_rs_debate_19910604_security_discussion',
         'in_rs_debate_19910731_budget',
         'in_rs_debate_19960715_remarks_against_congress_president',
         'in_rs_debate_19960910_ministry_of_power')
CLOUDINARY = ('in_aicc_press_release_cea_composition_20170429',
              'in_inc_cp_remarks_cwc_20170606',
              'in_aicc_cwc_press_briefing_highlights_20170606',
              'in_cea_organisational_elections_programme_20170606',
              'in_cea_schedule_congress_president_election_20171120',
              'in_cea_declaration_of_result_rahul_gandhi_20171211',
              'in_aicc_cea_press_briefing_highlights_20171211',
              'in_aicc_highlights_president_elect_address_20171213',
              'in_aicc_transcript_rahul_gandhi_speech_20171216',
              'in_aicc_sonia_gandhi_outgoing_president_speech_20171216',
              'in_aicc_cwc_resolution_outgoing_president_20171222',
              'in_inc_cp_opening_remarks_cwc_20171222',
              'in_inc_cwc_resolution_20190525',
              'in_inc_cwc_media_byte_20190810',
              'in_inc_cwc_three_resolutions_20190810',
              'in_inc_cwc_evening_briefing_20190810',
              'in_inc_cp_speech_rajiv75_20190822',
              'in_inc_cwc_resolution_20200824',
              'in_inc_cwc_briefing_20200824',
              'in_inc_cp_opening_remarks_cwc_20211016',
              'in_inc_cwc_briefing_20211016',
              'in_inc_cwc_briefing_20220828',
              'in_inc_cea_notification_20220922',
              'in_inc_cea_media_bite_20221001',
              'in_inc_cea_briefing_20221008',
              'in_inc_cea_briefing_20221017',
              'in_inc_cea_declaration_of_result_20221019',
              'in_inc_cea_chairman_briefing_20221019',
              'in_inc_president_elect_briefing_20221019',
              'in_inc_maken_resolution_of_thanks_20221026',
              'in_inc_sonia_gandhi_speech_20221026',
              'in_inc_kharge_address_20221026',
              'in_inc_venugopal_address_20221026',
              'in_inc_steering_committee_release_20221026',
              'in_inc_plenary_appreciation_resolution_20230225',
              'in_inc_cp_media_bite_20260903',
              'in_inc_dcc_odisha_release_20260907')
# PDF pages rendered or read (one-based), per source.
PDF_PAGES = {
    'in_rs_debate_19900829_sri_lanka': [1, 17],
    'in_rs_debate_19910304_surveillance': [1],
    'in_rs_debate_19910305_surveillance_rajiv_gandhi': [12, 13],
    'in_rs_debate_19910603_resolution_demise_rajiv_gandhi': [1, 5, 19],
    'in_rs_debate_19910604_security_discussion': [4, 18, 28, 54, 55, 60],
    'in_rs_debate_19910731_budget': [22],
    'in_rs_debate_19960715_remarks_against_congress_president': [1, 2, 12],
    'in_rs_debate_19960910_ministry_of_power': [14],
    'in_rs_member_sketches_k_20101005': [8],
    'in_aicc_press_release_cea_composition_20170429': [1],
    'in_inc_cp_remarks_cwc_20170606': [1, 2, 3],
    'in_aicc_cwc_press_briefing_highlights_20170606': [1, 2, 3],
    'in_cea_organisational_elections_programme_20170606': [1, 2],
    'in_cea_schedule_congress_president_election_20171120': [1],
    'in_aicc_cea_press_briefing_highlights_20171211': [1, 2, 3, 4],
    'in_aicc_highlights_president_elect_address_20171213': [1, 2, 3],
    'in_aicc_transcript_rahul_gandhi_speech_20171216': [1, 2, 3, 4],
    'in_aicc_sonia_gandhi_outgoing_president_speech_20171216': [1, 3, 4, 8],
    'in_aicc_cwc_resolution_outgoing_president_20171222': [1, 2, 3],
    'in_inc_cp_opening_remarks_cwc_20171222': [1, 2],
    'in_inc_cwc_resolution_20190525': [1, 2, 3],
    'in_inc_cwc_media_byte_20190810': [1, 2, 3, 4],
    'in_inc_cwc_three_resolutions_20190810': [1, 2, 3],
    'in_inc_cwc_evening_briefing_20190810': [1, 2, 3, 4],
    'in_inc_cp_speech_rajiv75_20190822': [1],
    'in_inc_cwc_resolution_20200824': [1, 2, 4],
    'in_inc_cwc_briefing_20200824': [1, 2],
    'in_inc_cp_opening_remarks_cwc_20211016': [1, 2],
    'in_inc_cwc_briefing_20211016': [1, 2, 3],
    'in_inc_cwc_briefing_20220828': [1, 2],
    'in_inc_cea_media_bite_20221001': [1],
    'in_inc_cea_briefing_20221008': [1, 2],
    'in_inc_cea_briefing_20221017': [1, 2],
    'in_inc_cea_chairman_briefing_20221019': [1, 2, 3],
    'in_inc_president_elect_briefing_20221019': [1, 2, 3],
    'in_inc_maken_resolution_of_thanks_20221026': [1, 2],
    'in_inc_sonia_gandhi_speech_20221026': [1, 2],
    'in_inc_kharge_address_20221026': [1, 2, 3, 4, 5],
    'in_inc_venugopal_address_20221026': [1, 2],
    'in_inc_steering_committee_release_20221026': [1, 2, 3],
    'in_inc_plenary_appreciation_resolution_20230225': [1, 2],
    'in_inc_cp_media_bite_20260903': [1],
    'in_inc_dcc_odisha_release_20260907': [1],
}
# Every new claim's (attested_on, event_kind, review observation, holder_name), exactly: distinct dated events are never
# re-dated, relabelled, moved to another observation or given another holder.
EVENTS = {
    'in_rs_natarajan_rajiv_gandhi_congress_president_19900829':
        ('1990-08-29', 'in_office_attestation', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_heading_surveillance_on_congress_president_rajiv_gandhi_19910304':
        ('1991-03-04', 'in_office_continuation_attestation', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_goswami_rajiv_gandhi_congress_president_19910305':
        ('1991-03-05', 'in_office_continuation_attestation', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_inc_inspiration_rajiv_gandhi_death_19910521':
        ('1991-05-21', 'death_stated', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_chairman_resolution_demise_rajiv_gandhi_19910521':
        ('1991-05-21', 'death_stated', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_leader_of_house_rajiv_gandhi_president_congress_party_19910603':
        ('1991-06-03', 'posthumous_reference', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_arora_rajiv_gandhi_president_inc_i_19910603':
        ('1991-06-03', 'posthumous_reference', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_chavan_rajiv_gandhi_was_congress_president_19910604':
        ('1991-06-04', 'posthumous_reference', 'INC-PRES-01', 'Rajiv Gandhi'),
    'in_rs_pm_letter_from_unnamed_congress_president_19910604':
        ('1991-06-04', 'unnamed_holder_attestation', 'INC-PRES-01', None),
    'in_rs_bhattacharjee_recalls_rao_became_congress_president_1991':
        (None, 'selection_recalled_in_debate', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_ls_bio_span_rao_inc_president_19910529_199609':
        (None, 'retrospective_term_span', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_inc_bio_rao_presided_tirupati_session_1992':
        (None, 'retrospective_biography_statement', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_inc_sessions_79th_tirupati_rao_1992':
        (None, 'session_presided_listed', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_inc_sessions_special_surajkund_rao_1993':
        (None, 'session_presided_listed', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_inc_sessions_special_new_delhi_rao_1994':
        (None, 'session_presided_listed', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_inc_sessions_80th_calcutta_kesri_1997':
        (None, 'session_presided_listed', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_rs_gujral_rao_president_of_congress_party_19960715':
        ('1996-07-15', 'in_office_attestation', 'INC-PRES-02', 'P. V. Narasimha Rao'),
    'in_rs_narayanasamy_rao_our_congress_president_19960910':
        ('1996-09-10', 'in_office_continuation_attestation', 'INC-PRES-03', 'P. V. Narasimha Rao'),
    'in_aicc_timeline_rao_resigns_presidentship_1996':
        (None, 'resignation_recalled_retrospective', 'INC-PRES-09', 'P. V. Narasimha Rao'),
    'in_aicc_timeline_kesri_chosen_president_1996':
        (None, 'selection_recalled_retrospective', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_inc_history_rao_resigned_presidentship_1996':
        (None, 'resignation_recalled_retrospective', 'INC-PRES-09', 'P. V. Narasimha Rao'),
    'in_inc_history_kesri_elected_president_1996':
        (None, 'selection_recalled_retrospective', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_rs_sketch_span_kesri_inc_president_1996_98':
        (None, 'retrospective_term_span', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_ls_premchandran_kesri_cwc_president_19970411':
        ('1997-04-11', 'in_office_attestation', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_ls_deve_gowda_recalls_kesri_became_president_1997':
        (None, 'selection_recalled_in_debate', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_inc_bio_kesri_presided_calcutta_session_1997':
        (None, 'retrospective_biography_statement', 'INC-PRES-03', 'Sitaram Kesri'),
    'in_aicc_session_remarks_you_have_elected_me_president_19980406':
        ('1998-04-06', 'aicc_session_election_statement', 'INC-PRES-04', None),
    'in_aicc_session_remarks_accepts_office_19980406':
        ('1998-04-06', 'election_acceptance_statement', 'INC-PRES-04', None),
    'in_inc_speech_index_aicc_opening_remarks_19980406':
        ('1998-04-06', 'speech_index_entry', 'INC-PRES-04', None),
    'in_inc_speech_index_resignation_letter_19990515':
        ('1999-05-15', 'resignation_tendered', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_inc_speech_index_withdrawal_of_resignation_19990525':
        ('1999-05-25', 'resignation_withdrawn_speech', 'INC-PRES-09', None),
    'in_inc_site_heading_congress_president_sonia_gandhi_19990515':
        ('1999-05-15', 'in_office_attestation', 'INC-PRES-04', 'Sonia Gandhi'),
    'in_sonia_resignation_letter_to_cwc_19990515':
        ('1999-05-15', 'resignation_tendered', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_sonia_withdrawal_of_resignation_speech_19990525':
        ('1999-05-25', 'resignation_withdrawn_speech', 'INC-PRES-09', None),
    'in_inc_profile_span_sonia_president_from_march_1998':
        (None, 'retrospective_term_span', 'INC-PRES-04', 'Sonia Gandhi'),
    'in_sandesh_directory_sonia_elected_president_april_1998':
        (None, 'selection_recalled_retrospective', 'INC-PRES-04', 'Sonia Gandhi'),
    'in_inc_history_sonia_ratified_at_aicc_session_19980406':
        ('1998-04-06', 'ratification_recalled_retrospective', 'INC-PRES-04', 'Sonia Gandhi'),
    'in_inc_history_sonia_resigned_after_letter_1999':
        (None, 'resignation_recalled_retrospective', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_inc_history_cwc_reiterated_faith_1999':
        (None, 'working_committee_decision_recalled_retrospective', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_inc_past_president_page_sonia_became_president_april_1998':
        (None, 'selection_recalled_retrospective', 'INC-PRES-04', 'Sonia Gandhi'),
    'in_cea_mirdha_reschedules_president_election_20001005':
        ('2000-10-05', 'election_schedule', 'INC-PRES-05', None),
    'in_sonia_elected_congress_president_20001115':
        ('2000-11-15', 'election_result', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_cea_mirdha_declares_sonia_president_certificate_20001115':
        ('2000-11-15', 'result_declaration', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_sonia_converts_cwc_into_steering_committee_20001126':
        ('2000-11-26', 'in_office_attestation', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_foundation_day_resolution_sonia_re_elected_20001228':
        ('2000-12-28', 'in_office_continuation_attestation', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_sonia_elected_congress_president_20050528':
        ('2005-05-28', 'election_result', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_cea_fernandes_declares_sonia_duly_elected_20050528':
        ('2005-05-28', 'result_declaration', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_sonia_unanimously_elected_congress_president_20100903':
        ('2010-09-03', 'election_result', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_cea_fernandes_declares_and_hands_certificate_20100903':
        ('2010-09-03', 'result_declaration', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_certificate_sonia_duly_elected_unopposed_20100903':
        ('2010-09-03', 'certificate_of_election', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_congress_president_sonia_approves_cea_composition_20170429':
        ('2017-04-29', 'in_office_attestation', 'INC-PRES-05', 'Sonia Gandhi'),
    'in_congress_president_places_schedule_before_cwc_20170606':
        ('2017-06-06', 'election_schedule_placed_before_working_committee', 'INC-PRES-06', None),
    'in_cwc_approves_org_election_schedule_20170606':
        ('2017-06-06', 'working_committee_schedule_approval', 'INC-PRES-06', None),
    'in_cea_programme_congress_president_election_window_20170606':
        ('2017-06-06', 'election_schedule', 'INC-PRES-06', None),
    'in_cwc_approves_congress_president_election_schedule_20171120':
        ('2017-11-20', 'working_committee_schedule_approval', 'INC-PRES-06', None),
    'in_cea_declares_rahul_gandhi_elected_president_20171211':
        ('2017-12-11', 'result_declaration', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_cea_briefing_rahul_declared_elected_20171211':
        ('2017-12-11', 'result_declaration_reported', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_cea_briefing_certificate_handover_scheduled_20171211':
        ('2017-12-11', 'certificate_presentation_scheduled', 'INC-PRES-06', None),
    'in_cea_briefing_sonia_styled_congress_president_20171211':
        ('2017-12-11', 'in_office_continuation_attestation', 'INC-PRES-06', 'Sonia Gandhi'),
    'in_rahul_gandhi_styled_president_elect_20171213':
        ('2017-12-13', 'president_elect_styling', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_certificate_presented_to_newly_elected_president_20171216':
        ('2017-12-16', 'certificate_presentation', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_rahul_gandhi_accepts_position_20171216':
        ('2017-12-16', 'election_acceptance_statement', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_rahul_gandhi_styled_new_president_20171216':
        ('2017-12-16', 'in_office_attestation', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_sonia_last_address_as_congress_president_20171216':
        ('2017-12-16', 'farewell_address', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_cwc_resolution_thanks_outgoing_president_sonia_20171222':
        ('2017-12-22', 'predecessor_reference', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_rahul_gandhi_chairs_cwc_as_congress_president_20171222':
        ('2017-12-22', 'in_office_continuation_attestation', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_rahul_gandhi_thanks_former_congress_president_20171222':
        ('2017-12-22', 'predecessor_reference', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_inc_rahul_gandhi_assumption_recalled_20171216':
        ('2017-12-16', 'assumption_recalled_retrospective', 'INC-PRES-06', 'Rahul Gandhi'),
    'in_inc_rahul_gandhi_stepped_down_recalled_201905':
        (None, 'resignation_recalled_retrospective', 'INC-PRES-09', 'Rahul Gandhi'),
    'in_rahul_gandhi_offers_resignation_cwc_20190525':
        ('2019-05-25', 'resignation_offered', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_cwc_rejects_resignation_offer_20190525':
        ('2019-05-25', 'working_committee_resignation_rejected', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_rahul_gandhi_attested_congress_president_20190525':
        ('2019-05-25', 'in_office_attestation', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_inc_lists_rahul_gandhi_resignation_statement_20190703':
        ('2019-07-03', 'resignation_statement_published', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_rahul_gandhi_states_he_has_resigned_2019':
        (None, 'resignation_statement', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_cwc_members_ask_rahul_gandhi_to_continue_20190810':
        ('2019-08-10', 'working_committee_request_to_continue', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_rahul_gandhi_says_resignation_decision_final_20190810':
        ('2019-08-10', 'resignation_maintained', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_resignation_still_under_cwc_consideration_20190810':
        ('2019-08-10', 'resignation_pending', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_rahul_gandhi_styled_congress_president_20190810':
        ('2019-08-10', 'in_office_continuation_attestation', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_cwc_resolves_rahul_gandhi_continue_20190810':
        ('2019-08-10', 'working_committee_decision_continue', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_rahul_gandhi_declines_to_withdraw_resignation_20190810':
        ('2019-08-10', 'resignation_maintained', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_cwc_requests_sonia_gandhi_interim_president_20190810':
        ('2019-08-10', 'interim_president_requested', 'INC-PRES-07', 'Sonia Gandhi'),
    'in_cwc_first_resolution_records_stepping_down_20190810':
        ('2019-08-10', 'resignation_recorded', 'INC-PRES-07', 'Rahul Gandhi'),
    'in_sonia_gandhi_accepts_interim_request_20190810':
        ('2019-08-10', 'interim_request_accepted', 'INC-PRES-07', 'Sonia Gandhi'),
    'in_sonia_gandhi_styled_president_inc_20190822':
        ('2019-08-22', 'interim_service_attestation', 'INC-PRES-07', 'Sonia Gandhi'),
    'in_cwc_requests_sonia_gandhi_continue_20200824':
        ('2020-08-24', 'interim_continuation_requested', 'INC-PRES-07', 'Sonia Gandhi'),
    'in_cwc_resolution_read_out_continue_20200824':
        ('2020-08-24', 'interim_continuation_requested', 'INC-PRES-07', 'Sonia Gandhi'),
    'in_sonia_gandhi_interim_congress_president_self_description_20211016':
        ('2021-10-16', 'interim_service_attestation', 'INC-PRES-07', 'Sonia Gandhi'),
    'in_cwc_approves_schedule_aicc_president_election_20211016':
        ('2021-10-16', 'working_committee_schedule_approval', 'INC-PRES-08', None),
    'in_cwc_approves_final_schedule_20220828':
        ('2022-08-28', 'working_committee_schedule_approval', 'INC-PRES-08', None),
    'in_cea_notification_congress_president_election_20220922':
        ('2022-09-22', 'election_notification', 'INC-PRES-08', None),
    'in_cea_scrutiny_two_candidates_20221001':
        ('2022-10-01', 'nomination_scrutiny', 'INC-PRES-08', None),
    'in_cea_final_list_two_candidates_20221008':
        ('2022-10-08', 'final_candidate_list', 'INC-PRES-08', None),
    'in_cea_poll_held_20221017':
        ('2022-10-17', 'election_poll', 'INC-PRES-08', None),
    'in_cea_declaration_recites_poll_20221017':
        ('2022-10-17', 'election_poll_recited', 'INC-PRES-08', None),
    'in_inc_president_count_20221019':
        ('2022-10-19', 'election_count', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_cea_declares_kharge_elected_president_20221019':
        ('2022-10-19', 'result_declaration', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_mistry_announces_kharge_elected_briefing_20221019':
        ('2022-10-19', 'result_declaration_reported', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_certificate_to_be_given_20221019':
        ('2022-10-19', 'certificate_presentation_announced', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_kharge_styled_president_elect_20221019':
        ('2022-10-19', 'president_elect_styling', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_kharge_charge_on_26th_announced_20221019':
        ('2022-10-19', 'assumption_announced_prospective', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_sonia_gandhi_handing_over_post_20221026':
        ('2022-10-26', 'handover_statement', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_kharge_receives_post_of_president_20221026':
        ('2022-10-26', 'handover_statement', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_sonia_gandhi_relieved_of_responsibility_today_20221026':
        ('2022-10-26', 'interim_service_end_stated', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_sonia_gandhi_says_responsibility_now_on_kharge_20221026':
        ('2022-10-26', 'handover_statement', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_kharge_styled_president_inc_20221026':
        ('2022-10-26', 'in_office_attestation', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_kharge_says_reached_this_post_today_20221026':
        ('2022-10-26', 'handover_statement', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_venugopal_greets_congress_president_kharge_20221026':
        ('2022-10-26', 'in_office_attestation', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_venugopal_outgoing_president_sonia_gandhi_20221026':
        ('2022-10-26', 'predecessor_reference', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_congress_president_constitutes_steering_committee_20221026':
        ('2022-10-26', 'unnamed_president_act', 'INC-PRES-08', None),
    'in_steering_committee_list_kharge_congress_president_20221026':
        ('2022-10-26', 'in_office_attestation', 'INC-PRES-08', 'Mallikarjun Kharge'),
    'in_plenary_resolution_former_president_sonia_gandhi_20230225':
        ('2023-02-25', 'plenary_resolution_retrospective', 'INC-PRES-09', 'Sonia Gandhi'),
    'in_kharge_congress_president_attested_20260903':
        ('2026-09-03', 'in_office_attestation', 'INC-PRES-10', 'Mallikarjun Kharge'),
    'in_congress_president_approves_dcc_odisha_20260907':
        ('2026-09-07', 'unnamed_president_act', 'INC-PRES-10', None),
}
ELECTIONS = ('in_aicc_session_remarks_you_have_elected_me_president_19980406',
             'in_aicc_session_remarks_accepts_office_19980406',
             'in_inc_speech_index_aicc_opening_remarks_19980406',
             'in_cea_mirdha_reschedules_president_election_20001005',
             'in_sonia_elected_congress_president_20001115',
             'in_cea_mirdha_declares_sonia_president_certificate_20001115',
             'in_sonia_elected_congress_president_20050528',
             'in_cea_fernandes_declares_sonia_duly_elected_20050528',
             'in_sonia_unanimously_elected_congress_president_20100903',
             'in_cea_fernandes_declares_and_hands_certificate_20100903',
             'in_certificate_sonia_duly_elected_unopposed_20100903',
             'in_congress_president_places_schedule_before_cwc_20170606',
             'in_cwc_approves_org_election_schedule_20170606',
             'in_cea_programme_congress_president_election_window_20170606',
             'in_cwc_approves_congress_president_election_schedule_20171120',
             'in_cea_declares_rahul_gandhi_elected_president_20171211',
             'in_cea_briefing_rahul_declared_elected_20171211',
             'in_cea_briefing_certificate_handover_scheduled_20171211',
             'in_rahul_gandhi_styled_president_elect_20171213',
             'in_certificate_presented_to_newly_elected_president_20171216',
             'in_rahul_gandhi_accepts_position_20171216',
             'in_cwc_approves_schedule_aicc_president_election_20211016',
             'in_cwc_approves_final_schedule_20220828',
             'in_cea_notification_congress_president_election_20220922',
             'in_cea_scrutiny_two_candidates_20221001',
             'in_cea_final_list_two_candidates_20221008',
             'in_cea_poll_held_20221017',
             'in_cea_declaration_recites_poll_20221017',
             'in_inc_president_count_20221019',
             'in_cea_declares_kharge_elected_president_20221019',
             'in_mistry_announces_kharge_elected_briefing_20221019',
             'in_certificate_to_be_given_20221019',
             'in_kharge_styled_president_elect_20221019',
             'in_kharge_charge_on_26th_announced_20221019')
ENDINGS = ('in_inc_inspiration_rajiv_gandhi_death_19910521',
           'in_rs_chairman_resolution_demise_rajiv_gandhi_19910521',
           'in_rs_leader_of_house_rajiv_gandhi_president_congress_party_19910603',
           'in_rs_arora_rajiv_gandhi_president_inc_i_19910603',
           'in_rs_chavan_rajiv_gandhi_was_congress_president_19910604',
           'in_aicc_timeline_rao_resigns_presidentship_1996',
           'in_inc_history_rao_resigned_presidentship_1996',
           'in_inc_speech_index_resignation_letter_19990515',
           'in_inc_speech_index_withdrawal_of_resignation_19990525',
           'in_sonia_resignation_letter_to_cwc_19990515',
           'in_sonia_withdrawal_of_resignation_speech_19990525',
           'in_inc_history_sonia_resigned_after_letter_1999',
           'in_inc_history_cwc_reiterated_faith_1999',
           'in_sonia_last_address_as_congress_president_20171216',
           'in_cwc_resolution_thanks_outgoing_president_sonia_20171222',
           'in_rahul_gandhi_thanks_former_congress_president_20171222',
           'in_inc_rahul_gandhi_stepped_down_recalled_201905',
           'in_rahul_gandhi_offers_resignation_cwc_20190525',
           'in_cwc_rejects_resignation_offer_20190525',
           'in_inc_lists_rahul_gandhi_resignation_statement_20190703',
           'in_rahul_gandhi_states_he_has_resigned_2019',
           'in_cwc_members_ask_rahul_gandhi_to_continue_20190810',
           'in_rahul_gandhi_says_resignation_decision_final_20190810',
           'in_resignation_still_under_cwc_consideration_20190810',
           'in_cwc_resolves_rahul_gandhi_continue_20190810',
           'in_rahul_gandhi_declines_to_withdraw_resignation_20190810',
           'in_cwc_first_resolution_records_stepping_down_20190810',
           'in_sonia_gandhi_handing_over_post_20221026',
           'in_kharge_receives_post_of_president_20221026',
           'in_sonia_gandhi_relieved_of_responsibility_today_20221026',
           'in_sonia_gandhi_says_responsibility_now_on_kharge_20221026',
           'in_kharge_says_reached_this_post_today_20221026',
           'in_venugopal_outgoing_president_sonia_gandhi_20221026',
           'in_plenary_resolution_former_president_sonia_gandhi_20230225')
INTERIM = ('in_cwc_requests_sonia_gandhi_interim_president_20190810',
           'in_sonia_gandhi_accepts_interim_request_20190810',
           'in_sonia_gandhi_styled_president_inc_20190822',
           'in_cwc_requests_sonia_gandhi_continue_20200824',
           'in_cwc_resolution_read_out_continue_20200824',
           'in_sonia_gandhi_interim_congress_president_self_description_20211016')
CONTINUATION = ('in_rs_heading_surveillance_on_congress_president_rajiv_gandhi_19910304',
                'in_rs_goswami_rajiv_gandhi_congress_president_19910305',
                'in_rs_narayanasamy_rao_our_congress_president_19960910',
                'in_foundation_day_resolution_sonia_re_elected_20001228',
                'in_cea_briefing_sonia_styled_congress_president_20171211',
                'in_rahul_gandhi_chairs_cwc_as_congress_president_20171222',
                'in_rahul_gandhi_styled_congress_president_20190810')
RETROSPECTIVE = ('in_rs_bhattacharjee_recalls_rao_became_congress_president_1991',
                 'in_ls_bio_span_rao_inc_president_19910529_199609',
                 'in_inc_bio_rao_presided_tirupati_session_1992',
                 'in_inc_sessions_79th_tirupati_rao_1992',
                 'in_inc_sessions_special_surajkund_rao_1993',
                 'in_inc_sessions_special_new_delhi_rao_1994',
                 'in_inc_sessions_80th_calcutta_kesri_1997',
                 'in_aicc_timeline_kesri_chosen_president_1996',
                 'in_inc_history_kesri_elected_president_1996',
                 'in_rs_sketch_span_kesri_inc_president_1996_98',
                 'in_ls_deve_gowda_recalls_kesri_became_president_1997',
                 'in_inc_bio_kesri_presided_calcutta_session_1997',
                 'in_inc_profile_span_sonia_president_from_march_1998',
                 'in_sandesh_directory_sonia_elected_president_april_1998',
                 'in_inc_history_sonia_ratified_at_aicc_session_19980406',
                 'in_inc_past_president_page_sonia_became_president_april_1998',
                 'in_inc_rahul_gandhi_assumption_recalled_20171216')
UNNAMED = ('in_rs_pm_letter_from_unnamed_congress_president_19910604',
           'in_congress_president_constitutes_steering_committee_20221026',
           'in_congress_president_approves_dcc_odisha_20260907')
HOLDER_OBSERVATIONS = tuple(cid for cid, e in EVENTS.items() if e[1] == 'in_office_attestation')
NEVER_HOLDER = ELECTIONS + ENDINGS + INTERIM + CONTINUATION + RETROSPECTIVE + UNNAMED
# The one event kind that may date a holder, and the kinds that would state a start or an end; no claim has either of the
# latter, and every other kind never feeds a holder.
HOLDER_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_statement'}
UNTIL_KINDS = {'end_of_office_stated'}
ELECTIONS_KINDS = {
    'aicc_session_election_statement', 'assumption_announced_prospective', 'certificate_of_election',
    'certificate_presentation', 'certificate_presentation_announced', 'certificate_presentation_scheduled',
    'election_acceptance_statement', 'election_count', 'election_notification', 'election_poll', 'election_poll_recited',
    'election_result', 'election_schedule', 'election_schedule_placed_before_working_committee', 'final_candidate_list',
    'nomination_scrutiny', 'president_elect_styling', 'result_declaration', 'result_declaration_reported',
    'speech_index_entry', 'working_committee_schedule_approval'}
ENDINGS_KINDS = {
    'death_stated', 'farewell_address', 'handover_statement', 'interim_service_end_stated',
    'plenary_resolution_retrospective', 'posthumous_reference', 'predecessor_reference', 'resignation_maintained',
    'resignation_offered', 'resignation_pending', 'resignation_recalled_retrospective', 'resignation_recorded',
    'resignation_statement', 'resignation_statement_published', 'resignation_tendered', 'resignation_withdrawn_speech',
    'working_committee_decision_continue', 'working_committee_decision_recalled_retrospective',
    'working_committee_request_to_continue', 'working_committee_resignation_rejected'}
INTERIM_KINDS = {'interim_continuation_requested', 'interim_president_requested', 'interim_request_accepted',
                 'interim_service_attestation'}
CONTINUATION_KINDS = {'in_office_continuation_attestation'}
RETROSPECTIVE_KINDS = {'assumption_recalled_retrospective', 'ratification_recalled_retrospective',
                       'retrospective_biography_statement', 'retrospective_term_span', 'selection_recalled_in_debate',
                       'selection_recalled_retrospective', 'session_presided_listed'}
UNNAMED_KINDS = {'unnamed_holder_attestation', 'unnamed_president_act'}
NEVER_KINDS = (ELECTIONS_KINDS | ENDINGS_KINDS | INTERIM_KINDS | CONTINUATION_KINDS | RETROSPECTIVE_KINDS
               | UNNAMED_KINDS)
GROUPS = ((ELECTIONS, ELECTIONS_KINDS), (ENDINGS, ENDINGS_KINDS), (INTERIM, INTERIM_KINDS),
          (CONTINUATION, CONTINUATION_KINDS), (RETROSPECTIVE, RETROSPECTIVE_KINDS), (UNNAMED, UNNAMED_KINDS))
RG, PVR, SK, SG, RAG, MK = ('Rajiv Gandhi', 'P. V. Narasimha Rao', 'Sitaram Kesri', 'Sonia Gandhi', 'Rahul Gandhi',
                            'Mallikarjun Kharge')
SURNAMES = {RG: 'Gandhi', PVR: 'Rao', SK: 'Kesri', SG: 'Gandhi', RAG: 'Gandhi', MK: 'Kharge'}

# Exact holder observations of in_inc_president: (name, attested_on, from, until), in chronological order. No source
# reviewed states the day any President assumed or left the office, so no holder has a start or an end.
HOLDERS = [
    (RG, '1990-08-29', None, None),
    (PVR, '1996-07-15', None, None),
    (SK, '1997-04-11', None, None),
    (SG, '1999-05-15', None, None),
    (SG, '2000-11-26', None, None),
    (SG, '2017-04-29', None, None),
    (RAG, '2017-12-16', None, None),
    (RAG, '2019-05-25', None, None),
    (MK, '2022-10-26', None, None),
    (MK, '2026-09-03', None, None),
]
HOLDER_CLAIMS = [
    ['in_rs_natarajan_rajiv_gandhi_congress_president_19900829'],
    ['in_rs_gujral_rao_president_of_congress_party_19960715'],
    ['in_ls_premchandran_kesri_cwc_president_19970411'],
    ['in_inc_site_heading_congress_president_sonia_gandhi_19990515'],
    ['in_sonia_converts_cwc_into_steering_committee_20001126'],
    ['in_congress_president_sonia_approves_cea_composition_20170429'],
    ['in_rahul_gandhi_styled_new_president_20171216'],
    ['in_rahul_gandhi_attested_congress_president_20190525'],
    ['in_kharge_styled_president_inc_20221026', 'in_venugopal_greets_congress_president_kharge_20221026',
     'in_steering_committee_list_kharge_congress_president_20221026'],
    ['in_kharge_congress_president_attested_20260903'],
]
HOLDER_REVIEW = ['INC-PRES-01', 'INC-PRES-02', 'INC-PRES-03', 'INC-PRES-04', 'INC-PRES-05', 'INC-PRES-05', 'INC-PRES-06',
                 'INC-PRES-07', 'INC-PRES-08', 'INC-PRES-10']
# The Working Committee's interim arrangement: Sonia Gandhi's service from its request of 10 August 2019 to the handover of
# 26 October 2022 is claims only, never a holder.
INTERIM_WINDOW = ('2019-08-10', '2022-10-26')
# Dates that are never any holder's attested_on, start or end: elections, schedules, notifications, polls, counts,
# declarations, certificates and President-elect stylings; deaths, posthumous references, resignations, withdrawals,
# farewells, handovers and predecessor references; interim service; recollections and later attestations; the
# retrospective sketch's candidate start and the dossier's rejected session-list days.
NEVER_HOLDER_DATE = sorted(
    ({e[0] for cid, e in EVENTS.items() if cid in NEVER_HOLDER and e[0]} - {h[1] for h in HOLDERS})
    | {'1991-05-29', '1992-04-14', '1997-08-08'})
# Claims printed without a structured date: spans, year- or month-only statements, undated lists and recollections.
UNDATED = tuple(cid for cid, e in EVENTS.items() if e[0] is None)
# Rows whose source names nobody: holder_name null.
NAMELESS = tuple(cid for cid, e in EVENTS.items() if e[3] is None)
# Dossier claim and source ids renamed, merged or withdrawn by the checks; none may remain as an id.
STALE_IDS = ('in_aicc_sonia_accepts_presidency_19980406', 'in_aicc_sonia_says_aicc_elected_her_president_19980406',
             'in_cea_briefing_certificate_handover_scheduled_20171216', 'in_cea_fernandes_declares_sonia_duly_elected_2005',
             'in_cea_programme_congress_president_election_window_2017', 'in_certificate_of_election_function_20221026',
             'in_cwc_converted_to_steering_committee_after_election_20001126',
             'in_inc_bio_rajiv_gandhi_party_president_at_assassination_199105', 'in_inc_president_poll_held_20221017',
             'in_inc_profile_sonia_president_march_1998_onwards', 'in_inc_rahul_gandhi_assumed_presidency_20171216',
             'in_inc_sessions_79th_tirupati_rao_19920414', 'in_inc_sessions_80th_calcutta_kesri_19970808',
             'in_inc_sessions_special_new_delhi_rao_19940610', 'in_inc_sessions_special_surajkund_rao_19930327',
             'in_ls_bio_rao_inc_president_from_19910529', 'in_ls_bio_rao_inc_president_until_199609',
             'in_rahul_gandhi_states_he_has_resigned_20190703', 'in_rs_sketch_kesri_inc_president_1996_98',
             'in_inc_release_record_certificate_function_20221026', 'in_inc_past_president_rajiv_gandhi_20221026',
             'attested_period')
# Event kinds of the three research parts that one vocabulary replaced; no row may use them.
OLD_KINDS = {'acceptance', 'acceptance_interim_president', 'aicc_ratification', 'aicc_session_address_as_president',
             'aicc_session_election_stated', 'assumption_announced', 'assumption_stated', 'attestation_member_statement',
             'attestation_unnamed_holder', 'continuation_attestation', 'continuation_attested',
             'cwc_decision_election_schedule', 'cwc_reconstitution_after_election', 'cwc_resolution',
             'death_of_holder_stated', 'election_retrospective', 'handover_stated_interim_service_end',
             'interim_service_attested', 'interim_service_end_reference', 'posthumous_attestation',
             'president_elect_attested', 'president_elect_status', 'president_exercises_office',
             'relinquishment_statement', 'resignation', 'resignation_retrospective', 'resignation_stated',
             'resignation_withdrawal', 'resolution_attestation', 'result_declaration_and_certificate',
             'result_declaration_announced', 'retrospective_attestation', 'retrospective_election_month',
             'retrospective_end_stated', 'retrospective_start_month', 'retrospective_start_stated',
             'retrospective_tenure_stated', 'selection_retrospective', 'session_presidency_listed',
             'session_presidency_retrospective', 'stated_assumption_of_office', 'working_committee_decision',
             'working_committee_decision_interim_president', 'working_committee_decision_resignation_rejected'}
# Leads, declined records, per-request pages and secondary sources: never a source URL.
LEAD_URL_MARKERS = ('wikipedia', 'eparlib.nic.in.510/', 'eparlib.nic.in.3144', 'eparlib.nic.in.3262', 'eparlib.nic.in.3517',
                    'eparlib.nic.in.10512', 'eparlib.nic.in.10908', 'eparlib.nic.in.7696', 'ID_178_15071996_01_p239-240',
                    'rsdebate.nic.in', 'rajiv-gandhi.json', '5MH7AH6akrt52ENFFCW-S', 'pdfjoiner_a4790fa70e',
                    'SHRI_RAHUL_GANDHI_879ceb7fc1', 'CWC_briefing_25_05_2019', 'PRESS_RELEASE_11_12_2017',
                    'HON_fcc708f21e', 'Shri_Mallikarjun_Kharge_02_09_2026', 'aicc-president-bio', 'elected-president1',
                    '37-Brief-Life-Sketches', 'Congress-Working-Committee-Meeting-September-8-2015', 'CongressSandesh/79',
                    'CongressSandesh/205', 'dec_2k/report4', 'rajiv_gandhi_president.htm', 'rajiv-60th-birthday',
                    'MESSAGE_BY_CONGRESS_PRESIDENT', 'INDEPENDENCE_DAY_MESSAGE', 'Jaitley_Condolence',
                    'New_Jharkhand_PCC', 'PR_AICC_GS_and_Incharges', 'brief-history-of-congress/1985-1995',
                    'sansad.in')
# URL patterns of responses generated per request, growing listings, searches, cache-busting queries or session state.
PER_REQUEST = re.compile(r'[?&](cb|_|s|q|token|sessionid)=|reviewcb|X-Amz-|Signature=|cdx/search|email-protection|cdn-cgi|'
                         r'press-releases/page/|/search|nocache|form_build_id|ASPSESSION|__VIEWSTATE', re.I)
OFFICIAL_HOSTS = {'bucketapi.rajyasabha.digital', 'res.cloudinary.com'}
REPORT = research.RESEARCH / 'india-inc-presidents-1990-2026-20.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-20.md'


def party_role(packet):
    org, = [o for o in packet['organizations'] if o['id'] == ORG]
    return org, org['roles'][0]


def load_rows(packet):
    rows = {}
    for source in packet['sources']:
        if source['id'] in RESPONSES:
            extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def inc_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = [o for o in packet['organizations'] if o['id'] == ORG]
    assert len(orgs) == 1
    org = orgs[0]
    # The recognition observation keeps its identity, lifecycle and empty game mapping.
    assert (org['name'], org['kind'], org['jurisdiction']) == ('Indian National Congress',
                                                               'political_party_national_recognition_observation', 'India')
    assert org['source_identifier']['value'] == ORG and org['recognition']['attested_on'] == '2024-03-23'
    assert org['recognition']['from'] is None and org['recognition']['until'] is None
    assert org['lifecycle'] == {'status': 'unresearched', 'from': None, 'until': None,
                                'note': 'Recognition attestation is not an organizational foundation, dissolution or '
                                        'name-change date.'}
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['coverage']['status'] == 'reporting_identity_only'
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one role on the Indian National Congress observation'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (TITLE, 'party_leader')
    leaders = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
               if r['kind'] == 'party_leader' or r['id'] == ROLE]
    assert leaders == [(ORG, ROLE)], 'no other party-leader role and no copy of this one'
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    # Party office and state office never feed each other: no claim or source is shared with an institution.
    inc_claims = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
    inc_sources = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
    for inst in packet['institutions']:
        i_claims = set(inst['claim_ids']) | {c for r in inst['roles'] for c in r['claim_ids']} | {
            c for r in inst['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        i_sources = set(inst['sources']) | {s for r in inst['roles'] for s in r['sources']} | {
            s for r in inst['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not inc_claims & i_claims and not inc_sources & i_sources, inst['id']
        for r in inst['roles']:
            assert not {h['name'] for h in r['holder_claims']} & {h['name'] for h in role['holder_claims']} - {PVR}, \
                (inst['id'], 'cross-institution holder')
    for entry in packet['organizations']:
        if entry is not org:
            assert not set(entry['claim_ids']) & inc_claims and not set(entry['sources']) & inc_sources, entry['id']
    previous = ''
    for holder in role['holder_claims']:
        name = holder['name']
        assert isinstance(holder, dict) and name in SURNAMES, name
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] >= previous, (name, 'holders stay in chronological order')
        previous = dated[0]
        assert not {holder['attested_on'], holder['from'], holder['until']} & set(NEVER_HOLDER_DATE), name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        if name == SG:
            assert not INTERIM_WINDOW[0] <= dated[0] <= INTERIM_WINDOW[1], (name, 'interim service is never a holder')
            assert not holder['until'] or not INTERIM_WINDOW[0] <= holder['until'] <= INTERIM_WINDOW[1], name
        expected, kinds = [], {}
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'], cid
            assert (row['holder_name'], row['role_id'], row['role_title']) == (name, ROLE, TITLE), (name, cid)
            assert row['observation_id'] == ORG, cid
            assert row['event_kind'] in HOLDER_KINDS | FROM_KINDS | UNTIL_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
            if claim_source[cid] not in expected:
                expected.append(claim_source[cid])
        assert holder['sources'] == expected, name
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        attest_days = set().union(*(kinds.get(k, set()) for k in HOLDER_KINDS))
        # A start only where a source states the day the office was assumed; an end only where one states the day it ended.
        assert ({holder['from']} == from_days) if holder['from'] else not from_days, (name, 'start')
        assert ({holder['until']} == until_days) if holder['until'] else not until_days, (name, 'end')
        # Each holder observation cites only in-office attestations of its own day.
        if holder['attested_on']:
            assert attest_days == {holder['attested_on']}, (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and dated[0] <= holder['until']
    # Claims that never feed a holder stay on the role with their own kinds; undated claims carry no structured date.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] in NEVER_KINDS, cid
    for cid in UNDATED:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid
    for cid in role['claim_ids']:
        assert 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid
        assert rows[cid]['review_observation'] in REVIEW, cid


def inc_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    inc_rules(packet, rows)
    _org, role = party_role(packet)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    # Distinct dated events stay distinct and keep their own days.
    for cid, (day, _kind, _obs, _name) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # The stacked prime-minister and president holders are unchanged.
    pm, presidency = packet['institutions']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pm['roles'][0]['holder_claims']] == c01_11.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in presidency['roles'][0]['holder_claims']] == c01_15.HOLDERS


class IndiaIncPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.org, cls.role = party_role(cls.packet)
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (73, 113))
        pm, presidency = self.packet['institutions']
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(ORIGINAL_SOURCES) + pm['sources'] + presidency['sources'] + NEW_SOURCES)
        self.assertEqual((pm['sources'], presidency['sources']), (c01_11.NEW_SOURCES, c01_15.NEW_SOURCES))
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (84, 3))
        self.assertEqual(len(self.packet['organizations']), 82)
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual(sorted(holder_claims), sorted(HOLDER_OBSERVATIONS))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER)), (12, 101))
        self.assertEqual(list(EVENTS), self.new_claims)
        for group, kinds in GROUPS:
            self.assertEqual({EVENTS[cid][1] for cid in group}, kinds)
        # Every new claim and source is cited by the party role and its recognition observation, and by nothing else.
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.org['claim_ids'], ['in_eci_20240323_np_row_05'] + self.new_claims)
        self.assertEqual(self.org['sources'], ['in_eci_national_parties_20240323'] + NEW_SOURCES)
        for entry in self.packet['organizations'] + self.packet['institutions']:
            if entry is not self.org:
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
                self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
                for role in entry['roles']:
                    self.assertFalse(set(self.new_claims) & set(role['claim_ids']), role['id'])
        # At most ten observations, INC-PRES-01..10, each reported and carrying rows; no row reuses a state-office number.
        observations = re.findall(r'^### (INC-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({self.rows[cid]['review_observation'] for cid in self.new_claims}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        for cid in self.new_claims:
            self.assertNotRegex(self.rows[cid]['review_observation'], r'^IN-(PRES|PM)-', cid)
        for stale in STALE_IDS:
            self.assertNotIn(f'"{stale}"', self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(f'"{stale}"', json.dumps(extract), stale)
        kinds = {self.rows[cid]['event_kind'] for cid in self.new_claims}
        self.assertFalse(kinds & OLD_KINDS)
        self.assertEqual(kinds, HOLDER_KINDS | NEVER_KINDS)

    def test_holders_are_exactly_as_intended(self):
        inc_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            self.assertRegex(holder['uncertainty'], r'No start', holder['name'])
            for cid in holder['claim_ids']:
                self.assertIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        for cid in NEVER_HOLDER:
            self.assertNotIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
        for cid in self.new_claims:
            row = self.rows[cid]
            self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), (ORG, ROLE, TITLE), cid)
            self.assertEqual(row['holder_name'], EVENTS[cid][3], cid)
        # Holder names are normalised; the printed forms stay in the claim text. Rows whose source names nobody are null.
        self.assertEqual({self.rows[cid]['holder_name'] for cid in self.new_claims} - {None}, set(SURNAMES))
        self.assertEqual(len(NAMELESS), 21)
        for cid in NAMELESS:
            self.assertIn(self.rows[cid]['event_kind'], NEVER_KINDS, cid)
        self.assertIn("'Mallikajun'", self.claims['in_cea_declares_kharge_elected_president_20221019']['uncertainty'])
        self.assertIn('Shri Mallikajun Kharge', self.claims['in_cea_declares_kharge_elected_president_20221019']['text'])
        self.assertIn("'Kesari'", self.claims['in_ls_deve_gowda_recalls_kesri_became_president_1997']['uncertainty'])
        self.assertIn('नये अध्यक्ष श्री राहुल गांधी', self.claims['in_rahul_gandhi_styled_new_president_20171216']['text'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims, rows = self.claims, self.rows
        for holder in self.role['holder_claims']:
            self.assertIsNone(holder['from'], holder['name'])
            self.assertIsNone(holder['until'], holder['name'])
        self.assertFalse({rows[cid]['event_kind'] for cid in self.new_claims} & (FROM_KINDS | UNTIL_KINDS))
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
        # Deaths, resignations, withdrawals, farewells, handovers and predecessor references are never an end, and
        # interim service is never a holder.
        for cid in ENDINGS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never an (until|end)|never a holder date|never a start', cid)
        for cid in INTERIM:
            self.assertIn('claim only, never a holder', claims[cid]['uncertainty'], cid)
        for cid in ELECTIONS + RETROSPECTIVE + UNNAMED + CONTINUATION:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never (a|dates? a) holder|never feeds|cannot feed|'
                                                         r'never a (start|boundary)|never boundaries|claims only', cid)
        for cid in ('in_kharge_receives_post_of_president_20221026', 'in_sonia_gandhi_says_responsibility_now_on_kharge_20221026',
                    'in_kharge_says_reached_this_post_today_20221026'):
            self.assertEqual(rows[cid]['event_kind'], 'handover_statement', cid)
            self.assertIn('never a start', claims[cid]['uncertainty'].lower(), cid)
        self.assertIn('karyabhar sambhala', claims['in_kharge_receives_post_of_president_20221026']['uncertainty'])
        self.assertIn('CLAUDE-C01-15 check C1', claims['in_inc_rahul_gandhi_assumption_recalled_20171216']['uncertainty'])
        self.assertIn('a death is its own claim', self.role['scope_note'])
        self.assertIn('never an until', claims['in_rs_chairman_resolution_demise_rajiv_gandhi_19910521']['uncertainty'].lower())
        self.assertIn('farewell is never an until', claims['in_sonia_last_address_as_congress_president_20171216']['uncertainty'])
        scope = self.role['scope_note']
        for phrase in ('no in_prime_minister or in_presidency claim or source feeds this role',
                       'none of its claims feeds either institution', 'Do not fill the interval',
                       "infer an outgoing holder's last day from a successor's election", 'are claims only, never holders',
                       'procedure only, never a date', 'no structured date is stored',
                       'Parliament records are used only where they record the party office'):
            self.assertIn(phrase, scope)
        self.assertTrue(self.org['coverage']['unresolved'][-1].startswith('INC Presidents 1990-2026 (CLAUDE-C01-20)'))
        self.assertEqual(self.org['coverage']['unresolved'][:-1], [
            'Reconcile exact organization identity, jurisdiction and name variants before mapping to the game.',
            'Research founding, splits, mergers, dissolution and separately dated party offices from 1990 through the fixed cutoff.',
            'Review later amendments and any identity or status qualifications; no current legal conclusion is drawn.'])
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-20' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][-1].startswith('INC Presidents 1990-2026 (CLAUDE-C01-20, INC-PRES-01..10)'))
        self.assertEqual(len(coverage['unresolved']), 10)
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])

    def test_party_and_state_offices_stay_separate(self):
        pm, presidency = self.packet['institutions']
        for inst in (pm, presidency):
            self.assertFalse(set(inst['claim_ids']) & set(self.new_claims), inst['id'])
            self.assertFalse(set(inst['sources']) & set(NEW_SOURCES), inst['id'])
            for role in inst['roles']:
                for holder in role['holder_claims']:
                    self.assertFalse(set(holder['claim_ids']) & set(self.new_claims), holder['name'])
        # The one person who held both party and government office keeps separate claims in each: the stacked
        # prime-ministership's Rao holders cite only Gazette and Parliament records of that office.
        rao_pm = [h for h in pm['roles'][0]['holder_claims'] if h['name'] == PVR]
        self.assertEqual(len(rao_pm), 1)
        self.assertFalse(set(rao_pm[0]['claim_ids']) & set(self.new_claims))
        for cid in self.new_claims:
            self.assertEqual(self.rows[cid]['role_id'], ROLE, cid)
        # The Leader of the Opposition, the Congress Parliamentary Party and the prime-ministership named in party records
        # are recorded as context only.
        self.assertIn('Rajya Sabha office is a separate parliamentary role and is not used',
                      self.claims['in_kharge_congress_president_attested_20260903']['uncertainty'])
        self.assertIn("speaker's own office belongs to the prime-ministership",
                      self.claims['in_rs_pm_letter_from_unnamed_congress_president_19910604']['uncertainty'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-09-25', sid)
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for phrase in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256'):
                self.assertIn(phrase, extract['provenance_note'], sid)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-20 only', extract['bounded_scope'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'] and source['publisher'])
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
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['holder_name'])
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
        for sid in STORE:
            url = urlsplit(self.sources[sid]['url'])
            self.assertEqual(url.hostname, 'bucketapi.rajyasabha.digital', sid)
            self.assertIn("Official file served by the publisher's own host", self.extracts[sid]['provenance_note'])
            self.assertIn('ETag equals the MD5 of the body', self.extracts[sid]['provenance_note'])
        for sid in CLOUDINARY:
            url = urlsplit(self.sources[sid]['url'])
            self.assertEqual(url.hostname, 'res.cloudinary.com', sid)
            self.assertRegex(url.path, r'^/dkplc2mbj/image/upload/v\d{10}/[A-Za-z0-9_.-]+\.(pdf|jpg)$', sid)
            note = self.extracts[sid]['provenance_note']
            self.assertIn("Indian National Congress's own media store", note)
            self.assertIn('ETag equals the MD5 of the body', note)
            self.assertIn('not an origin test', note)
        self.assertEqual((len(ARCHIVED), len(STORE), len(CLOUDINARY)), (28, 8, 37))
        self.assertEqual(set(ARCHIVED) | set(STORE) | set(CLOUDINARY), set(NEW_SOURCES))
        for sid in NEW_SOURCES:
            self.assertNotIn('mirror_of', self.extracts[sid])
            if sid not in ARCHIVED:
                self.assertNotIn('original_url', self.sources[sid])
                self.assertNotIn('archive_capture_utc', self.extracts[sid])
        for sid, (encoding, decoded_bytes, decoded_sha) in ENCODED.items():
            extract = self.extracts[sid]
            self.assertIn(sid, ARCHIVED)
            self.assertEqual(extract['source_response_content_encoding'], encoding)
            self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']),
                             (decoded_bytes, decoded_sha))
            self.assertIn(f'{encoding}-encoded', extract['provenance_note'])
            self.assertIn(decoded_sha, extract['provenance_note'])
            self.assertIn('curl without --compressed', extract['provenance_note'])
        self.assertEqual(sorted(e[0] for e in ENCODED.values()), ['gzip', 'gzip', 'gzip', 'gzip', 'zstd'])
        for sid in set(NEW_SOURCES) - set(ENCODED):
            self.assertNotIn('source_response_content_encoding', self.extracts[sid])
        # Undated pages carry their dating record as a date anchor with its own response identity.
        anchored = {sid for sid in NEW_SOURCES if 'date_anchor' in self.extracts[sid]}
        self.assertEqual(anchored, {'in_ls_debates_19970411_premchandran', 'in_ls_debates_19970411_deve_gowda_reply',
                                    'in_inc_site_sonia_resignation_letter_19990515'})
        for sid in anchored:
            anchor = self.extracts[sid]['date_anchor']
            self.assertTrue(anchor['url'].startswith('https://web.archive.org/web/'), sid)
            self.assertRegex(anchor['source_response_sha256'], r'^[0-9a-f]{64}$')
            self.assertLess(anchor['archive_capture_utc'], '2026-09-07')
        letter = self.extracts['in_inc_site_sonia_resignation_letter_19990515']['date_anchor']
        self.assertEqual((letter['source_id'], letter['source_response_bytes'], letter['source_response_sha256']),
                         ('in_inc_site_congress_president_speech_index_2000',)
                         + RESPONSES['in_inc_site_congress_president_speech_index_2000'])

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST.search(url), url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if parts.hostname == 'web.archive.org':
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
            else:
                self.assertIn(parts.hostname, OFFICIAL_HOSTS, url)
            # The Rajya Sabha store's query only sets the served content type and file name; no other URL has a query.
            if parts.hostname == 'bucketapi.rajyasabha.digital':
                self.assertEqual(sorted(k.split('=')[0] for k in parts.query.split('&')),
                                 ['response-content-disposition', 'response-content-type'])
            else:
                self.assertEqual(parts.query, '', url)
            # Live inc.in pages and build-id page data are regenerated per request: only fixed pre-cutoff captures.
            if 'inc.in/' in url or '/_next/data/' in url:
                self.assertIn(sid, ARCHIVED)
        # The one listing recorded is a fixed capture of 4 July 2019, never the live, growing listing.
        listing = self.sources['in_inc_press_release_listing_20190704']['url']
        self.assertTrue(listing.startswith('https://web.archive.org/web/20190704080621id_/'))

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources']:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'britannica', 'the hindu', 'indianexpress', 'ndtv', 'hindustantimes', 'timesofindia',
                       'news18', 'reuters', 'bbc.'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('eparlib.nic.in.510', 'eparlib.nic.in.3144', 'eparlib.nic.in.3262', 'eparlib.nic.in.3517',
                       'eparlib.nic.in.10512', 'eparlib.nic.in.10908', 'ID_178_15071996_01_p239-240',
                       'rajiv-gandhi.json', '5MH7AH6akrt52ENFFCW-S', 'pdfjoiner_a4790fa70e', 'PRESS_RELEASE_11_12_2017',
                       'wikipedia'):
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

        def org(packet):
            return party_role(packet)[0]

        def role(packet):
            return party_role(packet)[1]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def pm_role(packet):
            return packet['institutions'][0]['roles'][0]

        def president_role(packet):
            return packet['institutions'][1]['roles'][0]

        def cite(index, cid, **dates):
            def change(packet):
                holder(packet, index).update(dates)
                holder(packet, index)['claim_ids'].append(cid)
                sid = self.claim_source[cid]
                if sid not in holder(packet, index)['sources']:
                    holder(packet, index)['sources'].append(sid)
            return change

        def extra_holder(name, day, cid, until=None):
            return {'name': name, 'attested_on': day, 'from': None, 'until': until, 'sources': [self.claim_source[cid]],
                    'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        validator_cases = [
            (lambda p: source(p, 'in_inc_steering_committee_release_20221026')['snapshot'].update(sha256='0' * 64),
             'checksum mismatch'),
            (lambda p: source(p, 'in_rs_debate_19900829_sri_lanka')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'in_inc_cp_media_bite_20260903')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 9).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 9).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'in_congress_president_approves_dcc_odisha_20260907').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'in_ls_bio_span_rao_inc_president_19910529_199609').update(
                period={'from': '1991-05-29', 'through': '2026-09-30'}), 'exceeds cutoff'),
            (lambda p: holder(p, 8).update({'from': '2022-10-26', 'until': '2022-01-01'}), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('in_sonia_converts_cwc_into_steering_committee_20001126'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('in_does_not_exist'), 'Unknown'),
            (lambda p: org(p).update(represented_party_ids=['India/guessed_inc']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        rule_cases = [
            # A successor's observation or start used as an end.
            ('successor observation used as an end (Rajiv Gandhi)', lambda p: holder(p, 0).update(until='1996-07-15')),
            ('successor claim cited as an end (Rao)',
             cite(1, 'in_ls_premchandran_kesri_cwc_president_19970411', until='1997-04-11')),
            ('successor observation used as an end (Sonia Gandhi 2017)', lambda p: holder(p, 5).update(until='2017-12-16')),
            ('successor observation used as an end (Rahul Gandhi 2019)', lambda p: holder(p, 7).update(until='2022-10-26')),
            ('an end invented at the cutoff (Kharge)', lambda p: holder(p, 9).update(until='2026-09-07')),
            # A death, a resignation, a farewell or a handover used as an end.
            ('death used as an end (Rajiv Gandhi)', cite(0, 'in_rs_chairman_resolution_demise_rajiv_gandhi_19910521',
                                                        until='1991-05-21')),
            ('farewell used as an end (Sonia Gandhi)', cite(5, 'in_sonia_last_address_as_congress_president_20171216',
                                                           until='2017-12-16')),
            ('resignation used as an end (Sonia Gandhi 1999)', cite(3, 'in_sonia_resignation_letter_to_cwc_19990515',
                                                                   until='1999-05-15')),
            ('resignation statement used as an end (Rahul Gandhi)', lambda p: holder(p, 7).update(until='2019-07-03')),
            ('stepping down cited as an end (Rahul Gandhi)',
             cite(7, 'in_cwc_first_resolution_records_stepping_down_20190810', until='2019-08-10')),
            ('retrospective span used as an end (Rao)', lambda p: holder(p, 1).update(until='1996-09-30')),
            # An election, nomination, declaration or certificate date used as a start or an observation.
            ('election date used as a start (Sonia Gandhi 2000)',
             lambda p: holder(p, 4).update({'attested_on': None, 'from': '2000-11-15'})),
            ('declaration cited as a start (Rahul Gandhi 2017)',
             cite(6, 'in_cea_declares_rahul_gandhi_elected_president_20171211', **{'from': '2017-12-11'})),
            ('declaration used as a start (Kharge)', lambda p: holder(p, 8).update({'attested_on': None, 'from': '2022-10-19'})),
            ('candidate start from a retrospective sketch (Rao)',
             lambda p: holder(p, 1).update({'attested_on': None, 'from': '1991-05-29'})),
            ('recollection used as a start (Rahul Gandhi)',
             cite(6, 'in_inc_rahul_gandhi_assumption_recalled_20171216', **{'from': '2017-12-16'})),
            ('handover statement used as a start (Kharge)',
             cite(8, 'in_kharge_receives_post_of_president_20221026', **{'from': '2022-10-26'})),
            ('President-elect styling used as an observation (Kharge)', lambda p: holder(p, 8).update(attested_on='2022-10-19')),
            ('certificate used as an observation (Sonia Gandhi 2010)',
             lambda p: role(p)['holder_claims'].insert(5, extra_holder(SG, '2010-09-03',
                                                                       'in_certificate_sonia_duly_elected_unopposed_20100903'))),
            ('session list used as an observation (Rao)', lambda p: holder(p, 1).update(attested_on='1992-04-14')),
            ('election result cited by a holder (Sonia Gandhi 2000)', cite(4, 'in_sonia_elected_congress_president_20001115')),
            ('unnamed record cited by a holder (Rajiv Gandhi)',
             cite(0, 'in_rs_pm_letter_from_unnamed_congress_president_19910604')),
            ('continuation claim cited by a holder (Rahul Gandhi 2017)',
             cite(6, 'in_rahul_gandhi_chairs_cwc_as_congress_president_20171222')),
            # Acting, interim or continuation service added as a holder.
            ('interim service added as a holder (Sonia Gandhi 2019)', lambda p: role(p)['holder_claims'].insert(
                8, extra_holder(SG, '2019-08-22', 'in_sonia_gandhi_styled_president_inc_20190822'))),
            ('interim service with an end added as a holder (Sonia Gandhi 2021)', lambda p: role(p)['holder_claims'].insert(
                8, extra_holder(SG, '2021-10-16', 'in_sonia_gandhi_interim_congress_president_self_description_20211016',
                                until='2022-10-26'))),
            ('continuation attestation added as a holder (Rao 1996)', lambda p: role(p)['holder_claims'].insert(
                2, extra_holder(PVR, '1996-09-10', 'in_rs_narayanasamy_rao_our_congress_president_19960910'))),
            # Cross-role and cross-institution holders and claims.
            ('prime minister added as a party President', lambda p: role(p)['holder_claims'].append(
                extra_holder('Narendra Modi', '2024-06-09', 'in_modi_appointed_pm_communique_20240609'))),
            ('President of India added as a party President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][7]))),
            ('party President moved into the prime-ministership',
             lambda p: pm_role(p)['holder_claims'].append(role(p)['holder_claims'].pop(8))),
            ('party President added to the presidency',
             lambda p: president_role(p)['holder_claims'].append(copy.deepcopy(holder(p, 9)))),
            ('prime-ministership claim cited by a party President', cite(1, 'in_rao_appointed_pm_wef_19910621')),
            ('party claim moved onto the prime-ministership', lambda p: (
                pm_role(p)['claim_ids'].append('in_rs_gujral_rao_president_of_congress_party_19960715'),
                pm_role(p)['sources'].append('in_rs_debate_19960715_remarks_against_congress_president'))),
            ('party role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('party role copied onto an institution',
             lambda p: p['institutions'][0]['roles'].append(dict(copy.deepcopy(role(p)), id='in_inc_president_2'))),
            ('second role on the observation',
             lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='in_inc_general_secretary'))),
            ('role kind changed', lambda p: role(p).update(kind='head_of_government')),
            ('observation lifecycle given a start', lambda p: org(p)['lifecycle'].update({'from': '1885-12-28'})),
            ('observation mapped to a game party', lambda p: org(p).update(represented_party_ids=['India/inc'])),
            ('observation coverage upgraded', lambda p: org(p)['coverage'].update(status='partial')),
            # Structured dates added to undated claims, and order.
            ('span given a structured date', lambda p: claim(p, 'in_rs_sketch_span_kesri_inc_president_1996_98').update(
                attested_on='1996-09-23')),
            ('recollection given a period', lambda p: claim(p, 'in_rahul_gandhi_states_he_has_resigned_2019').update(
                attested_period={'from': '2019-07-03', 'through': '2019-07-04'})),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        inc_rules(self.packet, self.rows)
        inc_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    inc_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    inc_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('in_cea_declares_rahul_gandhi_elected_president_20171211', '2017-12-16'),
                         ('in_cea_declares_kharge_elected_president_20221019', '2022-10-26'),
                         ('in_sonia_elected_congress_president_20001115', '2000-11-26'),
                         ('in_cea_poll_held_20221017', '2022-10-19'),
                         ('in_sonia_gandhi_accepts_interim_request_20190810', '2019-08-22')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                inc_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = dict.fromkeys([f'{n:02d}' for n in range(1, 9)], 'Accepted in part')
        decisions.update({'09': 'Unresolved', '10': 'Accepted'})
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| INC-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 12)] + [f'B{n}' for n in range(1, 20)] + [f'C{n}' for n in range(1, 15)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal|Declined|Already recorded)')
        records = [line for line in defects.splitlines() if re.match(r'\| [ABC]-R\d+ ', line)]
        self.assertEqual(len(records), 30)
        for row in records:
            self.assertRegex(row, r'\*\*(Imported|Declined|Already recorded)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('d19f756b', 'dd58a610', 'claude/c01-in-15', 'claude/c01-in-11', 'research-index.json',
                     'test_india_research_s10e.py', 'test_india_prime_ministers_c01_11.py',
                     'test_india_presidents_c01_15.py', 'test_campaign_census', 'No existing extract'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'india-inc-presidents-1990-2026-20.md', 'claude/c01-in-20', 'd19f756b',
                     'claude/c01-in-15', 'dd58a610', 'test_india_inc_presidents_c01_20.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (2, 3))
        self.assertEqual(country['mapping_pending'], 84)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
