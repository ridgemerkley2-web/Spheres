"""CLAUDE-C01-27: the National Presidents of the Bharatiya Janata Party, 1990-2026, are one party role on the Bharatiya
Janata Party recognition observation, kept apart from the prime-ministership, the presidency and the Congress presidency
both ways. National Council and National Executive decisions, organisational elections, endorsements, assumptions of
charge, acting and working presidencies, extensions, handovers, resignations and farewells stay separate claims. A holder
has a start or an end only where a source states the day (three starts and one end); every other holder is a dated
in-office observation."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_india_inc_presidents_c01_20 as c01_20
import test_india_presidents_c01_15 as c01_15
import test_india_prime_ministers_c01_11 as c01_11

ORG = 'in_eci_20240323_np_03'
ROLE = 'in_bjp_president'
TITLE = 'National President of the Bharatiya Janata Party'
ORIGINAL_SOURCES = ('in_eci_national_parties_20240323', 'in_eci_state_parties_20240323')
REVIEW = [f'BJP-PRES-{n:02d}' for n in range(1, 11)]
A, MMJ, KT, BL, JK, VN, RS, NG, AS, JPN, NN = (
    'L. K. Advani', 'Murli Manohar Joshi', 'Kushabhau Thakre', 'Bangaru Laxman', 'K. Jana Krishnamurthi',
    'M. Venkaiah Naidu', 'Rajnath Singh', 'Nitin Gadkari', 'Amit Shah', 'J. P. Nadda', 'Nitin Nabin')
# The surname (or its stem, for the two printed spellings of Krishnamurthi) that every holder claim's text must carry.
SURNAMES = {A: 'Advani', MMJ: 'Joshi', KT: 'Thakre', BL: 'Laxman', JK: 'Jana Krishnamurth', VN: 'Naidu', RS: 'Rajnath',
            NG: 'Gadkari', AS: 'Shah', JPN: 'Nadda', NN: 'Nabin'}

# Original response identity recorded in each extract: (bytes, sha256), in packet order. Every source is reproducible.
RESPONSES = {
    'in_bjp_elib_party_document_vol5_political_resolutions':
        (3810049, '33ddad0dcd6f64726a041646440ef66379ff6c88cdc4ee26cf0edb85c20022f5'),
    'in_bjp_elib_party_document_vol6_economic_resolutions':
        (3111032, '043834819a685a58cf6b9ae8eb2379e3ce47c1d76a93dd7713ae470374babb07'),
    'in_rs_written_answers_19910108_usq1586_rath_yatra':
        (100324, 'df546af7447066924d4b2b83f2e82c0f74fb0a5061ab8e0649e90fb48725b9ad'),
    'in_bjp_elib_party_document_vol3_presidential_speeches_part2':
        (2822399, 'd12b2d2d86819f7ddebac2b8eb51096fbed3c931d47f3c4220342836ded59a20'),
    'in_bjp_org_advani_profile_19961019':
        (3567, '463e92e7a0ac792268fed6c6718d9e187b1385a2a823e5f0c12d21db3cf38ad0'),
    'in_bjp_org_lka_profile_biodata_20030317':
        (10760, '41036bbbe496d2fa54de7e2eaefd1eede2c5b952b9248c9c778a82d3afd96955'),
    'in_bjp_elib_party_document_vol10_evolution_of_bjp':
        (1409320, '2d33ba5938c217d67d1cbfc715a1b6b7c3d6dcb88307d090177f61ad0d79f6a4'),
    'in_rs_debate_19911210_ekta_yatra_bjp_president':
        (83271, '16d162004276da02b0ad7259d74ef8ce3cd7c210aa3787a2b6a12a2bb6a4bf23'),
    'in_rs_written_answers_19920304_usq1060_rocket_fire':
        (44815, '5c3a7881bdd28d660196c83357ff6b169badac4d3d0890092307ea6a816936e4'),
    'in_rs_written_answers_19920304_usq1067_flag_hoisting':
        (59344, 'fc4df69aa85e1287999c6c70b43b7b16be3cb1a3465fe37492db7ec747fe807f'),
    'in_bjp_elib_party_document_vol2_presidential_speeches_part1':
        (2118173, 'd6b44cbfd05642a16cb1b57959a03a21b01f384d039ad3871410c1384ff6df83'),
    'in_bjp_org_swarna_jayanti_press_release_19970716':
        (24191, '0eb50f1109f6b90d928a72f52a3870304b946a2e18f5c797e8318fdf2681918e'),
    'in_bjp_elib_party_document_vol4_foreign_policy_resolutions':
        (2538135, 'ba547ac20a962cc9bd983d1064126abecdd37577da5d20c95c12c3bfaa77729b'),
    'in_bjp_site_advani_opening_remarks_ne_19980411':
        (35726, '76916f4cc8e0f55f9d75025c5c125e89efafcf813a002e4c0e710da6fcf31904'),
    'in_bjp_site_thakre_presidential_speech_gandhinagar_199805':
        (35968, 'c4bc6c891a41fc6d49b2a7ab94e2c75cec2d30235d888cd50a8c6cf3e935f8c2'),
    'in_bjp_site_thakre_budget_statement_19990227':
        (8112, '22db19ee147876c4e31af13920e9a9513a659b8380b771eec12daa77ac0fc04e'),
    'in_bjp_site_thakre_press_statement_20000704':
        (4440, '7c5119c79ec709e9c37f248f95dc058cf14e0f5edaadd9ddf57352e3a89359da'),
    'in_bjp_site_laxman_nomination_20000802':
        (10983, '5394657a29dd4cd0984bc5614afa0ded540f80258032e6a897d41295ca7b508c'),
    'in_bjp_site_laxman_presidential_address_nagpur_200008':
        (133649, '6672c98e7612b6f292d5d1873721d166553af43e02e2406c165592a1defbb5cb'),
    'in_bjp_site_general_secretary_report_nagpur_200008':
        (44599, '5471202185e3fb71b2398bb5864a82039fba56f5a5051f9554284c73d5a18c60'),
    'in_bjp_site_laxman_statement_20000831':
        (18262, '9554a703e65331364b736c4335b42b6d28fcdb7c568647e7ffc5bd35b54f643e'),
    'in_bjp_site_laxman_statement_20000901':
        (20465, '6c94660b81ac31c8e6b5f652e9def50232360868663b8dd22d05641d86a99b33'),
    'in_bjp_site_laxman_press_statement_20010309':
        (7888, '0808ef1f2f34083bcc059c706c506533787e035bd28038c1926f441dcef4d7c4'),
    'in_bjp_site_office_bearers_press_release_20010314':
        (5816, 'b9261b7f54969d294a8aae7f75a7377adc2f9f06a72ffb30a4ff39c9f148fe5b'),
    'in_bjp_site_press_release_20010315':
        (11219, 'e8ac6e05e2a64247533c1cba2f22af2766bd8a7ca31cea12043c4dd06646ebaa'),
    'in_bjp_site_jana_krishnamurthy_press_statement_20010318':
        (10853, '15a82af6b69e49cd56f33b52592d2e1d357e87d598fb4a5510f421f29bbf063c'),
    'in_bjp_site_jana_krishnamurthy_ne_address_20010324':
        (23578, '1acdbd7ba5bf2a1cad69e15a0ddd455f8f20d7e464aacfc4c319eef7f58a5bc3'),
    'in_bjp_site_ne_resolution_political_situation_20010324':
        (16071, '2d45d7b043094324437189fb65c1867f8fc946b50f644e94fd8a34c0f7cbf01a'),
    'in_bjp_site_jana_krishnamurthi_press_statement_20010329':
        (16372, '1b9e9a4e98f3fed99028c7aa9e203c3a6abee6e669146ae66c5a253a8c59c482'),
    'in_bjp_site_shastri_press_statement_20020624':
        (5870, '94bfc48904da4d8023372709a9b53e131c2962fb3ec65027ca517fd0dd2f8037'),
    'in_bjp_site_naidu_profile_20020701':
        (22063, 'b524e4d3ee878f82f3c2699982f2438880ee7cd754059e03184b58b8f6e18b92'),
    'in_bjp_site_naidu_first_press_conference_20020711':
        (17606, 'e379da92f8bff0219ac6f529fb184fd8bb2e8c531a8ac4be126264c75a566dde'),
    'in_bjp_today_editorial_20020716':
        (8628, '121728d4ebf9617d8fb7382979a9472a8ceb00c103630b5e0f637de4aa0fd499'),
    'in_bjp_site_naidu_national_council_address_20020803':
        (86460, 'b864c13744e4a6707d14eb4fae549967a57f5d3846fda9de8521ea4b194a79ca'),
    'in_bjp_site_press_statement_naidu_resignation_20041018':
        (5685, 'fe4232cb0a40f0852fbf7aa92a995185497426a1a538ead01cb737f2edb4743b'),
    'in_bjp_site_advani_statement_20041020':
        (25103, '1129703f17c57ddc5ab0df95e16843ee1c96e9ddc48c05a4838037aa41313208'),
    'in_bjp_today_national_council_report_20041027':
        (13669, 'a92ec6f429b0e59b73cb334c0c67517bbf0ff9f71128342f8532dad00f933776'),
    'in_bjp_site_press_release_office_bearers_20041030':
        (79065, 'e48ed67a018b75fdd7ab78ea31d7bcea158edb1dac6290a06979ab0909637cd0'),
    'in_bjp_site_resolution_advani_resignation_20050608':
        (7570, '8dff1d2cd5a2afac46a0e914df6d35449309fc13c4f0e46323b6fcac24fbfa29'),
    'in_bjp_site_advani_speech_20050615':
        (24489, '356702ffa3d27c80a685290fa1f99ed69a8d6b739e36953f85c3d3f64e7c2c09'),
    'in_bjp_site_advani_opening_remarks_ne_20051226':
        (13330, 'b6f4e69e65b89f13240e43631ceb39c72c058e0792937280f7cc5b9e20bbe304'),
    'in_bjp_site_rajat_jayanti_sandesh_20051230':
        (40023, '6099088f1d715f4fb3152ee96d1d0161a0f98bbf6c1494afbc424cac6c13f316'),
    'in_bjp_site_rajnath_singh_statement_20060102':
        (8405, '01d1b4061f41be0f17163f428cae2f7a4de71e6d262dba7cab10ba646b3553bf'),
    'in_bjp_site_rajnath_singh_national_council_address_20060120':
        (70234, 'c0940aca25290fd58f8fcfec8dd634c6d03b77692dce88a0781281dd06c2be51'),
    'in_bjp_site_profile_gadkari_national_president_20091218':
        (42195, '512d377daefe9acffa8a6587f1e2d8dcb0a0e1afca7be18e5f3e7a97e59dc568'),
    'in_bjp_site_parliamentary_board_resolution_rajnath_singh_20091219':
        (27136, '96a0f6d7df42d380f0ecd28ce63406c5b0fb580457eec732af9f067191749ec1'),
    'in_bjp_site_gadkari_first_press_conference_as_president_20091224':
        (34928, '7e18342428d0969d7b935798fa834858022abf7b0dfa485012824f48b3d773e9'),
    'in_bjp_site_press_release_listing_20091228':
        (47742, 'a8499b4367337d9b36ece18ab8f0acb696d1b702b1c14fb1d7a441fb3c0a9956'),
    'in_bjp_site_gadkari_presidential_address_national_council_indore_20100218':
        (80095, 'aa6b085313f78b276cb61ca1a9101af1e8e12b4a892fa6200839ea2130580867'),
    'in_bjp_site_gadkari_announces_national_executive_20100316':
        (83228, 'af9c1cf809bf9e3642bbda3941f879a571845711639b0edbb8d6dbf3e0e09256'),
    'in_bjp_site_gadkari_statement_no_second_term_20130122':
        (82918, '6bd1f49a8a29213e5c6ca62331d7e360b7cc4736afb6d046abc81358c48f6782'),
    'in_bjp_site_national_election_officer_declaration_rajnath_singh_20130123':
        (510935, '32e7d314a50dfc1b7abd93810ebfda57f7d596121c131bc22da45b8cf92df532'),
    'in_bjp_site_rajnath_singh_acceptance_speech_newly_elected_20130123':
        (81330, 'e9a82d0f54c95d802d4c77b28f93815ff97ee2b42813cc9ec54001092557914d'),
    'in_bjp_site_gadkari_styled_former_president_20130127':
        (84724, '1f00274dcc0a43150f1bbd84e4834c2900d6ed9d4aa479abe2c8934f5026b12c'),
    'in_bjp_site_rajnath_singh_presidential_address_national_council_20130302':
        (159181, '9d046824395207416f23a1d6e0dc0579d6a0657e2b1a8346fc953c8b785c49c7'),
    'in_bjp_site_bjp_presidents_1980_2013_list_20140117':
        (60215, 'ccbb3f8afa3129fb2033480b687556250ca04c041f5108978ba90e0eb15086f3'),
    'in_bjp_site_parliamentary_board_resolution_rajnath_singh_20140709':
        (69735, '918e060f286920e258676dff45bd8ec8f1ecc3569a475e0a621e4262094ce115'),
    'in_bjp_site_profile_national_president_amit_shah_20140709':
        (74948, '561aec7e5ee5064417d238009b5549bfda3a7b02e5e86fbf4de3cece64a7084e'),
    'in_bjp_site_amit_shah_presidential_address_national_council_20140809':
        (101609, 'c886ac72423db1a97e791d787d82072cf55d6285f8d8c763dc67885ea610d1da'),
    'in_bjp_site_amit_shah_re_elected_national_president_20160124':
        (72410, '805c932deaa508f32577bd2f88eee416ee73134e1e6ada0e396407b7fb0f33ec'),
    'in_bjp_site_amit_shah_felicitation_20160202':
        (76078, 'd8104fcf252978ce78732b4833f8e462d080b4cd5fcde3505dd71a46ce0e8f86'),
    'in_ks_amit_shah_valmiki_jayanti_address_20161015':
        (18829, '68a8cd1917e184da0be0e1789062f9d031b69b6ae8e63005a56c993546e27c89'),
    'in_ks_nadda_appointed_working_president_20190617':
        (30947, '3003480b831e8a1bfeea844409a09de8c0f11242a95ec4b33dcbcec24c4025ea'),
    'in_kamal_sandesh_vol15_no03_20200201':
        (3452018, '03d5e769396cc0f7509238f2b3e16921f81ebd0df8fd63be3c30e1c41cd74c79'),
    'in_bjp_site_nadda_letter_to_karyakartas_20230117':
        (109764, '8bdac60556ae1e4b6b0bd8982fa4ae9f9a22c4366f9a7f9a2e9d86bd8d8030f9'),
    'in_kamal_sandesh_vol18_no03_20230201':
        (3893800, '86b4773c6eb826ddd38ce3668824f99a55c44849d2a61852be5513e4fdf5251a'),
    'in_ks_nadda_address_national_convention_20240217':
        (24183, '255b81f465be62d2f6870dbc70cbeafe83b1838671276397aca5bdc052ceddd6'),
    'in_bjp_site_bjp_presidents_list_20251120':
        (83494, '523a4c17c0a46fa41f27b5402adfef127a81ce76e8f370fa3621210bd0ed5978'),
    'in_kamal_sandesh_vol20_no24_20251216':
        (1855901, '4436fe02345c3cfa06f9d605f6e74bf282ee58adc71ead469325af5de66f80ad'),
    'in_bjp_site_returning_officer_notice_20260116':
        (659783, '24f5fc1b928e4ed6109c1eaa5ae37b1bd4cea6ba8e3cdc4bfc7972426cb11557'),
    'in_bjp_site_returning_officer_statement_20260119':
        (441845, '43a4cdb66469889ed9bdcbfff4befce64c8ed6fc6dd07fd810a34ef983ef264c'),
    'in_bjp_site_nadda_felicitation_speech_20260120':
        (101761, 'b0daf4fca5ef0dd95843e0308970e42e9dc469b823a510ab440c68be2778eb6c'),
    'in_kamal_sandesh_vol21_no02_20260116':
        (1853966, '37da7adcbe88a98e051bd01181d0e27d5627e7b4501b6aed44f51d6ea2c56c8a'),
    'in_ks_national_president_appoints_office_bearers_20260817':
        (87779, '9f0ee4234af85c678e14ecfc7894cb37103f602172fd6938b96061d0dd76301b'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (id_ form), with their capture timestamps; all before the cutoff.
ARCHIVED = {
    'in_bjp_org_advani_profile_19961019': '19961019165000',
    'in_bjp_org_lka_profile_biodata_20030317': '20030317090529',
    'in_bjp_org_swarna_jayanti_press_release_19970716': '19980519064759',
    'in_bjp_site_advani_opening_remarks_ne_19980411': '19990202202118',
    'in_bjp_site_thakre_presidential_speech_gandhinagar_199805': '19980519031912',
    'in_bjp_site_thakre_budget_statement_19990227': '20000930091736',
    'in_bjp_site_thakre_press_statement_20000704': '20001008183149',
    'in_bjp_site_laxman_nomination_20000802': '20001008182352',
    'in_bjp_site_laxman_presidential_address_nagpur_200008': '20010303224851',
    'in_bjp_site_general_secretary_report_nagpur_200008': '20010407083239',
    'in_bjp_site_laxman_statement_20000831': '20010304053024',
    'in_bjp_site_laxman_statement_20000901': '20001217155800',
    'in_bjp_site_laxman_press_statement_20010309': '20010407074629',
    'in_bjp_site_office_bearers_press_release_20010314': '20010407074954',
    'in_bjp_site_press_release_20010315': '20010407075430',
    'in_bjp_site_jana_krishnamurthy_press_statement_20010318': '20010407073014',
    'in_bjp_site_jana_krishnamurthy_ne_address_20010324': '20010816142640',
    'in_bjp_site_ne_resolution_political_situation_20010324': '20010407072805',
    'in_bjp_site_jana_krishnamurthi_press_statement_20010329': '20010407074928',
    'in_bjp_site_shastri_press_statement_20020624': '20030118142243',
    'in_bjp_site_naidu_profile_20020701': '20040825154555',
    'in_bjp_site_naidu_first_press_conference_20020711': '20030115222731',
    'in_bjp_today_editorial_20020716': '20021121185317',
    'in_bjp_site_naidu_national_council_address_20020803': '20021018112710',
    'in_bjp_site_press_statement_naidu_resignation_20041018': '20041214090024',
    'in_bjp_site_advani_statement_20041020': '20041216204631',
    'in_bjp_today_national_council_report_20041027': '20041222014246',
    'in_bjp_site_press_release_office_bearers_20041030': '20041101042457',
    'in_bjp_site_resolution_advani_resignation_20050608': '20070807192659',
    'in_bjp_site_advani_speech_20050615': '20061004201838',
    'in_bjp_site_advani_opening_remarks_ne_20051226': '20060628134007',
    'in_bjp_site_rajat_jayanti_sandesh_20051230': '20060618024255',
    'in_bjp_site_rajnath_singh_statement_20060102': '20061004205048',
    'in_bjp_site_rajnath_singh_national_council_address_20060120': '20061206185844',
    'in_bjp_site_profile_gadkari_national_president_20091218': '20091225011522',
    'in_bjp_site_parliamentary_board_resolution_rajnath_singh_20091219': '20091223100951',
    'in_bjp_site_gadkari_first_press_conference_as_president_20091224': '20100409085726',
    'in_bjp_site_press_release_listing_20091228': '20091228094325',
    'in_bjp_site_gadkari_presidential_address_national_council_indore_20100218': '20100221075709',
    'in_bjp_site_gadkari_announces_national_executive_20100316': '20160204070452',
    'in_bjp_site_gadkari_statement_no_second_term_20130122': '20130125005230',
    'in_bjp_site_national_election_officer_declaration_rajnath_singh_20130123': '20131021002030',
    'in_bjp_site_rajnath_singh_acceptance_speech_newly_elected_20130123': '20130202062351',
    'in_bjp_site_gadkari_styled_former_president_20130127': '20130202062841',
    'in_bjp_site_rajnath_singh_presidential_address_national_council_20130302': '20130310032553',
    'in_bjp_site_bjp_presidents_1980_2013_list_20140117': '20140117184748',
    'in_bjp_site_parliamentary_board_resolution_rajnath_singh_20140709': '20140713000334',
    'in_bjp_site_profile_national_president_amit_shah_20140709': '20140712141004',
    'in_bjp_site_amit_shah_presidential_address_national_council_20140809': '20140812204526',
    'in_bjp_site_amit_shah_re_elected_national_president_20160124': '20160125083201',
    'in_bjp_site_amit_shah_felicitation_20160202': '20160203091502',
    'in_ks_amit_shah_valmiki_jayanti_address_20161015': '20260506003147',
    'in_ks_nadda_appointed_working_president_20190617': '20251107062528',
    'in_kamal_sandesh_vol15_no03_20200201': '20250511111443',
    'in_bjp_site_nadda_letter_to_karyakartas_20230117': '20230117215424',
    'in_kamal_sandesh_vol18_no03_20230201': '20240807131117',
    'in_ks_nadda_address_national_convention_20240217': '20260506075729',
    'in_kamal_sandesh_vol20_no24_20251216': '20260130210424',
    'in_bjp_site_returning_officer_notice_20260116': '20260116185004',
    'in_bjp_site_returning_officer_statement_20260119': '20260119133645',
    'in_bjp_site_nadda_felicitation_speech_20260120': '20260120172716',
    'in_kamal_sandesh_vol21_no02_20260116': '20260203120007',
    'in_ks_national_president_appoints_office_bearers_20260817': '20260904100632',
}
# Raw Arquivo.pt captures (id_ form), with their capture timestamps; all before the cutoff.
ARQUIVO = {'in_bjp_site_bjp_presidents_list_20251120': '20251120213124'}
# Static bitstreams on the party's own e-Library (library.bjp.org).
LIBRARY = ('in_bjp_elib_party_document_vol5_political_resolutions',
           'in_bjp_elib_party_document_vol6_economic_resolutions',
           'in_bjp_elib_party_document_vol3_presidential_speeches_part2',
           'in_bjp_elib_party_document_vol10_evolution_of_bjp',
           'in_bjp_elib_party_document_vol2_presidential_speeches_part1',
           'in_bjp_elib_party_document_vol4_foreign_policy_resolutions')
# Files on the Rajya Sabha Secretariat's debates store.
STORE = ('in_rs_written_answers_19910108_usq1586_rath_yatra',
         'in_rs_debate_19911210_ekta_yatra_bjp_president',
         'in_rs_written_answers_19920304_usq1060_rocket_fire',
         'in_rs_written_answers_19920304_usq1067_flag_hoisting')
# The one undated page, dated by the facing column of the same printed sheet in another store file.
DATE_ANCHORS = {
    'in_rs_written_answers_19910108_usq1586_rath_yatra':
        ('IQ_156_08011991_U1587_p324-325.pdf', 180594,
         '5a201ebcca9a545a5ecc20d805ad7ccb08d1fbb714f40e39197384aab6794baa'),
}
# PDF pages rendered or read (one-based), per source.
PDF_PAGES = {
    'in_bjp_elib_party_document_vol5_political_resolutions': [136, 137, 202, 203, 218, 220, 242, 246, 247, 268, 292],
    'in_bjp_elib_party_document_vol6_economic_resolutions': [163, 189, 196, 205, 209],
    'in_rs_written_answers_19910108_usq1586_rath_yatra': [1],
    'in_bjp_elib_party_document_vol3_presidential_speeches_part2': [1, 3, 28, 31, 32, 73, 74, 85, 203, 204],
    'in_bjp_elib_party_document_vol10_evolution_of_bjp': [20, 21, 31, 40],
    'in_rs_debate_19911210_ekta_yatra_bjp_president': [1, 2, 3],
    'in_rs_written_answers_19920304_usq1060_rocket_fire': [1],
    'in_rs_written_answers_19920304_usq1067_flag_hoisting': [1, 2],
    'in_bjp_elib_party_document_vol2_presidential_speeches_part1': [296, 298, 310, 311, 316, 372, 438],
    'in_bjp_elib_party_document_vol4_foreign_policy_resolutions': [81, 82, 223, 225],
    'in_bjp_site_national_election_officer_declaration_rajnath_singh_20130123': [1],
    'in_kamal_sandesh_vol15_no03_20200201': [1, 3, 6, 7, 8, 9, 12],
    'in_kamal_sandesh_vol18_no03_20230201': [1, 8],
    'in_kamal_sandesh_vol20_no24_20251216': [1, 2, 6, 36],
    'in_bjp_site_returning_officer_notice_20260116': [1],
    'in_bjp_site_returning_officer_statement_20260119': [1],
    'in_kamal_sandesh_vol21_no02_20260116': [1, 6, 7, 8, 9, 36],
}
# Every new claim's (attested_on, event_kind, review observation, holder_name), exactly: distinct dated events are never
# re-dated, relabelled, moved to another observation or given another holder.
EVENTS = {
    'in_bjp_ne_resolution_narrates_president_advani_199004':
        (None, 'narrated_styling', 'BJP-PRES-01', A),
    'in_bjp_ne_resolution_bjp_president_rath_yatra_199011':
        (None, 'meeting_span_attestation', 'BJP-PRES-01', A),
    'in_bjp_ne_resolution_our_president_joshi_ekta_yatra_199203':
        (None, 'meeting_span_attestation', 'BJP-PRES-02', MMJ),
    'in_bjp_ne_resolution_release_joshi_president_bjp_199212':
        (None, 'meeting_span_attestation', 'BJP-PRES-02', MMJ),
    'in_bjp_ne_resolution_assault_bjp_president_joshi_19930227':
        ('1993-02-27', 'in_office_continuation_attestation', 'BJP-PRES-02', MMJ),
    'in_bjp_ne_resolution_our_president_advani_199312':
        (None, 'meeting_span_attestation', 'BJP-PRES-03', A),
    'in_bjp_ne_resolution_ex_president_joshi_199312':
        (None, 'predecessor_reference', 'BJP-PRES-02', MMJ),
    'in_bjp_ne_resolution_party_president_advani_hawala_199602':
        (None, 'meeting_span_attestation', 'BJP-PRES-03', A),
    'in_bjp_ne_resolution_authorises_party_president_advani_199007':
        (None, 'meeting_span_attestation', 'BJP-PRES-01', A),
    'in_rs_sahay_advani_president_bjp_rath_yatra_19910108':
        ('1991-01-08', 'in_office_attestation', 'BJP-PRES-01', A),
    'in_bjp_valedictory_joshi_taking_over_tomorrow_19910131':
        ('1991-01-31', 'assumption_announced_prospective', 'BJP-PRES-01', MMJ),
    'in_bjp_valedictory_joshi_to_assume_presidency_tomorrow_1991':
        (None, 'assumption_announced_prospective', 'BJP-PRES-01', MMJ),
    'in_bjp_outgoing_president_farewell_five_year_tenure_199101':
        (None, 'farewell_address', 'BJP-PRES-01', None),
    'in_bjp_nc_jaipur_new_president_accepts_presidentship_19910201':
        ('1991-02-01', 'election_acceptance_statement', 'BJP-PRES-01', None),
    'in_bjp_nc_jaipur_advani_declined_third_term_19910201':
        ('1991-02-01', 'declined_further_term', 'BJP-PRES-01', A),
    'in_bjp_profile_advani_president_until_199101':
        (None, 'retrospective_term_span', 'BJP-PRES-01', A),
    'in_bjp_biodata_advani_president_19860509_to_1991':
        (None, 'retrospective_term_span', 'BJP-PRES-01', A),
    'in_bjp_biodata_advani_president_199307_to_19980502':
        (None, 'retrospective_term_span', 'BJP-PRES-03', A),
    'in_bjp_evolution_advani_national_president_except_1991_93':
        (None, 'retrospective_term_span', 'BJP-PRES-01', A),
    'in_bjp_evolution_joshi_president_1991_1993':
        (None, 'retrospective_term_span', 'BJP-PRES-02', MMJ),
    'in_bjp_evolution_advani_president_swarna_jayanti_yatra_19970518':
        ('1997-05-18', 'retrospective_biography_statement', 'BJP-PRES-03', A),
    'in_rs_narayanasamy_ekta_yatra_bjp_president_joshi_19911210':
        ('1991-12-10', 'in_office_attestation', 'BJP-PRES-02', MMJ),
    'in_rs_question_joshi_president_bjp_rocket_fire_19920304':
        ('1992-03-04', 'in_office_continuation_attestation', 'BJP-PRES-02', MMJ),
    'in_rs_jacob_answer_bjp_president_joshi_flag_hoisting_19920304':
        ('1992-03-04', 'in_office_continuation_attestation', 'BJP-PRES-02', MMJ),
    'in_bjp_nc_bangalore_elected_once_again_president_19930618':
        ('1993-06-18', 'national_council_election_statement', 'BJP-PRES-03', None),
    'in_bjp_nc_bangalore_joshi_proposed_name_19930618':
        ('1993-06-18', 'nomination_proposal_stated', 'BJP-PRES-03', None),
    'in_bjp_nc_bangalore_joshi_stewardship_predecessor_19930618':
        ('1993-06-18', 'predecessor_reference', 'BJP-PRES-02', MMJ),
    'in_bjp_nc_mumbai_elected_president_yet_another_term_19951110':
        ('1995-11-10', 'reelection_statement', 'BJP-PRES-03', None),
    'in_bjp_ne_new_delhi_last_meeting_during_tenure_19980411':
        ('1998-04-11', 'unnamed_holder_attestation', 'BJP-PRES-03', None),
    'in_bjp_ne_gandhinagar_i_as_president_19980502':
        ('1998-05-02', 'unnamed_holder_attestation', 'BJP-PRES-03', None),
    'in_bjp_ne_gandhinagar_handover_announced_for_tomorrow_19980502':
        ('1998-05-02', 'handover_statement', 'BJP-PRES-03', None),
    'in_bjp_ne_gandhinagar_thakre_elected_unanimously_19980502':
        ('1998-05-02', 'election_result_stated', 'BJP-PRES-04', KT),
    'in_bjp_ne_gandhinagar_thakre_president_elect_19980502':
        ('1998-05-02', 'president_elect_styling', 'BJP-PRES-04', KT),
    'in_bjp_ne_gandhinagar_farewell_as_party_president_19980502':
        ('1998-05-02', 'farewell_address', 'BJP-PRES-03', None),
    'in_bjp_press_release_president_advani_19970716':
        ('1997-07-16', 'in_office_attestation', 'BJP-PRES-03', A),
    'in_bjp_ne_resolution_gratitude_party_president_advani_199804':
        (None, 'meeting_span_attestation', 'BJP-PRES-03', A),
    'in_bjp_advani_new_president_to_be_declared_next_week_19980411':
        ('1998-04-11', 'election_stated_prospective', 'BJP-PRES-04', None),
    'in_bjp_advani_outgoing_president_last_ne_19980411':
        ('1998-04-11', 'outgoing_president_statement', 'BJP-PRES-04', A),
    'in_bjp_thakre_thanks_party_for_unanimous_election_199805':
        (None, 'election_acceptance_statement', 'BJP-PRES-04', KT),
    'in_bjp_thakre_states_he_assumes_office_199805':
        (None, 'assumption_stated_undated', 'BJP-PRES-04', KT),
    'in_bjp_thakre_speaks_as_president_national_council_199805':
        (None, 'meeting_span_attestation', 'BJP-PRES-04', KT),
    'in_bjp_thakre_styled_president_budget_statement_19990227':
        ('1999-02-27', 'in_office_attestation', 'BJP-PRES-04', KT),
    'in_bjp_thakre_press_statement_as_president_20000704':
        ('2000-07-04', 'in_office_continuation_attestation', 'BJP-PRES-04', KT),
    'in_bjp_laxman_nomination_papers_filed_20000802':
        ('2000-08-02', 'nomination_filed', 'BJP-PRES-04', BL),
    'in_bjp_thakre_outgoing_president_present_20000802':
        ('2000-08-02', 'predecessor_reference', 'BJP-PRES-04', KT),
    'in_bjp_laxman_thanks_council_for_electing_him_fifth_president_200008':
        (None, 'national_council_election_statement', 'BJP-PRES-04', BL),
    'in_bjp_laxman_names_thakre_immediate_predecessor_200008':
        (None, 'predecessor_reference', 'BJP-PRES-04', KT),
    'in_bjp_gs_report_thakre_took_over_at_gandhinagar_1998':
        (None, 'assumption_recalled_retrospective', 'BJP-PRES-04', KT),
    'in_bjp_laxman_styled_president_maiden_press_conference_20000831':
        ('2000-08-31', 'in_office_attestation', 'BJP-PRES-04', BL),
    'in_bjp_laxman_took_over_reins_recalled_maiden_press_conference_200008':
        (None, 'assumption_recalled_retrospective', 'BJP-PRES-04', BL),
    'in_bjp_laxman_statement_as_president_20000901':
        ('2000-09-01', 'in_office_continuation_attestation', 'BJP-PRES-04', BL),
    'in_bjp_laxman_took_over_presidency_at_nagpur_session_200008':
        (None, 'assumption_recalled_retrospective', 'BJP-PRES-04', BL),
    'in_bjp_laxman_press_statement_as_president_20010309':
        ('2001-03-09', 'in_office_continuation_attestation', 'BJP-PRES-04', BL),
    'in_bjp_office_bearers_consider_laxman_letter_20010314':
        ('2001-03-14', 'resignation_considered', 'BJP-PRES-04', BL),
    'in_bjp_office_bearers_accept_laxman_resignation_immediate_effect_20010314':
        ('2001-03-14', 'end_of_office_stated', 'BJP-PRES-04', BL),
    'in_bjp_jana_krishnamurthi_designated_acting_president_20010314':
        ('2001-03-14', 'acting_president_designated', 'BJP-PRES-05', JK),
    'in_bjp_laxman_has_stepped_down_pending_enquiry_20010315':
        ('2001-03-15', 'resignation_stated', 'BJP-PRES-04', BL),
    'in_bjp_jana_krishnamurthy_styled_president_20010318':
        ('2001-03-18', 'acting_service_attestation', 'BJP-PRES-05', JK),
    'in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324':
        ('2001-03-24', 'acting_service_attestation', 'BJP-PRES-05', JK),
    'in_bjp_ne_entrusted_presidentship_to_jana_krishnamurthy_20010324':
        ('2001-03-24', 'national_executive_entrustment_statement', 'BJP-PRES-05', JK),
    'in_bjp_laxman_resignation_letter_dated_20010313_recalled':
        ('2001-03-13', 'resignation_tendered', 'BJP-PRES-04', BL),
    'in_bjp_office_bearers_acceptance_recalled_20010314':
        ('2001-03-14', 'resignation_acceptance_recalled', 'BJP-PRES-04', BL),
    'in_bjp_jana_krishnamurthy_accepted_acting_responsibility_recalled_2001':
        (None, 'acting_responsibility_accepted', 'BJP-PRES-05', JK),
    'in_bjp_ne_resolution_laxman_resigned_as_president_20010324':
        ('2001-03-24', 'resignation_recorded', 'BJP-PRES-04', BL),
    'in_bjp_jana_krishnamurthi_styled_president_20010329':
        ('2001-03-29', 'acting_service_attestation', 'BJP-PRES-05', JK),
    'in_bjp_jana_krishnamurthi_taken_charge_recalled_2001':
        (None, 'assumption_recalled_retrospective', 'BJP-PRES-05', JK),
    'in_bjp_national_president_jana_krishnamurthi_constitutes_committee_20020624':
        ('2002-06-24', 'acting_service_attestation', 'BJP-PRES-05', JK),
    'in_bjp_naidu_profile_span_ist_july_2002_onwards':
        (None, 'retrospective_term_span', 'BJP-PRES-05', VN),
    'in_bjp_naidu_president_first_press_conference_20020711':
        ('2002-07-11', 'in_office_continuation_attestation', 'BJP-PRES-05', VN),
    'in_bjp_naidu_entrusted_and_accepted_recalled_2002':
        (None, 'selection_recalled_retrospective', 'BJP-PRES-05', VN),
    'in_bjp_today_naidu_took_over_on_july_1_20020701':
        ('2002-07-01', 'assumption_statement', 'BJP-PRES-05', VN),
    'in_bjp_today_jana_krishnamurthi_predecessor_20020701':
        ('2002-07-01', 'predecessor_reference', 'BJP-PRES-05', JK),
    'in_bjp_national_council_address_endorsement_stated_20020803':
        ('2002-08-03', 'national_council_endorsement_statement', 'BJP-PRES-05', None),
    'in_bjp_naidu_chairs_meeting_as_party_president_20041018':
        ('2004-10-18', 'in_office_continuation_attestation', 'BJP-PRES-05', VN),
    'in_bjp_naidu_offers_resignation_20041018':
        ('2004-10-18', 'resignation_offered', 'BJP-PRES-05', VN),
    'in_bjp_naidu_resignation_accepted_20041018':
        ('2004-10-18', 'resignation_accepted', 'BJP-PRES-05', VN),
    'in_bjp_advani_appointed_president_ratification_pending_20041018':
        ('2004-10-18', 'appointment_decision', 'BJP-PRES-06', A),
    'in_bjp_advani_statement_as_president_20041020':
        ('2004-10-20', 'in_office_attestation', 'BJP-PRES-06', A),
    'in_bjp_advani_assumed_office_two_days_ago_recalled_20041018':
        ('2004-10-18', 'assumption_recalled_retrospective', 'BJP-PRES-06', A),
    'in_bjp_today_national_council_endorses_advani_election_20041027':
        ('2004-10-27', 'national_council_endorsement', 'BJP-PRES-06', A),
    'in_bjp_today_outgoing_president_naidu_farewell_20041027':
        ('2004-10-27', 'farewell_address', 'BJP-PRES-05', VN),
    'in_bjp_today_vajpayee_fifth_time_advani_appointed_2004':
        (None, 'retrospective_biography_statement', 'BJP-PRES-06', A),
    'in_bjp_advani_elected_by_national_executive_20041027':
        ('2004-10-27', 'election_by_national_executive_reported', 'BJP-PRES-06', A),
    'in_bjp_advani_constitutes_national_executive_20041030':
        ('2004-10-30', 'in_office_continuation_attestation', 'BJP-PRES-06', A),
    'in_bjp_parliamentary_board_rejects_advani_resignation_20050608':
        ('2005-06-08', 'resignation_rejected', 'BJP-PRES-06', A),
    'in_bjp_advani_styled_president_after_resignation_episode_20050615':
        ('2005-06-15', 'in_office_continuation_attestation', 'BJP-PRES-06', A),
    'in_bjp_advani_styled_president_opening_remarks_20051226':
        ('2005-12-26', 'in_office_continuation_attestation', 'BJP-PRES-06', A),
    'in_bjp_advani_last_meeting_he_will_preside_20051226':
        ('2005-12-26', 'outgoing_president_statement', 'BJP-PRES-06', A),
    'in_bjp_rajat_jayanti_national_convention_dated_2005':
        (None, 'convention_dates_context', 'BJP-PRES-06', None),
    'in_bjp_rajnath_singh_styled_national_president_20060102':
        ('2006-01-02', 'in_office_attestation', 'BJP-PRES-06', RS),
    'in_bjp_rajnath_singh_accepts_office_20060102':
        ('2006-01-02', 'election_acceptance_statement', 'BJP-PRES-06', RS),
    'in_bjp_rajnath_singh_taken_over_recalled_2006':
        (None, 'assumption_recalled_retrospective', 'BJP-PRES-06', RS),
    'in_bjp_rajnath_singh_styled_president_national_council_20060120':
        ('2006-01-20', 'in_office_continuation_attestation', 'BJP-PRES-06', RS),
    'in_bjp_rajnath_singh_responsibility_placed_after_convention_recalled_2005':
        (None, 'selection_recalled_retrospective', 'BJP-PRES-06', RS),
    'in_bjp_gadkari_profile_heading_national_president_200912':
        (None, 'profile_heading_styling', 'BJP-PRES-07', NG),
    'in_bjp_parliamentary_board_thanks_rajnath_singh_for_tenure_20091219':
        ('2009-12-19', 'predecessor_reference', 'BJP-PRES-07', RS),
    'in_bjp_gadkari_first_press_conference_as_president_20091224':
        ('2009-12-24', 'in_office_attestation', 'BJP-PRES-07', NG),
    'in_bjp_gadkari_entrusted_new_responsibility_recalled_2009':
        (None, 'selection_recalled_retrospective', 'BJP-PRES-07', NG),
    'in_bjp_listing_dates_gadkari_profile_20091219':
        ('2009-12-19', 'press_release_index_entry', 'BJP-PRES-07', NG),
    'in_bjp_gadkari_styled_national_president_indore_address_20100218':
        ('2010-02-18', 'in_office_continuation_attestation', 'BJP-PRES-07', NG),
    'in_bjp_gadkari_presidential_address_council_elected_him_20100218':
        ('2010-02-18', 'national_council_election_statement', 'BJP-PRES-07', NG),
    'in_bjp_gadkari_announces_national_executive_as_president_20100316':
        ('2010-03-16', 'in_office_continuation_attestation', 'BJP-PRES-07', NG),
    'in_bjp_gadkari_styled_national_president_statement_20130122':
        ('2013-01-22', 'in_office_attestation', 'BJP-PRES-07', NG),
    'in_bjp_gadkari_decides_not_to_seek_second_term_20130122':
        ('2013-01-22', 'decision_not_to_seek_second_term', 'BJP-PRES-07', NG),
    'in_bjp_national_president_election_process_completed_20130123':
        ('2013-01-23', 'election_result', 'BJP-PRES-07', RS),
    'in_bjp_gehlot_declares_rajnath_singh_elected_unopposed_20130123':
        ('2013-01-23', 'result_declaration', 'BJP-PRES-07', RS),
    'in_bjp_rajnath_singh_styled_newly_elected_president_20130123':
        ('2013-01-23', 'president_elect_styling', 'BJP-PRES-07', RS),
    'in_bjp_gadkari_styled_former_national_president_20130127':
        ('2013-01-27', 'predecessor_reference', 'BJP-PRES-07', NG),
    'in_bjp_rajnath_singh_presidential_address_national_council_20130302':
        ('2013-03-02', 'in_office_attestation', 'BJP-PRES-07', RS),
    'in_bjp_rajnath_singh_took_over_again_recalled_2013':
        (None, 'assumption_recalled_retrospective', 'BJP-PRES-07', RS),
    'in_bjp_presidents_list_jana_krishnamurthy_2001_to_2002':
        (None, 'retrospective_term_span', 'BJP-PRES-05', JK),
    'in_bjp_presidents_list_gadkari_2010_to_2013':
        (None, 'retrospective_term_span', 'BJP-PRES-07', NG),
    'in_bjp_presidents_list_rajnath_singh_2013_to_present':
        (None, 'retrospective_term_span', 'BJP-PRES-07', RS),
    'in_bjp_parliamentary_board_thanks_outgoing_president_rajnath_singh_20140709':
        ('2014-07-09', 'predecessor_reference', 'BJP-PRES-08', RS),
    'in_bjp_amit_shah_styled_national_president_profile_20140709':
        ('2014-07-09', 'in_office_attestation', 'BJP-PRES-08', AS),
    'in_bjp_amit_shah_styled_president_national_council_address_20140809':
        ('2014-08-09', 'in_office_continuation_attestation', 'BJP-PRES-08', AS),
    'in_bjp_amit_shah_presidential_address_thanks_council_for_election_20140809':
        ('2014-08-09', 'national_council_election_statement', 'BJP-PRES-08', AS),
    'in_bjp_amit_shah_recalls_rajnath_singh_tenure_20140809':
        ('2014-08-09', 'predecessor_reference', 'BJP-PRES-07', RS),
    'in_bjp_amit_shah_nomination_filed_national_president_20160124':
        ('2016-01-24', 'nomination_filed', 'BJP-PRES-08', AS),
    'in_bjp_amit_shah_re_elected_heading_20160124':
        ('2016-01-24', 'election_result', 'BJP-PRES-08', AS),
    'in_bjp_amit_shah_styled_national_president_felicitation_20160202':
        ('2016-02-02', 'in_office_attestation', 'BJP-PRES-08', AS),
    'in_ks_amit_shah_styled_bjp_national_president_20161015':
        ('2016-10-15', 'in_office_continuation_attestation', 'BJP-PRES-08', AS),
    'in_ks_amit_shah_styled_national_president_at_board_meeting_20190617':
        ('2019-06-17', 'in_office_attestation', 'BJP-PRES-08', AS),
    'in_ks_parliamentary_board_appoints_nadda_working_president_20190617':
        ('2019-06-17', 'working_president_appointment', 'BJP-PRES-09', JPN),
    'in_ks_amit_shah_request_to_be_relieved_reported_20190617':
        ('2019-06-17', 'request_to_relinquish_reported', 'BJP-PRES-08', AS),
    'in_ks_amit_shah_continues_national_president_20190617':
        ('2019-06-17', 'continuation_statement', 'BJP-PRES-08', AS),
    'in_ks_amit_shah_became_president_july_2014_recalled_2019':
        (None, 'selection_recalled_retrospective', 'BJP-PRES-08', AS),
    'in_ks_nadda_takes_charge_working_president_2019':
        (None, 'working_president_assumption_of_charge', 'BJP-PRES-09', JPN),
    'in_ks_life_sketch_nadda_working_president_onwards_2019':
        (None, 'working_president_period_retrospective', 'BJP-PRES-09', JPN),
    'in_ks_nadda_elected_unopposed_11th_president_20200120':
        ('2020-01-20', 'election_result', 'BJP-PRES-09', JPN),
    'in_ks_nadda_took_charge_20200120':
        ('2020-01-20', 'assumption_statement', 'BJP-PRES-09', JPN),
    'in_ks_nadda_nomination_sole_candidate_20200120':
        ('2020-01-20', 'nomination_filed', 'BJP-PRES-09', JPN),
    'in_ks_radha_mohan_singh_declares_nadda_elected_20200120':
        ('2020-01-20', 'result_declaration', 'BJP-PRES-09', JPN),
    'in_ks_amit_shah_hands_over_charge_20200120':
        ('2020-01-20', 'handover_statement', 'BJP-PRES-08', AS),
    'in_ks_life_sketch_nadda_working_president_period_2019_2020':
        (None, 'working_president_period_retrospective', 'BJP-PRES-09', JPN),
    'in_ks_life_sketch_nadda_elected_unopposed_20200120':
        ('2020-01-20', 'retrospective_biography_statement', 'BJP-PRES-09', JPN),
    'in_ks_gadkari_styled_former_party_president_20200120':
        ('2020-01-20', 'predecessor_reference', 'BJP-PRES-07', NG),
    'in_ks_rajnath_singh_styled_former_party_president_20200120':
        ('2020-01-20', 'predecessor_reference', 'BJP-PRES-07', RS),
    'in_ks_radha_mohan_singh_hands_certificate_to_nadda_2020':
        (None, 'certificate_presentation', 'BJP-PRES-09', JPN),
    'in_ks_amit_shah_today_nadda_takes_over_2020':
        (None, 'handover_statement', 'BJP-PRES-08', AS),
    'in_bjp_nadda_styled_national_president_letter_20230117':
        ('2023-01-17', 'in_office_attestation', 'BJP-PRES-09', JPN),
    'in_ks_national_executive_extends_nadda_tenure_to_june_2024_20230117':
        ('2023-01-17', 'term_extension', 'BJP-PRES-09', JPN),
    'in_ks_amit_shah_announces_extension_20230117':
        ('2023-01-17', 'extension_announcement', 'BJP-PRES-09', JPN),
    'in_ks_nadda_took_charge_20200120_recalled':
        ('2020-01-20', 'assumption_recalled_retrospective', 'BJP-PRES-09', JPN),
    'in_ks_nadda_styled_national_president_convention_20240217':
        ('2024-02-17', 'in_office_continuation_attestation', 'BJP-PRES-09', JPN),
    'in_bjp_presidents_list_gadkari_2010_2013':
        (None, 'retrospective_term_span', 'BJP-PRES-07', NG),
    'in_bjp_presidents_list_rajnath_singh_2005_2009_2013_2014':
        (None, 'retrospective_term_span', 'BJP-PRES-07', RS),
    'in_bjp_presidents_list_amit_shah_2014_2017_2017_2020':
        (None, 'retrospective_term_span', 'BJP-PRES-08', AS),
    'in_bjp_presidents_list_nadda_2020_present':
        (None, 'retrospective_term_span', 'BJP-PRES-09', JPN),
    'in_ks_parliamentary_board_appoints_nabin_working_president_20251214':
        ('2025-12-14', 'working_president_appointment', 'BJP-PRES-10', NN),
    'in_ks_nabin_assumes_working_presidency_20251215':
        ('2025-12-15', 'working_president_assumption_of_charge', 'BJP-PRES-10', NN),
    'in_ks_nadda_styled_national_president_20251215':
        ('2025-12-15', 'in_office_attestation', 'BJP-PRES-09', JPN),
    'in_bjp_returning_officer_announces_president_election_schedule_20260116':
        ('2026-01-16', 'election_schedule', 'BJP-PRES-10', None),
    'in_bjp_nabin_nominations_filed_20260119':
        ('2026-01-19', 'nomination_filed', 'BJP-PRES-10', NN),
    'in_bjp_nabin_nominations_valid_on_scrutiny_20260119':
        ('2026-01-19', 'nomination_scrutiny', 'BJP-PRES-10', NN),
    'in_bjp_returning_officer_announces_only_nabin_proposed_20260119':
        ('2026-01-19', 'sole_candidate_announced', 'BJP-PRES-10', NN),
    'in_bjp_nadda_says_nabin_assumed_charge_today_20260120':
        ('2026-01-20', 'assumption_statement', 'BJP-PRES-10', NN),
    'in_bjp_nadda_styled_ex_national_president_20260120':
        ('2026-01-20', 'predecessor_reference', 'BJP-PRES-10', JPN),
    'in_ks_nabin_elected_unopposed_stated_20260119':
        ('2026-01-19', 'election_result', 'BJP-PRES-10', NN),
    'in_ks_laxman_declares_nabin_elected_20260120':
        ('2026-01-20', 'result_declaration', 'BJP-PRES-10', NN),
    'in_ks_nabin_assumes_charge_national_president_20260120':
        ('2026-01-20', 'assumption_statement', 'BJP-PRES-10', NN),
    'in_ks_nadda_hands_over_charge_20260120':
        ('2026-01-20', 'handover_statement', 'BJP-PRES-10', JPN),
    'in_ks_nabin_working_president_appointed_20251214_recalled':
        ('2025-12-14', 'working_president_appointment_recalled', 'BJP-PRES-10', NN),
    'in_ks_former_national_president_rajnath_singh_20260120':
        ('2026-01-20', 'predecessor_reference', 'BJP-PRES-10', RS),
    'in_ks_former_national_president_amit_shah_20260120':
        ('2026-01-20', 'predecessor_reference', 'BJP-PRES-10', AS),
    'in_ks_former_national_president_nitin_gadkari_20260120':
        ('2026-01-20', 'predecessor_reference', 'BJP-PRES-10', NG),
    'in_ks_nabin_appoints_national_office_bearers_20260817':
        ('2026-08-17', 'in_office_attestation', 'BJP-PRES-10', NN),
}
ELECTIONS = ('in_bjp_valedictory_joshi_taking_over_tomorrow_19910131',
             'in_bjp_valedictory_joshi_to_assume_presidency_tomorrow_1991',
             'in_bjp_nc_jaipur_new_president_accepts_presidentship_19910201',
             'in_bjp_nc_bangalore_elected_once_again_president_19930618',
             'in_bjp_nc_bangalore_joshi_proposed_name_19930618',
             'in_bjp_nc_mumbai_elected_president_yet_another_term_19951110',
             'in_bjp_ne_gandhinagar_thakre_elected_unanimously_19980502',
             'in_bjp_ne_gandhinagar_thakre_president_elect_19980502',
             'in_bjp_advani_new_president_to_be_declared_next_week_19980411',
             'in_bjp_thakre_thanks_party_for_unanimous_election_199805',
             'in_bjp_thakre_states_he_assumes_office_199805',
             'in_bjp_laxman_nomination_papers_filed_20000802',
             'in_bjp_laxman_thanks_council_for_electing_him_fifth_president_200008',
             'in_bjp_ne_entrusted_presidentship_to_jana_krishnamurthy_20010324',
             'in_bjp_national_council_address_endorsement_stated_20020803',
             'in_bjp_advani_appointed_president_ratification_pending_20041018',
             'in_bjp_today_national_council_endorses_advani_election_20041027',
             'in_bjp_advani_elected_by_national_executive_20041027',
             'in_bjp_rajnath_singh_accepts_office_20060102',
             'in_bjp_gadkari_presidential_address_council_elected_him_20100218',
             'in_bjp_national_president_election_process_completed_20130123',
             'in_bjp_gehlot_declares_rajnath_singh_elected_unopposed_20130123',
             'in_bjp_rajnath_singh_styled_newly_elected_president_20130123',
             'in_bjp_amit_shah_presidential_address_thanks_council_for_election_20140809',
             'in_bjp_amit_shah_nomination_filed_national_president_20160124',
             'in_bjp_amit_shah_re_elected_heading_20160124',
             'in_ks_nadda_elected_unopposed_11th_president_20200120',
             'in_ks_nadda_nomination_sole_candidate_20200120',
             'in_ks_radha_mohan_singh_declares_nadda_elected_20200120',
             'in_ks_radha_mohan_singh_hands_certificate_to_nadda_2020',
             'in_bjp_returning_officer_announces_president_election_schedule_20260116',
             'in_bjp_nabin_nominations_filed_20260119',
             'in_bjp_nabin_nominations_valid_on_scrutiny_20260119',
             'in_bjp_returning_officer_announces_only_nabin_proposed_20260119',
             'in_ks_nabin_elected_unopposed_stated_20260119',
             'in_ks_laxman_declares_nabin_elected_20260120')
ENDINGS = ('in_bjp_ne_resolution_ex_president_joshi_199312',
           'in_bjp_outgoing_president_farewell_five_year_tenure_199101',
           'in_bjp_nc_jaipur_advani_declined_third_term_19910201',
           'in_bjp_nc_bangalore_joshi_stewardship_predecessor_19930618',
           'in_bjp_ne_gandhinagar_handover_announced_for_tomorrow_19980502',
           'in_bjp_ne_gandhinagar_farewell_as_party_president_19980502',
           'in_bjp_advani_outgoing_president_last_ne_19980411',
           'in_bjp_thakre_outgoing_president_present_20000802',
           'in_bjp_laxman_names_thakre_immediate_predecessor_200008',
           'in_bjp_office_bearers_consider_laxman_letter_20010314',
           'in_bjp_laxman_has_stepped_down_pending_enquiry_20010315',
           'in_bjp_laxman_resignation_letter_dated_20010313_recalled',
           'in_bjp_office_bearers_acceptance_recalled_20010314',
           'in_bjp_ne_resolution_laxman_resigned_as_president_20010324',
           'in_bjp_today_jana_krishnamurthi_predecessor_20020701',
           'in_bjp_naidu_offers_resignation_20041018',
           'in_bjp_naidu_resignation_accepted_20041018',
           'in_bjp_today_outgoing_president_naidu_farewell_20041027',
           'in_bjp_parliamentary_board_rejects_advani_resignation_20050608',
           'in_bjp_advani_last_meeting_he_will_preside_20051226',
           'in_bjp_parliamentary_board_thanks_rajnath_singh_for_tenure_20091219',
           'in_bjp_gadkari_decides_not_to_seek_second_term_20130122',
           'in_bjp_gadkari_styled_former_national_president_20130127',
           'in_bjp_parliamentary_board_thanks_outgoing_president_rajnath_singh_20140709',
           'in_bjp_amit_shah_recalls_rajnath_singh_tenure_20140809',
           'in_ks_amit_shah_request_to_be_relieved_reported_20190617',
           'in_ks_amit_shah_hands_over_charge_20200120',
           'in_ks_gadkari_styled_former_party_president_20200120',
           'in_ks_rajnath_singh_styled_former_party_president_20200120',
           'in_ks_amit_shah_today_nadda_takes_over_2020',
           'in_bjp_nadda_styled_ex_national_president_20260120',
           'in_ks_nadda_hands_over_charge_20260120',
           'in_ks_former_national_president_rajnath_singh_20260120',
           'in_ks_former_national_president_amit_shah_20260120',
           'in_ks_former_national_president_nitin_gadkari_20260120')
INTERIM = ('in_bjp_jana_krishnamurthi_designated_acting_president_20010314',
           'in_bjp_jana_krishnamurthy_styled_president_20010318',
           'in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324',
           'in_bjp_jana_krishnamurthy_accepted_acting_responsibility_recalled_2001',
           'in_bjp_jana_krishnamurthi_styled_president_20010329',
           'in_bjp_national_president_jana_krishnamurthi_constitutes_committee_20020624',
           'in_ks_parliamentary_board_appoints_nadda_working_president_20190617',
           'in_ks_nadda_takes_charge_working_president_2019',
           'in_ks_life_sketch_nadda_working_president_onwards_2019',
           'in_ks_life_sketch_nadda_working_president_period_2019_2020',
           'in_ks_parliamentary_board_appoints_nabin_working_president_20251214',
           'in_ks_nabin_assumes_working_presidency_20251215',
           'in_ks_nabin_working_president_appointed_20251214_recalled')
CONTINUATION = ('in_bjp_ne_resolution_narrates_president_advani_199004',
                'in_bjp_ne_resolution_bjp_president_rath_yatra_199011',
                'in_bjp_ne_resolution_our_president_joshi_ekta_yatra_199203',
                'in_bjp_ne_resolution_release_joshi_president_bjp_199212',
                'in_bjp_ne_resolution_assault_bjp_president_joshi_19930227',
                'in_bjp_ne_resolution_our_president_advani_199312',
                'in_bjp_ne_resolution_party_president_advani_hawala_199602',
                'in_bjp_ne_resolution_authorises_party_president_advani_199007',
                'in_rs_question_joshi_president_bjp_rocket_fire_19920304',
                'in_rs_jacob_answer_bjp_president_joshi_flag_hoisting_19920304',
                'in_bjp_ne_resolution_gratitude_party_president_advani_199804',
                'in_bjp_thakre_speaks_as_president_national_council_199805',
                'in_bjp_thakre_press_statement_as_president_20000704',
                'in_bjp_laxman_statement_as_president_20000901',
                'in_bjp_laxman_press_statement_as_president_20010309',
                'in_bjp_naidu_president_first_press_conference_20020711',
                'in_bjp_naidu_chairs_meeting_as_party_president_20041018',
                'in_bjp_advani_constitutes_national_executive_20041030',
                'in_bjp_advani_styled_president_after_resignation_episode_20050615',
                'in_bjp_advani_styled_president_opening_remarks_20051226',
                'in_bjp_rajnath_singh_styled_president_national_council_20060120',
                'in_bjp_gadkari_styled_national_president_indore_address_20100218',
                'in_bjp_gadkari_announces_national_executive_as_president_20100316',
                'in_bjp_amit_shah_styled_president_national_council_address_20140809',
                'in_ks_amit_shah_styled_bjp_national_president_20161015',
                'in_ks_amit_shah_continues_national_president_20190617',
                'in_ks_national_executive_extends_nadda_tenure_to_june_2024_20230117',
                'in_ks_amit_shah_announces_extension_20230117',
                'in_ks_nadda_styled_national_president_convention_20240217')
RETROSPECTIVE = ('in_bjp_profile_advani_president_until_199101',
                 'in_bjp_biodata_advani_president_19860509_to_1991',
                 'in_bjp_biodata_advani_president_199307_to_19980502',
                 'in_bjp_evolution_advani_national_president_except_1991_93',
                 'in_bjp_evolution_joshi_president_1991_1993',
                 'in_bjp_evolution_advani_president_swarna_jayanti_yatra_19970518',
                 'in_bjp_gs_report_thakre_took_over_at_gandhinagar_1998',
                 'in_bjp_laxman_took_over_reins_recalled_maiden_press_conference_200008',
                 'in_bjp_laxman_took_over_presidency_at_nagpur_session_200008',
                 'in_bjp_jana_krishnamurthi_taken_charge_recalled_2001',
                 'in_bjp_naidu_profile_span_ist_july_2002_onwards',
                 'in_bjp_naidu_entrusted_and_accepted_recalled_2002',
                 'in_bjp_advani_assumed_office_two_days_ago_recalled_20041018',
                 'in_bjp_today_vajpayee_fifth_time_advani_appointed_2004',
                 'in_bjp_rajnath_singh_taken_over_recalled_2006',
                 'in_bjp_rajnath_singh_responsibility_placed_after_convention_recalled_2005',
                 'in_bjp_gadkari_profile_heading_national_president_200912',
                 'in_bjp_gadkari_entrusted_new_responsibility_recalled_2009',
                 'in_bjp_listing_dates_gadkari_profile_20091219',
                 'in_bjp_rajnath_singh_took_over_again_recalled_2013',
                 'in_bjp_presidents_list_jana_krishnamurthy_2001_to_2002',
                 'in_bjp_presidents_list_gadkari_2010_to_2013',
                 'in_bjp_presidents_list_rajnath_singh_2013_to_present',
                 'in_ks_amit_shah_became_president_july_2014_recalled_2019',
                 'in_ks_life_sketch_nadda_elected_unopposed_20200120',
                 'in_ks_nadda_took_charge_20200120_recalled',
                 'in_bjp_presidents_list_gadkari_2010_2013',
                 'in_bjp_presidents_list_rajnath_singh_2005_2009_2013_2014',
                 'in_bjp_presidents_list_amit_shah_2014_2017_2017_2020',
                 'in_bjp_presidents_list_nadda_2020_present')
UNNAMED = ('in_bjp_ne_new_delhi_last_meeting_during_tenure_19980411',
           'in_bjp_ne_gandhinagar_i_as_president_19980502')
CONTEXT = ('in_bjp_rajat_jayanti_national_convention_dated_2005',)
ELECTIONS_KINDS = {
    'appointment_decision', 'assumption_announced_prospective', 'assumption_stated_undated',
    'certificate_presentation', 'election_acceptance_statement', 'election_by_national_executive_reported',
    'election_result', 'election_result_stated', 'election_schedule', 'election_stated_prospective',
    'national_council_election_statement', 'national_council_endorsement', 'national_council_endorsement_statement',
    'national_executive_entrustment_statement', 'nomination_filed', 'nomination_proposal_stated',
    'nomination_scrutiny', 'president_elect_styling', 'reelection_statement', 'result_declaration',
    'sole_candidate_announced'
}
ENDINGS_KINDS = {
    'decision_not_to_seek_second_term', 'declined_further_term', 'farewell_address', 'handover_statement',
    'outgoing_president_statement', 'predecessor_reference', 'request_to_relinquish_reported',
    'resignation_acceptance_recalled', 'resignation_accepted', 'resignation_considered', 'resignation_offered',
    'resignation_recorded', 'resignation_rejected', 'resignation_stated', 'resignation_tendered'
}
INTERIM_KINDS = {
    'acting_president_designated', 'acting_responsibility_accepted', 'acting_service_attestation',
    'working_president_appointment', 'working_president_appointment_recalled',
    'working_president_assumption_of_charge', 'working_president_period_retrospective'
}
CONTINUATION_KINDS = {
    'continuation_statement', 'extension_announcement', 'in_office_continuation_attestation',
    'meeting_span_attestation', 'narrated_styling', 'term_extension'
}
RETROSPECTIVE_KINDS = {
    'assumption_recalled_retrospective', 'press_release_index_entry', 'profile_heading_styling',
    'retrospective_biography_statement', 'retrospective_term_span', 'selection_recalled_retrospective'
}
UNNAMED_KINDS = {
    'unnamed_holder_attestation'
}
CONTEXT_KINDS = {
    'convention_dates_context'
}
HOLDERS = [
    (A, '1991-01-08', None, None),
    (MMJ, '1991-12-10', None, None),
    (A, '1997-07-16', None, None),
    (KT, '1999-02-27', None, None),
    (BL, '2000-08-31', None, '2001-03-14'),
    (VN, None, '2002-07-01', None),
    (A, '2004-10-20', None, None),
    (RS, '2006-01-02', None, None),
    (NG, '2009-12-24', None, None),
    (NG, '2013-01-22', None, None),
    (RS, '2013-03-02', None, None),
    (AS, '2014-07-09', None, None),
    (AS, '2016-02-02', None, None),
    (AS, '2019-06-17', None, None),
    (JPN, None, '2020-01-20', None),
    (JPN, '2023-01-17', None, None),
    (JPN, '2025-12-15', None, None),
    (NN, None, '2026-01-20', None),
    (NN, '2026-08-17', None, None),
]
HOLDER_CLAIMS = [
    ['in_rs_sahay_advani_president_bjp_rath_yatra_19910108'],
    ['in_rs_narayanasamy_ekta_yatra_bjp_president_joshi_19911210'],
    ['in_bjp_press_release_president_advani_19970716'],
    ['in_bjp_thakre_styled_president_budget_statement_19990227'],
    ['in_bjp_laxman_styled_president_maiden_press_conference_20000831',
     'in_bjp_office_bearers_accept_laxman_resignation_immediate_effect_20010314'],
    ['in_bjp_today_naidu_took_over_on_july_1_20020701'],
    ['in_bjp_advani_statement_as_president_20041020'],
    ['in_bjp_rajnath_singh_styled_national_president_20060102'],
    ['in_bjp_gadkari_first_press_conference_as_president_20091224'],
    ['in_bjp_gadkari_styled_national_president_statement_20130122'],
    ['in_bjp_rajnath_singh_presidential_address_national_council_20130302'],
    ['in_bjp_amit_shah_styled_national_president_profile_20140709'],
    ['in_bjp_amit_shah_styled_national_president_felicitation_20160202'],
    ['in_ks_amit_shah_styled_national_president_at_board_meeting_20190617'],
    ['in_ks_nadda_took_charge_20200120'],
    ['in_bjp_nadda_styled_national_president_letter_20230117'],
    ['in_ks_nadda_styled_national_president_20251215'],
    ['in_bjp_nadda_says_nabin_assumed_charge_today_20260120', 'in_ks_nabin_assumes_charge_national_president_20260120'],
    ['in_ks_nabin_appoints_national_office_bearers_20260817'],
]
HOLDER_REVIEW = [
    'BJP-PRES-01', 'BJP-PRES-02', 'BJP-PRES-03', 'BJP-PRES-04', 'BJP-PRES-04', 'BJP-PRES-05', 'BJP-PRES-06',
    'BJP-PRES-06', 'BJP-PRES-07', 'BJP-PRES-07', 'BJP-PRES-07', 'BJP-PRES-08', 'BJP-PRES-08', 'BJP-PRES-08',
    'BJP-PRES-09', 'BJP-PRES-09', 'BJP-PRES-09', 'BJP-PRES-10', 'BJP-PRES-10'
]
NEVER_HOLDER = ELECTIONS + ENDINGS + INTERIM + CONTINUATION + RETROSPECTIVE + UNNAMED + CONTEXT
HOLDER_OBSERVATIONS = tuple(cid for cid, e in EVENTS.items()
                            if e[1] in ('in_office_attestation', 'assumption_statement', 'end_of_office_stated'))
# The event kind that may date an observation, and the kinds that state a start or an end; every other kind never feeds
# a holder.
HOLDER_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_statement'}
UNTIL_KINDS = {'end_of_office_stated'}
NEVER_KINDS = (ELECTIONS_KINDS | ENDINGS_KINDS | INTERIM_KINDS | CONTINUATION_KINDS | RETROSPECTIVE_KINDS | UNNAMED_KINDS
               | CONTEXT_KINDS)
GROUPS = ((ELECTIONS, ELECTIONS_KINDS), (ENDINGS, ENDINGS_KINDS), (INTERIM, INTERIM_KINDS),
          (CONTINUATION, CONTINUATION_KINDS), (RETROSPECTIVE, RETROSPECTIVE_KINDS), (UNNAMED, UNNAMED_KINDS),
          (CONTEXT, CONTEXT_KINDS))
# The only stated starts and ends: the party journal and organ for Naidu, Nadda and Nabin; the office bearers' acceptance
# 'with immediate effect' for Laxman.
STARTS = [(VN, '2002-07-01'), (JPN, '2020-01-20'), (NN, '2026-01-20')]
ENDS = [(BL, '2001-03-14')]
# Working presidencies and the acting presidency are claims only: no holder of these names is dated inside them. The
# acting service from 14 March 2001 has no stated end, so its window runs until Naidu's stated start of 1 July 2002.
WORKING = {JPN: ('2019-06-17', '2020-01-19'), NN: ('2025-12-14', '2026-01-19')}
ACTING = (JK, '2001-03-14', '2002-06-30')
HOLDER_DATES = {d for h in HOLDERS for d in h[1:] if d}
# Dates that are never any holder's attested_on, start or end: elections, nominations, declarations, endorsements,
# President-elect stylings, prospective announcements and undated assumptions; farewells, handovers, predecessor
# references, resignations and their consideration, acceptance without effect or rejection; acting and working service,
# extensions and continuations; recollections and retrospective statements; and the rejected candidate days (the
# bio-data's start, the announced handover of 3 May 1998, the Silver Jubilee convention's last day, the Gadkari profile
# page's date and the meeting of 1 September 2026, known only from a rendering modified after the cutoff).
NEVER_HOLDER_DATE = sorted(({e[0] for cid, e in EVENTS.items() if cid in NEVER_HOLDER and e[0]} - HOLDER_DATES)
                           | {'1986-05-09', '1998-05-03', '2005-12-30', '2009-12-18', '2026-09-01'})
# Claims printed without a structured date: multi-day meetings, spans, year- or month-only statements, undated lists and
# recollections.
UNDATED = tuple(cid for cid, e in EVENTS.items() if e[0] is None)
# Rows whose source names nobody as the holder concerned: holder_name null.
NAMELESS = tuple(cid for cid, e in EVENTS.items() if e[3] is None)
# Dossier claim and source ids renamed, split, merged, replaced or withdrawn by the checks, and a field the house format
# bans; none may remain.
STALE_IDS = (
    'in_bjp_advani_assumed_office_two_days_before_20041018', 'in_bjp_gadkari_styled_national_president_profile_20091218',
    'in_bjp_jana_krishnamurthi_has_taken_charge_of_presidentship_20010329',
    'in_bjp_jana_krishnamurthy_national_president_address_20010324', 'in_bjp_laxman_resignation_letter_dated_20010313',
    'in_bjp_naidu_profile_president_from_1_july_2002', 'in_bjp_nc_bangalore_joshi_proposed_name_stewardship_19930618',
    'in_bjp_ne_calcutta_presidential_address_19930410',
    'in_bjp_ne_resolution_bjp_president_advani_delhi_statehood_19900406',
    'in_bjp_ne_resolution_bjp_president_rath_yatra_advani_arrested_19901109',
    'in_bjp_ne_resolution_ex_president_joshi_19931218', 'in_bjp_ne_resolution_our_president_advani_19931218',
    'in_bjp_ne_resolution_our_president_joshi_ekta_yatra_19920313',
    'in_bjp_ne_resolution_release_joshi_president_bjp_19921223',
    'in_bjp_presidents_list_amit_shah_2014_2017_2017_2020_20251120', 'in_bjp_presidents_list_gadkari_2010_2013_20251120',
    'in_bjp_presidents_list_gadkari_2010_to_2013_20140117', 'in_bjp_presidents_list_nadda_2020_present_20251120',
    'in_bjp_presidents_list_rajnath_singh_2013_2014_20251120',
    'in_bjp_presidents_list_rajnath_singh_2013_to_present_20140117',
    'in_bjp_rajnath_singh_presidential_address_after_taking_over_again_20130302',
    'in_bjp_rajnath_singh_responsibility_placed_after_convention_20060120',
    'in_bjp_valedictory_joshi_to_assume_presidency_tomorrow_19910201',
    'in_ks_amit_shah_became_president_july_2014_recalled_20190702', 'in_ks_amit_shah_felicitation_speech_20200120',
    'in_ks_amit_shah_today_nadda_takes_over_20200120', 'in_ks_former_national_presidents_named_20260120',
    'in_ks_life_sketch_nadda_working_president_period_20190617_20200120',
    'in_ks_nabin_assumes_charge_working_president_20251215', 'in_ks_nabin_elected_unopposed_national_president_20260120',
    'in_ks_nabin_nominations_sole_candidate_20260119', 'in_ks_nabin_styled_national_president_20260901',
    'in_ks_nadda_elected_took_charge_20200120', 'in_ks_nadda_elected_unopposed_national_president_20200120',
    'in_ks_nadda_life_sketch_20200211', 'in_ks_nadda_took_charge_20200120_recalled_20230130',
    'in_ks_nadda_working_president_from_20190617_life_sketch',
    'in_ks_national_executive_extends_president_tenure_20230117',
    'in_ks_national_president_meets_ambassadors_20260901', 'attested_period')
# Event kinds of the three research parts that one vocabulary replaced; no row may use them.
OLD_KINDS = {
    'acceptance_speech_heading', 'acceptance_statement', 'announced_assumption_of_office', 'announced_handover',
    'appointment', 'assumption_of_charge', 'assumption_stated_relative_day', 'assumption_stated_retrospective_period',
    'attestation_member_statement', 'attestation_ministerial_answer', 'attestation_question_and_answer',
    'election_acknowledged', 'election_completed', 'election_process_stated_prospective', 'election_stated_by_holder',
    'election_unopposed_stated', 'farewell_of_outgoing_holder', 'first_press_conference_as_president',
    'former_holder_reference', 'former_president_designation', 'handover_of_charge', 'in_office_act',
    'in_office_attestation_party_press_release', 'in_office_attestation_party_resolution', 'in_office_attestation_period',
    'in_office_attestation_presidential_address', 'national_council_presidential_address', 'nomination_sole_candidate',
    'outgoing_president_reference', 'outgoing_president_resolution', 'predecessor_reference_and_nomination',
    'presidential_address_compilation', 're_election_reported', 'reelection_acknowledged',
    'resignation_accepted_with_immediate_effect', 'resignation_letter_considered',
    'resignation_recorded_by_national_executive', 'resignation_tendered_recalled', 'retrospective_attestation',
    'retrospective_end_stated', 'retrospective_list_entry', 'retrospective_statement', 'selection_recalled_undated',
    'start_stated_in_contemporaneous_profile', 'stated_acceptance_of_office', 'styled_president_while_acting'}
# Leads, declined records, per-request or post-cutoff renderings and secondary sources: never a source URL.
LEAD_URL_MARKERS = ('wikipedia', 'posts/41236', 'meets-with-the-ambassadors', '_fields=', 'nr2a13', 'offmem',
                    'advaniji.htm', 'history/history.html', 'leader/index.html', 'profiles/joshi.html', 'ID_161_05121991',
                    'ID_169_08121993', 'ID_169_09121993', 'IQ_161_18121991_U2927', 'July%200102a', 'Nov_1_p_12',
                    'Aug-2000b1', 'Contents_jan_0206', '20260122033851', '20260126175154', '20251217103812',
                    '20251225171922', 'Presidential%20Speech%20Sh%20Rajnath', 'bjp.org/shri-nitin-gadkari',
                    'content/view/3096', 'id=8509', 'national-council-meeting-at-jawaharlal-nehru-stadium',
                    'national-president-s-election-at-11-ashoka-road', 'press-bjp-national-president-shri-amit-shah',
                    'posts/10341', 'posts/10343', 'posts/40433', 'handle/123456789/276', 'handle/123456789/468',
                    'webarchive.loc.gov', 'sansad.in', 'eparlib', 'KS_ENG_Feb%202024_2', 'Thakre-Speech', 'kusha.htm',
                    'Bangaru-bio', 'JanaBio-data', 'Feb2802', 'June%202701', 'June%202402a', 'July%202102',
                    'Office_Bearers_1', 'sep_1605', 'dec_2805', 'jan_0206_p.htm', 'office-bearers_jan_2506')
# URL patterns of responses generated per request, growing listings, searches, cache-busting queries or session state.
PER_REQUEST = re.compile(r'[?&](cb|_|s|q|token|sessionid|search|checkcb)=|reviewcb|X-Amz-|Signature=|cdx/search|'
                         r'email-protection|cdn-cgi|press-releases/page/|/search|nocache|form_build_id|ASPSESSION|'
                         r'__VIEWSTATE|_fields=', re.I)
OFFICIAL_HOSTS = {'library.bjp.org', 'bucketapi.rajyasabha.digital'}
REPORT = research.RESEARCH / 'india-bjp-presidents-1990-2026-27.md'
# CLAUDE-C01-33 (claimed while stacked on this packet, now based on integration) adds one organization observation, the
# Janata Dal row of the Election Commission's national-party table of 10 January 1998, with one party role, and appends
# its sources after this packet's; its own test pins them. This packet's assertions are unchanged for its own records.
C01_33_ORG = 'in_eci_19980110_np_06'
C01_33_ROLE = 'in_jd_president'
C01_33_COUNTS = (22, 41)
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-27.md'


def party_role(packet):
    org, = [o for o in packet['organizations'] if o['id'] == ORG]
    return org, org['roles'][0]


def inc_role(packet):
    org, = [o for o in packet['organizations'] if o['id'] == c01_20.ORG]
    return org['roles'][0]


def load_rows(packet):
    rows = {}
    for source in packet['sources']:
        if source['id'] in RESPONSES:
            extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def bjp_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = [o for o in packet['organizations'] if o['id'] == ORG]
    assert len(orgs) == 1
    org = orgs[0]
    # The recognition observation keeps its identity, lifecycle and empty game mapping.
    assert (org['name'], org['kind'], org['jurisdiction']) == ('Bharatiya Janata Party',
                                                               'political_party_national_recognition_observation', 'India')
    assert org['source_identifier']['value'] == ORG and org['recognition']['attested_on'] == '2024-03-23'
    assert org['recognition']['level'] == 'national'
    assert org['recognition']['from'] is None and org['recognition']['until'] is None
    assert org['lifecycle'] == {'status': 'unresearched', 'from': None, 'until': None,
                                'note': 'Recognition attestation is not an organizational foundation, dissolution or '
                                        'name-change date.'}
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['coverage']['status'] == 'reporting_identity_only'
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one role on the Bharatiya Janata Party observation'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (TITLE, 'party_leader')
    leaders = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
               if r['kind'] == 'party_leader' or r['id'] == ROLE]
    # CLAUDE-C01-33's party role on the Janata Dal observation is the only party-leader role added after this one.
    assert leaders == [(ORG, ROLE), (c01_20.ORG, c01_20.ROLE), (C01_33_ORG, C01_33_ROLE)], \
        'the three party-leader roles only, and no copy of this one'
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    # Party office and state office never feed each other, and the Congress role shares nothing with this one.
    bjp_claims = set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
    bjp_sources = set(role['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
    for inst in packet['institutions']:
        i_claims = set(inst['claim_ids']) | {c for r in inst['roles'] for c in r['claim_ids']} | {
            c for r in inst['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        i_sources = set(inst['sources']) | {s for r in inst['roles'] for s in r['sources']} | {
            s for r in inst['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not bjp_claims & i_claims and not bjp_sources & i_sources, inst['id']
        for r in inst['roles']:
            assert not {h['name'] for h in r['holder_claims']} & set(SURNAMES), (inst['id'], 'cross-institution holder')
    for entry in packet['organizations']:
        if entry is not org:
            assert not set(entry['claim_ids']) & bjp_claims and not set(entry['sources']) & bjp_sources, entry['id']
            for r in entry['roles']:
                assert not set(r['claim_ids']) & bjp_claims and not set(r['sources']) & bjp_sources, r['id']
                assert not {h['name'] for h in r['holder_claims']} & set(SURNAMES), (r['id'], 'cross-party holder')
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
        if name in WORKING:
            assert not WORKING[name][0] <= dated[0] <= WORKING[name][1], (name, 'a working presidency is never a holder')
        if name == ACTING[0]:
            assert not ACTING[1] <= dated[0] <= ACTING[2], (name, 'acting service is never a holder')
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
        # Each observation cites only in-office attestations of its own day; a start cites only its stated day.
        assert attest_days == ({holder['attested_on']} if holder['attested_on'] else set()), (name, 'observation')
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


def bjp_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    bjp_rules(packet, rows)
    _org, role = party_role(packet)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    # Distinct dated events stay distinct and keep their own days.
    for cid, (day, _kind, _obs, _name) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # The stacked prime-minister, president and Congress-president holders are unchanged.
    pm, presidency = packet['institutions']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pm['roles'][0]['holder_claims']] == c01_11.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in presidency['roles'][0]['holder_claims']] == c01_15.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in inc_role(packet)['holder_claims']] == c01_20.HOLDERS
    assert inc_role(packet)['claim_ids'] == list(c01_20.EVENTS)


class IndiaBjpPresidentsTests(unittest.TestCase):
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (74, 167))
        pm, presidency = self.packet['institutions']
        inc = inc_role(self.packet)
        jd, = [o for o in self.packet['organizations'] if o['id'] == C01_33_ORG]
        self.assertEqual(([r['id'] for r in jd['roles']], len(jd['sources']), len(jd['claim_ids'])),
                         ([C01_33_ROLE],) + C01_33_COUNTS)
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(ORIGINAL_SOURCES) + pm['sources'] + presidency['sources'] + inc['sources'] + NEW_SOURCES
                         + jd['sources'])
        self.assertEqual((pm['sources'], presidency['sources'], inc['sources']),
                         (c01_11.NEW_SOURCES, c01_15.NEW_SOURCES, c01_20.NEW_SOURCES))
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (85, 5))
        self.assertEqual(len(self.packet['organizations']), 83)
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual(sorted(holder_claims), sorted(HOLDER_OBSERVATIONS))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER), len(HOLDERS)), (21, 146, 19))
        self.assertEqual(list(EVENTS), self.new_claims)
        for group, kinds in GROUPS:
            self.assertEqual({EVENTS[cid][1] for cid in group}, kinds)
        # Every new claim and source is cited by the party role and its recognition observation, and by nothing else.
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.org['claim_ids'], ['in_eci_20240323_np_row_03'] + self.new_claims)
        self.assertEqual(self.org['sources'], ['in_eci_national_parties_20240323'] + NEW_SOURCES)
        for entry in self.packet['organizations'] + self.packet['institutions']:
            if entry is not self.org:
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
                self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
                for role in entry['roles']:
                    self.assertFalse(set(self.new_claims) & set(role['claim_ids']), role['id'])
        # At most ten observations, BJP-PRES-01..10, each reported and carrying rows; no row reuses another packet's numbers.
        observations = re.findall(r'^### (BJP-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({self.rows[cid]['review_observation'] for cid in self.new_claims}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        for cid in self.new_claims:
            self.assertNotRegex(self.rows[cid]['review_observation'], r'^(IN-(PRES|PM)|INC-PRES)-', cid)
        for stale in STALE_IDS:
            self.assertNotIn(f'"{stale}"', self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(f'"{stale}"', json.dumps(extract, ensure_ascii=False), stale)
        kinds = {self.rows[cid]['event_kind'] for cid in self.new_claims}
        self.assertFalse(kinds & OLD_KINDS)
        self.assertEqual(kinds, HOLDER_KINDS | FROM_KINDS | UNTIL_KINDS | NEVER_KINDS)
        self.assertFalse(set(self.sources) & set(self.claims), 'no id is both a source and a claim')

    def test_holders_are_exactly_as_intended(self):
        bjp_invariants(self.packet, self.rows)
        phrase = {'in_office_attestation': 'Dates the holder observation', 'assumption_statement': 'Dates the holder start',
                  'end_of_office_stated': 'Dates the holder end'}
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            if holder['from']:
                self.assertTrue(holder['note'].startswith('From '), holder['name'])
                self.assertIn('The start is day-only', holder['uncertainty'], holder['name'])
            else:
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
                self.assertRegex(holder['uncertainty'], r'No start', holder['name'])
            if not holder['until']:
                self.assertRegex(holder['uncertainty'] + holder['note'], r'No end', holder['name'])
            else:
                # The atlas shows only 'Observed on' for a holder with attested_on, so the note carries the end.
                day = int(holder['until'][8:])
                self.assertIn('this note carries the end', holder['note'])
                self.assertRegex(holder['note'], rf'end: {day} \w+ {holder["until"][:4]}')
            for cid in holder['claim_ids']:
                self.assertIn(phrase[self.rows[cid]['event_kind']], self.claims[cid]['uncertainty'], cid)
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        for cid in NEVER_HOLDER:
            self.assertNotIn('Dates the holder', self.claims[cid]['uncertainty'], cid)
        for cid in self.new_claims:
            row = self.rows[cid]
            self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), (ORG, ROLE, TITLE), cid)
            self.assertEqual(row['holder_name'], EVENTS[cid][3], cid)
        # Holder names are normalised; the printed forms stay in the claim text. Rows whose source names nobody are null.
        self.assertEqual({self.rows[cid]['holder_name'] for cid in self.new_claims} - {None}, set(SURNAMES))
        self.assertEqual(len(NAMELESS), 13)
        for cid in NAMELESS:
            self.assertIn(self.rows[cid]['event_kind'], NEVER_KINDS, cid)
        printed = {
            'in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324': 'Jana Krishnamurthy National President',
            'in_bjp_thakre_outgoing_president_present_20000802': 'Khushubhau Thakre',
            'in_bjp_national_president_election_process_completed_20130123': 'श्री राजनाथ सिंह जी',
            'in_bjp_amit_shah_nomination_filed_national_president_20160124': 'श्री अमित भाई शाह',
            'in_rs_sahay_advani_president_bjp_rath_yatra_19910108': "'AHLUWAUA'",
            'in_bjp_ne_resolution_assault_bjp_president_joshi_19930227': "'Assault on Dr. MM Joshil' (sic)",
            'in_bjp_naidu_profile_span_ist_july_2002_onwards': "'Ist July 2002 onwards President",
            'in_bjp_rajnath_singh_took_over_again_recalled_2013': 'aftertaking',
            'in_rs_jacob_answer_bjp_president_joshi_flag_hoisting_19920304': 'डा० मुरलीमनोहर जोशी',
        }
        for cid, text in printed.items():
            self.assertIn(text, self.claims[cid]['text'], cid)
        # Addresses whose own text names no speaker never take a name from the 2016 compilations' section headings.
        for cid in ('in_bjp_outgoing_president_farewell_five_year_tenure_199101',
                    'in_bjp_nc_jaipur_new_president_accepts_presidentship_19910201',
                    'in_bjp_nc_bangalore_elected_once_again_president_19930618',
                    'in_bjp_nc_mumbai_elected_president_yet_another_term_19951110',
                    'in_bjp_ne_new_delhi_last_meeting_during_tenure_19980411',
                    'in_bjp_ne_gandhinagar_i_as_president_19980502',
                    'in_bjp_ne_gandhinagar_handover_announced_for_tomorrow_19980502',
                    'in_bjp_ne_gandhinagar_farewell_as_party_president_19980502'):
            self.assertIsNone(self.rows[cid]['holder_name'], cid)
        self.assertIn("'DR. MM JOSHI' at p.28, 'SHRI LK ADVANI' at p.85",
                      self.claims['in_bjp_outgoing_president_farewell_five_year_tenure_199101']['uncertainty'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims, rows = self.claims, self.rows
        by_kind = {k: [cid for cid in self.new_claims if rows[cid]['event_kind'] == k] for k in FROM_KINDS | UNTIL_KINDS}
        self.assertEqual(by_kind['assumption_statement'],
                         ['in_bjp_today_naidu_took_over_on_july_1_20020701', 'in_ks_nadda_took_charge_20200120',
                          'in_bjp_nadda_says_nabin_assumed_charge_today_20260120',
                          'in_ks_nabin_assumes_charge_national_president_20260120'])
        self.assertEqual(by_kind['end_of_office_stated'],
                         ['in_bjp_office_bearers_accept_laxman_resignation_immediate_effect_20010314'])
        for cid, words in (('in_bjp_today_naidu_took_over_on_july_1_20020701', 'took over from Shri Jana Krishnamurthi on '
                                                                               'July 1'),
                           ('in_ks_nadda_took_charge_20200120', 'took charge from the outgoing'),
                           ('in_bjp_nadda_says_nabin_assumed_charge_today_20260120', 'has assumed charge as the National'),
                           ('in_ks_nabin_assumes_charge_national_president_20260120', 'formally assumed charge'),
                           ('in_bjp_office_bearers_accept_laxman_resignation_immediate_effect_20010314',
                            'accept the resignation with immediate effect')):
            self.assertIn(words, claims[cid]['text'], cid)
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
        # Handovers, farewells, 'outgoing' or 'former' stylings, resignations accepted without effect or rejected, and
        # decisions not to seek another term are never an end; acting and working service are never a holder.
        for cid in ENDINGS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never an (until|end)|never a holder date|never a start', cid)
        for cid in INTERIM:
            self.assertIn('claim only, never a holder', claims[cid]['uncertainty'], cid)
        for cid in ELECTIONS + RETROSPECTIVE + UNNAMED + CONTINUATION + CONTEXT:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never (a|dates? a) holder|never feeds|cannot feed|'
                                                         r'never a (start|boundary)|never boundaries|claims only|a claim only',
                             cid)
        for cid in ('in_ks_amit_shah_hands_over_charge_20200120', 'in_ks_nadda_hands_over_charge_20260120',
                    'in_ks_amit_shah_today_nadda_takes_over_2020',
                    'in_bjp_ne_gandhinagar_handover_announced_for_tomorrow_19980502'):
            self.assertEqual(rows[cid]['event_kind'], 'handover_statement', cid)
            self.assertIn('never an until', claims[cid]['uncertainty'], cid)
        for cid in ('in_bjp_advani_assumed_office_two_days_ago_recalled_20041018', 'in_ks_nadda_took_charge_20200120_recalled'):
            self.assertEqual(rows[cid]['event_kind'], 'assumption_recalled_retrospective', cid)
            self.assertIn('CLAUDE-C01-15 check C1', claims[cid]['uncertainty'], cid)
        self.assertNotIn('immediate effect', claims['in_bjp_naidu_resignation_accepted_20041018']['text'])
        self.assertIn("'Ist July 2002 onwards President'", self.role['holder_claims'][5]['note'])
        scope = self.role['scope_note']
        for phrase in ('no in_prime_minister or in_presidency claim or source feeds this role',
                       'none of its claims feeds either institution', 'in_inc_president is untouched',
                       'Do not fill the interval', "infer an outgoing holder's last day from a successor's election",
                       'are claims only, never holders', 'procedure only, never a date', 'no structured date is stored',
                       'Parliament records are used only where they record the party office', 'holder_name null',
                       'multi-day meeting headings give no structured date'):
            self.assertIn(phrase, scope)
        self.assertTrue(self.org['coverage']['unresolved'][-1].startswith('BJP Presidents 1990-2026 (CLAUDE-C01-27)'))
        self.assertEqual(self.org['coverage']['unresolved'][:-1], [
            'Reconcile exact organization identity, jurisdiction and name variants before mapping to the game.',
            'Research founding, splits, mergers, dissolution and separately dated party offices from 1990 through the fixed cutoff.',
            'Review later amendments and any identity or status qualifications; no current legal conclusion is drawn.'])
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-27' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][10].startswith('BJP Presidents 1990-2026 (CLAUDE-C01-27, BJP-PRES-01..10)'))
        self.assertTrue(coverage['unresolved'][9].startswith('INC Presidents 1990-2026 (CLAUDE-C01-20, INC-PRES-01..10)'))
        self.assertTrue(coverage['unresolved'][-1].startswith('Janata Dal Presidents 1990-2026 (CLAUDE-C01-33, '
                                                              'JD-PRES-01..08)'))
        self.assertEqual(len(coverage['unresolved']), 12)
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])

    def test_party_and_state_offices_stay_separate(self):
        pm, presidency = self.packet['institutions']
        inc = inc_role(self.packet)
        for entry in (pm, presidency):
            self.assertFalse(set(entry['claim_ids']) & set(self.new_claims), entry['id'])
            self.assertFalse(set(entry['sources']) & set(NEW_SOURCES), entry['id'])
            for role in entry['roles']:
                for holder in role['holder_claims']:
                    self.assertFalse(set(holder['claim_ids']) & set(self.new_claims), holder['name'])
                    self.assertNotIn(holder['name'], SURNAMES)
        self.assertFalse(set(inc['claim_ids']) & set(self.new_claims))
        self.assertFalse(set(inc['sources']) & set(NEW_SOURCES))
        self.assertFalse({h['name'] for h in inc['holder_claims']} & set(SURNAMES))
        for cid in self.new_claims:
            self.assertEqual(self.rows[cid]['role_id'], ROLE, cid)
        # Government, parliamentary and ministerial offices named in party records are context only.
        self.assertIn('The Home Ministry is a separate office',
                      self.claims['in_ks_amit_shah_styled_national_president_at_board_meeting_20190617']['uncertainty'])
        self.assertIn('Leader of Opposition, a parliamentary office outside this role',
                      self.claims['in_bjp_ne_resolution_release_joshi_president_bjp_199212']['uncertainty'])
        self.assertIn('The Lok Sabha office is outside this role',
                      self.claims['in_bjp_advani_styled_president_after_resignation_episode_20050615']['uncertainty'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-09-28', sid)
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for phrase in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                           'at least 30 minutes apart'):
                self.assertIn(phrase, extract['provenance_note'], sid)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-27 only', extract['bounded_scope'])
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
            self.assertNotIn('source_response_content_encoding', extract)
            self.assertNotIn('mirror_of', extract)
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
            for phrase in ('Raw Internet Archive capture', 'Accept-Encoding: identity', 'default User-Agent'):
                self.assertIn(phrase, extract['provenance_note'], sid)
        for sid, stamp in ARQUIVO.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(urlsplit(source['url']).hostname, 'arquivo.pt')
            self.assertTrue(urlsplit(source['url']).path.startswith(f'/wayback/{stamp}id_/https://www.bjp.org/'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(source['url'].split('id_/', 1)[1], source['original_url'])
            self.assertIn('Raw Arquivo.pt capture', extract['provenance_note'])
        for sid in LIBRARY:
            url = urlsplit(self.sources[sid]['url'])
            self.assertEqual(url.hostname, 'library.bjp.org', sid)
            self.assertRegex(url.path, r'^/jspui/bitstream/123456789/\d{3}/\d/[^/]+\.pdf$', sid)
            self.assertIn("e-Library", self.extracts[sid]['provenance_note'])
        for sid in STORE:
            self.assertEqual(urlsplit(self.sources[sid]['url']).hostname, 'bucketapi.rajyasabha.digital', sid)
            self.assertIn('ETag equals the MD5 of the body', self.extracts[sid]['provenance_note'])
        self.assertEqual((len(ARCHIVED), len(ARQUIVO), len(LIBRARY), len(STORE)), (63, 1, 6, 4))
        self.assertEqual(set(ARCHIVED) | set(ARQUIVO) | set(LIBRARY) | set(STORE), set(NEW_SOURCES))
        for sid in set(LIBRARY) | set(STORE):
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
        # The one undated page carries its dating record as a date anchor with its own response identity.
        self.assertEqual({sid for sid in NEW_SOURCES if 'date_anchor' in self.extracts[sid]}, set(DATE_ANCHORS))
        for sid, (name, size, sha) in DATE_ANCHORS.items():
            anchor = self.extracts[sid]['date_anchor']
            self.assertEqual((anchor['source_response_bytes'], anchor['source_response_sha256']), (size, sha))
            self.assertEqual(urlsplit(anchor['url']).hostname, 'bucketapi.rajyasabha.digital')
            self.assertIn(name, anchor['url'])
            self.assertIn("'[ 8 JAN. 1991 ]'", anchor['note'])

    def test_response_identities_are_reproducible_urls(self):
        urls = [self.sources[sid]['url'] for sid in NEW_SOURCES]
        urls += [self.extracts[sid]['date_anchor']['url'] for sid in DATE_ANCHORS]
        for url in urls:
            self.assertIsNone(PER_REQUEST.search(url), url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if parts.hostname == 'web.archive.org':
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
            elif parts.hostname == 'arquivo.pt':
                self.assertRegex(parts.path, r'^/wayback/\d{14}id_/https://', url)
            else:
                self.assertIn(parts.hostname, OFFICIAL_HOSTS, url)
            # The Rajya Sabha store's query only sets the served content type and file name; library files have none.
            if parts.hostname == 'bucketapi.rajyasabha.digital':
                self.assertEqual(sorted(k.split('=')[0] for k in parts.query.split('&')),
                                 ['response-content-disposition', 'response-content-type'])
            elif parts.hostname == 'library.bjp.org':
                self.assertEqual(parts.query, '', url)
            # The live party site and the party organ's live WordPress renderings are never a recorded identity: only
            # fixed pre-cutoff captures of them.
            if re.search(r'(^|[/.])(bjp\.org|kamalsandesh\.org)', parts.netloc + parts.path.split('id_/', 1)[0]):
                self.assertIn(parts.hostname, {'web.archive.org', 'arquivo.pt', 'library.bjp.org'}, url)
            if 'kamalsandesh.org' in url or 'www.bjp.org' in url or 'bjp.org/' in url:
                self.assertIn(parts.hostname, {'web.archive.org', 'arquivo.pt', 'library.bjp.org'}, url)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, self.sources[sid]['url'], sid)
                self.assertNotIn(marker, self.extracts[sid].get('original_url', ''), sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'britannica', 'the hindu', 'indianexpress', 'ndtv', 'hindustantimes', 'timesofindia',
                       'news18', 'reuters', 'bbc.', 'tribune'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('posts/41236', 'nr2a13', 'offmem', 'ID_161_05121991', 'IQ_161_18121991_U2927', 'Nov_1_p_12',
                       'Aug-2000b1', 'Contents_jan_0206', '20260122033851', '20260126175154', '20251225171922',
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
            (lambda p: source(p, 'in_bjp_elib_party_document_vol5_political_resolutions')['snapshot'].update(
                sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'in_kamal_sandesh_vol21_no02_20260116')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'in_bjp_site_returning_officer_statement_20260119')['snapshot'].update(
                path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 18).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 18).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'in_ks_nabin_appoints_national_office_bearers_20260817').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: claim(p, 'in_bjp_presidents_list_nadda_2020_present').update(
                period={'from': '2020-01-20', 'through': '2026-09-30'}), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update(until='2002-01-01'), 'Reversed historical interval'),
            (lambda p: holder(p, 3)['claim_ids'].append('in_bjp_laxman_styled_president_maiden_press_conference_20000831'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('in_does_not_exist'), 'Unknown'),
            (lambda p: org(p).update(represented_party_ids=['India/guessed_bjp']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        rule_cases = [
            # A successor's observation or start used as an end.
            ('successor observation used as an end (Advani 1991)', lambda p: holder(p, 0).update(until='1991-12-10')),
            ('successor start used as the end of the acting service (Jana Krishnamurthi)',
             lambda p: role(p)['holder_claims'].insert(5, extra_holder(
                 JK, '2001-03-24', 'in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324',
                 until='2002-07-01'))),
            ('successor start cited as an end (Amit Shah)',
             cite(13, 'in_ks_nadda_took_charge_20200120', until='2020-01-20')),
            ('successor start used as an end (Nadda)', lambda p: holder(p, 16).update(until='2026-01-20')),
            ('an end invented at the cutoff (Nabin)', lambda p: holder(p, 18).update(until='2026-09-07')),
            # A handover, farewell, 'outgoing' or 'former' styling, resignation or decision used as an end.
            ('handover cited as an end (Amit Shah)',
             cite(13, 'in_ks_amit_shah_hands_over_charge_20200120', until='2020-01-20')),
            ('handover cited as an end (Nadda)', cite(16, 'in_ks_nadda_hands_over_charge_20260120', until='2026-01-20')),
            ('ex-President styling cited as an end (Nadda)',
             cite(16, 'in_bjp_nadda_styled_ex_national_president_20260120', until='2026-01-20')),
            ('farewell day used as an end (Advani 1998)', lambda p: holder(p, 2).update(until='1998-05-02')),
            ('announced handover day used as an end (Advani 1998)', lambda p: holder(p, 2).update(until='1998-05-03')),
            ('resignation accepted without effect cited as an end (Naidu)',
             cite(5, 'in_bjp_naidu_resignation_accepted_20041018', until='2004-10-18')),
            ('rejected resignation used as an end (Advani 2005)', lambda p: holder(p, 6).update(until='2005-06-08')),
            ('decision not to seek a second term cited as an end (Gadkari)',
             cite(9, 'in_bjp_gadkari_decides_not_to_seek_second_term_20130122', until='2013-01-22')),
            ('former styling used as an end (Gadkari)', lambda p: holder(p, 9).update(until='2013-01-27')),
            ('outgoing styling used as an end (Rajnath Singh 2014)', lambda p: holder(p, 10).update(until='2014-07-09')),
            ('tender of resignation used as the end (Laxman)', lambda p: holder(p, 4).update(until='2001-03-13')),
            ('stated end removed but until kept (Laxman)', lambda p: holder(p, 4)['claim_ids'].pop()),
            ('until dropped while its claim is cited (Laxman)', lambda p: holder(p, 4).update(until=None)),
            # An election, nomination, declaration, appointment, recollection or undated assumption used as a start.
            ('declaration used as a start (Rajnath Singh 2013)',
             lambda p: holder(p, 10).update({'attested_on': None, 'from': '2013-01-23'})),
            ('declaration cited by a start (Nabin)', cite(17, 'in_ks_laxman_declares_nabin_elected_20260120')),
            ('recollection used as a start (Advani 2004)',
             lambda p: holder(p, 6).update({'attested_on': None, 'from': '2004-10-18'})),
            ('appointment cited as a start (Advani 2004)',
             cite(6, 'in_bjp_advani_appointed_president_ratification_pending_20041018',
                  **{'attested_on': None, 'from': '2004-10-18'})),
            ('recollection cited by a start (Nadda 2023 recollection)',
             cite(14, 'in_ks_nadda_took_charge_20200120_recalled')),
            ('profile span cited by a start (Naidu)', cite(5, 'in_bjp_naidu_profile_span_ist_july_2002_onwards')),
            ('undated assumption used as a start (Thakre)',
             lambda p: holder(p, 3).update({'attested_on': None, 'from': '1998-05-03'})),
            ('retrospective list used as a start (Gadkari)',
             lambda p: holder(p, 8).update({'attested_on': None, 'from': '2010-01-01'})),
            ('stated start replaced by a continuation (Naidu)', lambda p: holder(p, 5).update(
                claim_ids=['in_bjp_naidu_president_first_press_conference_20020711'],
                sources=['in_bjp_site_naidu_first_press_conference_20020711'])),
            ('President-elect styling used as an observation (Thakre)', lambda p: holder(p, 3).update(attested_on='1998-05-02')),
            ('newly elected styling used as an observation (Rajnath Singh 2013)',
             lambda p: holder(p, 10).update(attested_on='2013-01-23')),
            ('election result cited by a holder (Amit Shah 2016)', cite(12, 'in_bjp_amit_shah_re_elected_heading_20160124')),
            ('continuation claim cited by a holder (Joshi)', cite(1, 'in_bjp_ne_resolution_assault_bjp_president_joshi_19930227')),
            ('extension cited by a holder (Nadda 2023)',
             cite(15, 'in_ks_national_executive_extends_nadda_tenure_to_june_2024_20230117')),
            ('multi-day meeting added as a holder (Advani 1990)', lambda p: role(p)['holder_claims'].insert(
                0, extra_holder(A, '1990-07-21', 'in_bjp_ne_resolution_authorises_party_president_advani_199007'))),
            ('unnamed address added as a holder (1993)', lambda p: role(p)['holder_claims'].insert(
                2, extra_holder(A, '1993-06-18', 'in_bjp_nc_bangalore_elected_once_again_president_19930618'))),
            # Acting or working service added as a holder.
            ('acting service added as a holder (Jana Krishnamurthi)', lambda p: role(p)['holder_claims'].insert(
                5, extra_holder(JK, '2001-03-18', 'in_bjp_jana_krishnamurthy_styled_president_20010318'))),
            ('acting service observed on 24 March 2001 added as a holder (Jana Krishnamurthi)',
             lambda p: role(p)['holder_claims'].insert(5, extra_holder(
                 JK, '2001-03-24', 'in_bjp_jana_krishnamurthy_styled_national_president_ne_address_20010324'))),
            ('acting designation day used as an observation (Jana Krishnamurthi)',
             lambda p: role(p)['holder_claims'].insert(5, extra_holder(
                 JK, '2001-03-14', 'in_bjp_jana_krishnamurthi_designated_acting_president_20010314'))),
            ('working presidency added as a holder (Nadda 2019)', lambda p: role(p)['holder_claims'].insert(
                14, extra_holder(JPN, '2019-06-17', 'in_ks_parliamentary_board_appoints_nadda_working_president_20190617'))),
            ('working presidency added as a holder (Nabin 2025)', lambda p: role(p)['holder_claims'].insert(
                17, extra_holder(NN, '2025-12-15', 'in_ks_nabin_assumes_working_presidency_20251215'))),
            ('working presidency used as a start (Nadda)', lambda p: holder(p, 14).update({'from': '2019-06-17'})),
            # Cross-role, cross-party and cross-institution holders and claims.
            ('prime minister added as a party President', lambda p: role(p)['holder_claims'].append(
                extra_holder('Narendra Modi', '2024-06-09', 'in_modi_appointed_pm_communique_20240609'))),
            ('President of India added as a party President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][7]))),
            ('Congress President added as a BJP President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(inc_role(p)['holder_claims'][9]))),
            ('party President moved into the prime-ministership',
             lambda p: pm_role(p)['holder_claims'].append(role(p)['holder_claims'].pop(5))),
            ('party President added to the presidency',
             lambda p: president_role(p)['holder_claims'].append(copy.deepcopy(holder(p, 18)))),
            ('party President added to the Congress role',
             lambda p: inc_role(p)['holder_claims'].append(copy.deepcopy(holder(p, 8)))),
            ('prime-ministership claim cited by a party President', cite(7, 'in_rao_appointed_pm_wef_19910621')),
            ('party claim moved onto the prime-ministership', lambda p: (
                pm_role(p)['claim_ids'].append('in_bjp_press_release_president_advani_19970716'),
                pm_role(p)['sources'].append('in_bjp_org_swarna_jayanti_press_release_19970716'))),
            ('party claim moved onto the Congress role', lambda p: (
                inc_role(p)['claim_ids'].append('in_bjp_press_release_president_advani_19970716'),
                inc_role(p)['sources'].append('in_bjp_org_swarna_jayanti_press_release_19970716'))),
            ('party role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('party role copied onto an institution',
             lambda p: p['institutions'][0]['roles'].append(dict(copy.deepcopy(role(p)), id='in_bjp_president_2'))),
            ('second role on the observation',
             lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='in_bjp_general_secretary'))),
            ('role kind changed', lambda p: role(p).update(kind='head_of_government')),
            ('observation lifecycle given a start', lambda p: org(p)['lifecycle'].update({'from': '1980-04-06'})),
            ('observation mapped to a game party', lambda p: org(p).update(represented_party_ids=['India/bjp'])),
            ('observation coverage upgraded', lambda p: org(p)['coverage'].update(status='partial')),
            # Structured dates added to undated claims, and order.
            ('span given a structured date', lambda p: claim(p, 'in_bjp_evolution_joshi_president_1991_1993').update(
                attested_on='1991-02-01')),
            ('multi-day meeting given a day', lambda p: claim(p, 'in_bjp_ne_resolution_our_president_advani_199312').update(
                attested_on='1993-12-18')),
            ('span given a period', lambda p: claim(p, 'in_bjp_biodata_advani_president_199307_to_19980502').update(
                attested_period={'from': '1993-07-01', 'through': '1998-05-02'})),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        bjp_rules(self.packet, self.rows)
        bjp_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    bjp_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    bjp_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('in_bjp_returning_officer_announces_only_nabin_proposed_20260119', '2026-01-20'),
                         ('in_ks_nabin_elected_unopposed_stated_20260119', '2026-01-20'),
                         ('in_bjp_laxman_resignation_letter_dated_20010313_recalled', '2001-03-14'),
                         ('in_bjp_advani_elected_by_national_executive_20041027', '2004-10-18'),
                         ('in_bjp_national_president_election_process_completed_20130123', '2013-03-02'),
                         ('in_ks_parliamentary_board_appoints_nabin_working_president_20251214', '2025-12-15'),
                         ('in_bjp_valedictory_joshi_taking_over_tomorrow_19910131', '1991-02-01')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                bjp_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = dict.fromkeys([f'{n:02d}' for n in range(1, 10)], 'Accepted in part')
        decisions['10'] = 'Accepted'
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| BJP-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 18)] + [f'B{n}' for n in range(1, 17)] + [f'C{n}' for n in range(1, 33)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal|Declined|Already recorded)')
        records = [line for line in defects.splitlines() if re.match(r'\| [ABC]-R\d+ ', line)]
        self.assertEqual(len(records), 38)
        for row in records:
            self.assertRegex(row, r'\*\*(Imported|Declined|Already recorded|Used as dating evidence)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('27c52dc4', 'da49f9c0', 'research-index.json', 'test_india_research_s10e.py',
                     'test_india_prime_ministers_c01_11.py', 'test_india_presidents_c01_15.py',
                     'test_india_inc_presidents_c01_20.py', 'test_campaign_census', 'No existing extract'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'india-bjp-presidents-1990-2026-27.md', 'claude/c01-in-27', '27c52dc4',
                     'test_india_bjp_presidents_c01_27.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (2, 5))
        self.assertEqual(country['mapping_pending'], 85)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
