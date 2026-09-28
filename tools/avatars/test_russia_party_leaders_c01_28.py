"""CLAUDE-C01-28: Russian party leaders, 1990-2026, keep congresses, elections and election reports, registrations,
reorganisations, the 2022 death and interim, the 2008 accession and the 2001 dissolution apart from the dated in-office
attestations that alone date a holder, state no start or end that no source states, and keep party offices apart from the
presidency, the Government and the 2021 Duma factions. Five party-leader roles: three on the 2021 ballot-list observations
and two on new organization observations made from the parties' own records, plus the 1993 bloc's own observation."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


EARLIER_SOURCE_COUNT = 170
# Original response identity recorded in each new extract: (bytes, sha256); downloaded twice by the dossier and twice
# by this packet at least 30 minutes apart, byte-identical each time.
RESPONSES = {
    'ru_kprf_i_plenum_notice_19970420': (1757, 'b26af368ce81ef4f7d9c01a9ba88c7c5022017a8d38e8b92aeb95f1c9207bdd5'),
    'ru_kprf_v_congress_notice_19980523': (2959, '8dc7cb0174f2476924cb2764b55998438deac24c2b3420644d6e43b3d9b393d8'),
    'ru_kprf_date_in_history_1993_20260213': (26386, '750fce202fef2eea9b2e70514d0f8926f9e25d6528dce20be0c1138e81aca23b'),
    'ru_kprf_party_reference_20250912': (33284, '6cb3631316ef553130ef831df03da8221b734b9653a13f66e6f46524fa070330'),
    'ru_kprf_xviii_congress_notice_20210424': (29955, '18d432dfbcd78ae8b4d69207244728ace5ecd3a4a0823ec42ff895ef99bb4efd'),
    'ru_kprf_i_plenum_notice_20210424': (24747, '470281b2c1b526e675989adfd19123bcc7fcd59796354b9e552db7dcf558b0a2'),
    'ru_kprf_xix_congress_notice_20250705': (41887, '4414392c9bd4221361d6ccf326f65cb07456a3bb3424ee454e68862416e6edc9'),
    'ru_kprf_i_plenum_notice_20250705': (28783, '19a6e954c0cba77685a09f263198d050fed63f48aa2a18faec5660f62dbb0329'),
    'ru_kprf_official_i_plenum_20250705': (18158, '3aa99b06879c7c6b882488255fa71f930d2309018001ac7a5aa968fb1e65b508'),
    'ru_kprf_zyuganov_greeting_20260828': (31109, 'a2a9026c7f8e4d758b328a3ce6b51a8ac12165d3105cf18d33a06f2f1a0e1cc5'),
    'ru_kprf_official_xix_second_stage_20260620': (15328, '6feb71b7794466210aeb0fd941ce944a37bac65ac35b9c593b34e1cda795a303'),
    'ru_ldpr_newspaper_plenum_resolution_19961123': (21866, '3910baae790f7035b46d30d70d024e8db118947bdf98bd5df59da7087b1cfacb'),
    'ru_ldpr_party_history_20100922': (36369, '693b4c46f11322968cce1625b44206cd616fc268f0dbe512942449ec3e85cdaa'),
    'ru_ldpr_party_history_2026': (245090, '1a65aaacafec3d5af3fdeae3459139f95ce74d36b8faef6ac4cc49b7fba97f78'),
    'ru_ldpr_newspaper_no04_20220330': (3628893, '0140c750569794e10587d5134937ed958b93e4d820ceaba5690a570456b40e35'),
    'ru_ldpr_news_zhirinovsky_death_20220406': (98960, 'cb54c3e62e2d6289f025c4267e7dac2bd2a0dfef827a2765a715357af61b3ca7'),
    'ru_ldpr_newspaper_no05_20220506': (5262052, 'def809b8f76ffb95597a81a36741c2de98e5c4b10e02fb70598d9379119837db'),
    'ru_ldpr_news_supreme_council_recommendation_20220526': (98454, '3bae640922307d0b660ad9822677644db82c87a78d90a5a95062f5d3ec0d6c91'),
    'ru_ldpr_news_xxxiv_congress_opens_20220527': (95630, '02b87926427668f49bafdabf36f8910ba77e417465b47666bba5f424a11413be'),
    'ru_ldpr_news_slutsky_elected_20220527': (97398, 'f7904c4a28c84d93b589a09c7c71d6df62412e5d57624f56a20c799bff05c689'),
    'ru_ldpr_news_vote_day_20220527': (97383, '661c9a01775e9b455f19f6b806f65beb2f7e42fe7df7fb05cd6d62c47b482b52'),
    'ru_ldpr_newspaper_no06_07_20220627': (4385525, '54e52e3fb3ad6fcb5062982fd0911c9d2649c1f2cdfbb19fa74dd89b6a26a07c'),
    'ru_ldpr_news_slutsky_reelected_20251002': (124272, 'e57f89ea3b83cd3cc581022616685c05c42f5f0678f5fc6a6be5974f38b8cbc0'),
    'ru_ldpr_news_slutsky_congress_post_20251002': (110098, '45af9535e5e5dca087b49d176167c4f8d0edbb2367d469a116d6f886fedea8e5'),
    'ru_ldpr_news_citizenship_proposal_20260811': (152667, 'f60abeb96d9b4e8202c0e5cc78a6766772f6a96bdad7e4c5a72455cf4d3cbdec'),
    'ru_yabloko_site_reference_1998': (20333, '699e5af47134de1f71fc6d331cfb510c02cafbde86daf33fd1b3bdaf73e1b014'),
    'ru_yabloko_vi_congress_report_19980314': (27960, '2073831f8ad8b988d57cac09a3a70918903b5cb7a863d60a83538ab37fe61c57'),
    'ru_yabloko_x_congress_section_2001': (49435, '99e5545f0354467aa2539e6016a2f84341bd4f95d284f8261e1fa6e09c6327e9'),
    'ru_yabloko_press_party_transformation_20011222': (4846, '2de4e34003bc29f1fda1a3b238c2a567a45dc01e65874405629ee053b2a389da'),
    'ru_yabloko_press_yavlinsky_elected_20011223': (4974, '861d4f81af6913a3bba9aea6d06fa90f0458992dad5012c065076dbb9ad9a65b'),
    'ru_yabloko_x_congress_chairman_speech_20011223': (10068, '9e97363d8fba6469497619768b9589e91606fb7dbd71425f378aecfa7b26968d'),
    'ru_yabloko_press_congress_schedule_20040703': (7308, '8224f98af3927f3f674258a2995f46717141c279977c5c914c7f1e290010ea8e'),
    'ru_yabloko_press_yavlinsky_reelected_20040703': (6103, 'a925474e81356d4e9641d2be167bbc23bcb3991c6b50d2e2eba6f26aa6c0bc0a'),
    'ru_yabloko_press_charter_amendments_20040704': (8028, '8a4df4bbd74f2425b62603f235a5e9078aff05314323a3c8469765a981a4fb77'),
    'ru_yabloko_leadership_report_2004_2008': (238829, '33605fe72f6f5bc1c913b501262c5184ccac7e16fb3bf088b48ab9246731cd1e'),
    'ru_yabloko_press_new_leadership_20080622': (11691, 'e5f132c5dcb757412424a38967931957427f599312fa11de4d3a72cadacfb088'),
    'ru_yabloko_press_xv_congress_decisions_20080626': (12728, '2bb4b07376397e9e818a4806908422ae1a99fdff29d73588f92c426e607f099a'),
    'ru_yabloko_news_term_limit_20151219': (51785, 'aad04b2c6b5a040b7721e255c2d7f82f3f7758e0ccba6c391fd0dd71cdcdb983'),
    'ru_yabloko_news_slabunova_elected_20151220': (53009, '3938b05732b7371d74e90d08fad34fa2fc3fb620ac458375c91cd05c44520317'),
    'ru_yabloko_news_new_chairman_deputies_20151220': (50818, '410a9fcf920e306ef9f18b3a1a9a97602a59107b11a3951db6e498ec2d5b4017'),
    'ru_yabloko_news_rybakov_elected_20191215': (48691, 'a4f7e57392ae431d6291a80b0e4b580b3dd5f6af1d45e838c985b9deef2b22e3'),
    'ru_yabloko_news_xxi_congress_main_20191216': (58217, '1aedcf739af38cb9bc631d6aa19c72efced17e3b9620faf9b6a6f93b182116f3'),
    'ru_yabloko_news_rybakov_second_term_20231209': (58023, 'dae451e56be9d65878c7f86bae6f81737792ac8e9929382143d1cca995423a63'),
    'ru_yabloko_news_xxii_congress_main_20231211': (51800, 'a2e879fbb23f312047fd0e2706da0d174a098c9abde33f105e29bdf2056d8788'),
    'ru_yabloko_news_fsin_appeal_20231213': (45887, '8140b7fb70d918e8db142eb160ff2295f2feefc40a450a33cd74b67f1b93b818'),
    'ru_yabloko_news_rybakov_appeal_20260819': (49945, '5837c6f8c2f8948255ac5cd741397a8eb40748e92c7c1b23bbc4d5d18ff70950'),
    'ru_apr_history_note_2002': (14361, '66679607a6f07c944035ce3023ac5732dbf78fc54e653f676515fc464323632e'),
    'ru_apr_release_xi_congress_20030910': (35412, '523a89c97d75d4319799bd1e68904d3021afeb188535703471b6474ab536dfe3'),
    'ru_apr_release_new_chairman_20040526': (32743, '918fbe1edec05ad1fd73a97625f6c4b4d00c697ccf28cb5916da399202e8efec'),
    'ru_apr_release_plenum_20040531': (33588, '9c487c8b6388d4b62d129cbfe454ac7f0feb6c72db74db94470c8857f39b4368'),
    'ru_apr_leadership_page_2004': (24462, 'ddbf238c820898f9ee875b6ae272f81e23d675024171abc9e366356d57cb9aaa'),
    'ru_apr_release_xii_congress_second_stage_20041012': (39474, 'dd8913f30fa1f2b53dda0fb4256ea763af3e250fa458913840342a4124dc11e2'),
    'ru_apr_chairman_page_2008': (9335, 'c85cca5ccc1033bc961c1dba9fbe852d96ec6bfd8b47bf809f1834ac129d9971'),
    'ru_apr_home_page_20081016': (17490, 'e8242f5ce3840ea923cd319572d629afd8a281406ed77bcb79d3c70faed2a551'),
    'ru_vybor_rossii_bloc_programme_19931017': (2040735, '1fdfa9b2135409586ea85eda9eac0320e543a78f72666e1ecdc68776f175b17f'),
    'ru_dvr_supporters_protocol_19940519': (122938, 'be163177e0a890176612f4b1e809492bd0b24f81e513ed1713573cfef00f5cae'),
    'ru_dvr_founding_congress_edition_1994': (10512385, 'df1015115b407715188bc13c223c8aeeace3d66c7bcf0eb7d38e7e7fd082e26f'),
    'ru_dvr_information_bulletin_1_1994': (1198783, '9f6aff6c2c7c21ed7131459227b2a539befe10c97522950548973768175071aa'),
    'ru_dvr_statement_grozny_199412': (436750, '778aaa6adcedccd4846545a8ce129521514a584159d01640c82806325754e75c'),
    'ru_dvr_congress_statement_1995': (22777, '33ef8455c3bbdb1259255135b878201067ed3adc583ce2ea78eb91e0b40a15a3'),
    'ru_dvr_ii_congress_decision_19950618': (23968, '06aef5e3c447767ebe670384358ca960aff33972236223b795b4137b0e4e0ce7'),
    'ru_dvr_v_congress_decision_19960921': (101000, '3ec3fc209f71a789aaa0310af9af70549af2b346ffad2e8276fb2abc5aa274bd'),
    'ru_dvr_v_congress_decisions_19960922': (341045, '79022c35e9990e3b552f4c7310e31a9a03a7e7af87c0aff4a70c1006c1d86a71'),
    'ru_dvr_politsovet_decision_19971216': (4566, '97a5c08d21ad52fe55c5e547263903fef10440da4c09ddec2d39b5c28bdb6de5'),
    'ru_dvr_history_page_1998': (8261, '343d9f2a5495055f915833ee3e28ca908f3c48ef4b8e8535f26314eed78c70ad'),
    'ru_dvr_politsovet_statement_20001216': (123226, 'bb874c6be907c178703b2149c882521f35915daee84d2d96fb76b1c104230d20'),
    'ru_dvr_about_page_2001': (11695, 'e631023bf841216b233277344f5e6bfe92fafaca9ac375fa111385d5404cb059'),
    'ru_dvr_newspaper_demvybor_21_2001': (99602, '1acc8f813aa35b243ce73fb89c131c325cee163ec369ef7e60cb40471e077fb6'),
}
# Raw Internet Archive capture timestamp of every new source (all made before the cutoff).
ARCHIVED = {
    'ru_kprf_i_plenum_notice_19970420': '20000306135653',
    'ru_kprf_v_congress_notice_19980523': '20000306152838',
    'ru_kprf_date_in_history_1993_20260213': '20260727165041',
    'ru_kprf_party_reference_20250912': '20250912215927',
    'ru_kprf_xviii_congress_notice_20210424': '20210426151642',
    'ru_kprf_i_plenum_notice_20210424': '20210426152225',
    'ru_kprf_xix_congress_notice_20250705': '20250708140517',
    'ru_kprf_i_plenum_notice_20250705': '20250708000201',
    'ru_kprf_official_i_plenum_20250705': '20251211130650',
    'ru_kprf_zyuganov_greeting_20260828': '20260829220349',
    'ru_kprf_official_xix_second_stage_20260620': '20260727152918',
    'ru_ldpr_newspaper_plenum_resolution_19961123': '19980207231628',
    'ru_ldpr_party_history_20100922': '20101016073607',
    'ru_ldpr_party_history_2026': '20260216025055',
    'ru_ldpr_newspaper_no04_20220330': '20220801065115',
    'ru_ldpr_news_zhirinovsky_death_20220406': '20251012234518',
    'ru_ldpr_newspaper_no05_20220506': '20220731053447',
    'ru_ldpr_news_supreme_council_recommendation_20220526': '20251012221126',
    'ru_ldpr_news_xxxiv_congress_opens_20220527': '20251012090749',
    'ru_ldpr_news_slutsky_elected_20220527': '20251012171837',
    'ru_ldpr_news_vote_day_20220527': '20251012070034',
    'ru_ldpr_newspaper_no06_07_20220627': '20220731053434',
    'ru_ldpr_news_slutsky_reelected_20251002': '20251012062735',
    'ru_ldpr_news_slutsky_congress_post_20251002': '20251012062743',
    'ru_ldpr_news_citizenship_proposal_20260811': '20260816173730',
    'ru_yabloko_site_reference_1998': '19980630062735',
    'ru_yabloko_vi_congress_report_19980314': '19980630062048',
    'ru_yabloko_x_congress_section_2001': '20020123194510',
    'ru_yabloko_press_party_transformation_20011222': '20020322235901',
    'ru_yabloko_press_yavlinsky_elected_20011223': '20020620151948',
    'ru_yabloko_x_congress_chairman_speech_20011223': '20020323204010',
    'ru_yabloko_press_congress_schedule_20040703': '20050302004743',
    'ru_yabloko_press_yavlinsky_reelected_20040703': '20050301190917',
    'ru_yabloko_press_charter_amendments_20040704': '20040927193602',
    'ru_yabloko_leadership_report_2004_2008': '20111112212637',
    'ru_yabloko_press_new_leadership_20080622': '20111112210033',
    'ru_yabloko_press_xv_congress_decisions_20080626': '20111112203944',
    'ru_yabloko_news_term_limit_20151219': '20151224043458',
    'ru_yabloko_news_slabunova_elected_20151220': '20151222103638',
    'ru_yabloko_news_new_chairman_deputies_20151220': '20151222194032',
    'ru_yabloko_news_rybakov_elected_20191215': '20210627225332',
    'ru_yabloko_news_xxi_congress_main_20191216': '20200513204833',
    'ru_yabloko_news_rybakov_second_term_20231209': '20231209211736',
    'ru_yabloko_news_xxii_congress_main_20231211': '20231212111820',
    'ru_yabloko_news_fsin_appeal_20231213': '20231217211933',
    'ru_yabloko_news_rybakov_appeal_20260819': '20260821172511',
    'ru_apr_history_note_2002': '20021117045251',
    'ru_apr_release_xi_congress_20030910': '20031022201803',
    'ru_apr_release_new_chairman_20040526': '20040710080750',
    'ru_apr_release_plenum_20040531': '20050523171303',
    'ru_apr_leadership_page_2004': '20040520161601',
    'ru_apr_release_xii_congress_second_stage_20041012': '20041124131828',
    'ru_apr_chairman_page_2008': '20080630045912',
    'ru_apr_home_page_20081016': '20081016023015',
    'ru_vybor_rossii_bloc_programme_19931017': '20131005073722',
    'ru_dvr_supporters_protocol_19940519': '20170823160756',
    'ru_dvr_founding_congress_edition_1994': '20160803131429',
    'ru_dvr_information_bulletin_1_1994': '20160813072102',
    'ru_dvr_statement_grozny_199412': '20160803121516',
    'ru_dvr_congress_statement_1995': '20170823162628',
    'ru_dvr_ii_congress_decision_19950618': '20151006140140',
    'ru_dvr_v_congress_decision_19960921': '20161108004556',
    'ru_dvr_v_congress_decisions_19960922': '20250115065600',
    'ru_dvr_politsovet_decision_19971216': '19980627072026',
    'ru_dvr_history_page_1998': '19980627060716',
    'ru_dvr_politsovet_statement_20001216': '20160827164330',
    'ru_dvr_about_page_2001': '20010303063231',
    'ru_dvr_newspaper_demvybor_21_2001': '20010607195936',
}
# Every new claim: (attested_on or None, event kind, review observation, role).
EVENTS = {
    'ru_kprf_i_plenum_zyuganov_elected_chairman_19970420': ('1997-04-20', 'leader_election', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_v_congress_held_19980523': ('1998-05-23', 'congress_dates', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_v_congress_zyuganov_reports_as_chairman_19980523': ('1998-05-23', 'in_office_attestation', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_retro_ii_extraordinary_congress_opened_19930213': ('1993-02-13', 'congress_dates_retrospective', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_retro_congress_renamed_party_1993': (None, 'renaming_retrospective', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_retro_congress_elected_cec_1993': (None, 'party_body_election_retrospective', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_retro_zyuganov_elected_cec_chairman_1993': (None, 'leader_election_retrospective', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_reference_zyuganov_since_february_1993': (None, 'retrospective_list', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_reference_registered_since_ii_congress_1993': (None, 'registration_statement', 'RU-PTY-01', 'ru_kprf_chairman'),
    'ru_kprf_xviii_congress_held_20210424': ('2021-04-24', 'congress_dates', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_xviii_congress_chairman_presents_report_20210424': ('2021-04-24', 'continuation_attestation', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_i_plenum_zyuganov_elected_chairman_20210424': ('2021-04-24', 'leader_election', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_xix_congress_held_20250705': ('2025-07-05', 'congress_dates', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_xix_congress_chairman_presents_party_cards_20250705': ('2025-07-05', 'continuation_attestation', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_i_plenum_zyuganov_elected_chairman_20250705': ('2025-07-05', 'leader_election', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_i_plenum_chairman_closing_word_20250705': ('2025-07-05', 'in_office_attestation', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_official_zyuganov_unanimously_elected_20250705': ('2025-07-05', 'leader_election', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_zyuganov_chairman_greeting_20260828': ('2026-08-28', 'in_office_attestation', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_kprf_xix_congress_second_stage_20260620': ('2026-06-20', 'congress_dates', 'RU-PTY-02', 'ru_kprf_chairman'),
    'ru_ldpr_plenum_resolution_chairman_report_19961123': ('1996-11-23', 'in_office_attestation', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2010_ldpss_founding_congress_19900331': ('1990-03-31', 'congress_dates_retrospective', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2010_ldpss_chairman_elected_19900331': ('1990-03-31', 'leader_election_retrospective', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2010_ussr_justice_registration_19910412': ('1991-04-12', 'registration_statement_retrospective', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2010_iii_congress_founds_ldpr_1992': (None, 'founding_retrospective', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2010_iii_congress_chairman_1992': (None, 'leader_election_retrospective', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2010_iv_congress_chairman_1993': (None, 'leader_election_retrospective', 'RU-PTY-03', 'ru_ldpr_chairman'),
    'ru_ldpr_history2026_zhirinovsky_death_20220406': ('2022-04-06', 'death_retrospective', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_history2026_slutsky_elected_chairman_20220527': ('2022-05-27', 'leader_election_retrospective', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_newspaper_chairman_zhirinovsky_contacts_20220330': ('2022-03-30', 'in_office_attestation', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_zhirinovsky_died_today_20220406': ('2022-04-06', 'death_statement', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_newspaper_interim_issue_no_chairman_20220506': ('2022-05-06', 'interim_period_record', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_xxxiv_congress_announced': (None, 'congress_dates_planned', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_supreme_council_recommends_slutsky_20220526': ('2022-05-26', 'leader_nomination', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_xxxiv_congress_opened_20220527': ('2022-05-27', 'congress_dates', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_xxxiv_congress_elects_slutsky_20220527': ('2022-05-27', 'leader_election_reported', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_chairman_speaks_after_election_20220527': ('2022-05-27', 'in_office_attestation', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_news_leader_vote_held_20220527': ('2022-05-27', 'leader_election', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_newspaper_chairman_speech_xxxiv_congress': (None, 'leader_election_reported', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_newspaper_death_referenced_undated': (None, 'death_reference', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_newspaper_chairman_slutsky_contacts_20220627': ('2022-06-27', 'continuation_attestation', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_xxxvii_congress_slutsky_reelected_20251002': ('2025-10-02', 'leader_election', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_xxxvii_congress_chairman_presents_strategy_20251002': ('2025-10-02', 'in_office_attestation', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_slutsky_says_reelected_today_20251002': ('2025-10-02', 'leader_election_reported', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_slutsky_post_signed_as_chairman_20251002': ('2025-10-02', 'in_office_attestation', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_ldpr_slutsky_chairman_proposal_20260811': ('2026-08-11', 'in_office_attestation', 'RU-PTY-04', 'ru_ldpr_chairman'),
    'ru_yabloko_reference_bloc_lists_autumn_1993': (None, 'retrospective_statement', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_reference_former_names_1993_1994': (None, 'retrospective_name_lineage', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_reference_founding_congress_19950105_19950106': (None, 'congress_dates_retrospective', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_reference_yavlinsky_chairman_elected_1995': (None, 'leader_election_retrospective', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_reference_yavlinsky_chairman_since_january_1995': (None, 'retrospective_list', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_reference_registration_19950210': ('1995-02-10', 'registration_statement', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_yavlinsky_chairman_report_vi_congress_19980314': ('1998-03-14', 'in_office_attestation', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_x_congress_dates_20011222_20011223': (None, 'congress_dates', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_x_congress_association_chairman_report_20011222': ('2001-12-22', 'continuation_attestation', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_x_congress_party_chairman_speech_20011223': ('2001-12-23', 'in_office_attestation', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_association_transformed_into_party_20011222': ('2001-12-22', 'reorganisation_decision', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_yavlinsky_elected_party_chairman_20011223': ('2001-12-23', 'leader_election', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_chairman_speech_page_20011223': ('2001-12-23', 'in_office_attestation', 'RU-PTY-05', 'ru_yabloko_chairman'),
    'ru_yabloko_chairman_vote_scheduled_2004': (None, 'election_schedule', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_yavlinsky_reelected_chairman_2004': (None, 'leader_election_reported', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_yavlinsky_proposes_term_limit_on_his_post_20040704': ('2004-07-04', 'in_office_attestation', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_party_named_successor_of_1995_association': (None, 'retrospective_name_lineage', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_report_2008_chairman_title_block': (None, 'continuation_attestation', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_report_2008_xii_congress_2004': (None, 'congress_dates', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_report_2008_xiii_congress_2006': (None, 'congress_dates', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_report_2008_bureau_list_chairman': (None, 'continuation_attestation', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_mitrokhin_chairman_joins_political_committee_20080622': ('2008-06-22', 'in_office_attestation', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_xv_congress_dates_20080621_20080622': (None, 'congress_dates', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_xv_congress_mitrokhin_elected_2008': (None, 'leader_election', 'RU-PTY-06', 'ru_yabloko_chairman'),
    'ru_yabloko_mitrokhin_holds_chairmanship_20151219': ('2015-12-19', 'in_office_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_retro_yavlinsky_chairman_2001_2008': (None, 'retrospective_term_statement', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_slabunova_elected_xviii_congress_20151220': ('2015-12-20', 'leader_election_reported', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_xviii_congress_dates_20151219_20151220': (None, 'congress_dates', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_mitrokhin_incumbent_nominates_slabunova_20151220': ('2015-12-20', 'continuation_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_slabunova_new_chairman_deputies_20151220': ('2015-12-20', 'in_office_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_elected_xxi_congress_20191215': ('2019-12-15', 'leader_election_reported', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_xxi_congress_dates_20191214_20191215': (None, 'congress_dates', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_slabunova_report_as_chairman_2015_2019': (None, 'retrospective_term_statement', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_xxi_congress_vote_about_one_am': (None, 'leader_election', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_plans_as_chairman_20191216': ('2019-12-16', 'in_office_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_incumbent_candidate_20231209': ('2023-12-09', 'continuation_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_reelected_second_term_20231209': ('2023-12-09', 'leader_election_reported', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_retained_chairmanship_20231211': ('2023-12-11', 'leader_election_reported', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_chairman_appeal_20231213': ('2023-12-13', 'in_office_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_yabloko_rybakov_chairman_appeal_20260819': ('2026-08-19', 'in_office_attestation', 'RU-PTY-07', 'ru_yabloko_chairman'),
    'ru_apr_history_chairman_heading_2002': (None, 'undated_attestation', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_founding_congress_19930226': ('1993-02-26', 'congress_dates_retrospective', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_lapshin_elected_founding_congress_1993': (None, 'leader_election_retrospective', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_charter_registered_19930409': ('1993-04-09', 'registration_statement', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_iii_congress_lapshin_reelected_1994': (None, 'leader_election_retrospective', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_vii_congress_lapshin_reelected_19990320': ('1999-03-20', 'leader_election_retrospective', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_ix_congress_lapshin_elected_20010324': ('2001-03-24', 'leader_election_retrospective', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_x_congress_transformation_20011208': ('2001-12-08', 'reorganisation_decision', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_x_congress_lapshin_elected_20011208': ('2001-12-08', 'leader_election_retrospective', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_v_congress_lapshin_remained_1997': (None, 'retrospective_statement', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_vi_congress_powers_confirmed_19980226': ('1998-02-26', 'retrospective_statement', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_charter_changes_registered_19980529': ('1998-05-29', 'registration_statement', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_history_party_registration_20020531': ('2002-05-31', 'registration_statement', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_xi_congress_opened_20030909': ('2003-09-09', 'congress_dates', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_xi_congress_lapshin_reports_as_chairman_20030909': ('2003-09-09', 'in_office_attestation', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_old_leadership_under_lapshin_20040426': ('2004-04-26', 'retrospective_statement', 'RU-PTY-08', 'ru_apr_chairman'),
    'ru_apr_xii_congress_plotnikov_elected_20040428': ('2004-04-28', 'leader_election', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_plenum_plotnikov_reports_as_chairman_20040528': ('2004-05-28', 'in_office_attestation', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_leadership_page_plotnikov_chairman_2004': (None, 'undated_attestation', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_xii_congress_second_stage_ended_20041009': ('2004-10-09', 'congress_dates', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_plotnikov_comments_as_chairman_20041012': ('2004-10-12', 'continuation_attestation', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_chairman_page_plotnikov_elected_20040428': ('2004-04-28', 'leader_election_retrospective', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_chairman_page_heading_2008': (None, 'undated_attestation', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_memorandum_with_united_russia_20080912': ('2008-09-12', 'merger_memorandum', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_plenum_plotnikov_reports_as_chairman_20080926': ('2008-09-26', 'in_office_attestation', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_xv_congress_held_20081010': ('2008-10-10', 'congress_dates', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_apr_xv_congress_accession_to_united_russia_20081010': ('2008-10-10', 'merger_decision', 'RU-PTY-09', 'ru_apr_chairman'),
    'ru_vybor_rossii_programme_adopted_founding_congress_19931017': ('1993-10-17', 'programme_adopted', 'RU-PTY-10', None),
    'ru_dvr_supporters_meeting_working_name_19940519': ('1994-05-19', 'party_working_name', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_supporters_founding_congress_planned': (None, 'congress_dates_planned', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_founding_declaration_19940612': ('1994-06-12', 'founding_declaration', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_political_council_gaidar_chairman_1994': (None, 'leader_election', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_council_resolution_signed_by_chairman_19940710': ('1994-07-10', 'in_office_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_charter_registered_19940809': ('1994-08-09', 'registration_statement', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_statement_signed_by_chairman_199412': (None, 'undated_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_ii_congress_statement_signed_by_chairman_1995': (None, 'conflicting_date_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_ii_congress_approves_chairman_work_19950618': ('1995-06-18', 'in_office_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_ii_congress_decision_signed_by_chairman_19950618': ('1995-06-18', 'in_office_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_v_congress_list_decision_signed_by_chairman_19960921': ('1996-09-21', 'continuation_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_v_congress_chairman_ballot_protocol_approved_19960922': ('1996-09-22', 'leader_ballot_record', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_v_congress_decision_signed_by_chairman_19960922': ('1996-09-22', 'in_office_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_politsovet_decision_signed_by_chairman_19971216': ('1997-12-16', 'in_office_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_history_created_on_basis_of_movement': (None, 'retrospective_name_lineage', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_history_founded_and_named_1994': (None, 'founding_retrospective', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_history_registered_19940809': ('1994-08-09', 'registration_statement', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_history_programme_adopted_19941119': ('1994-11-19', 'programme_adopted', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_history_ii_congress_19950618': ('1995-06-18', 'congress_dates_retrospective', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_history_iii_congress_19950826': ('1995-08-26', 'congress_dates_retrospective', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_politsovet_statement_signed_chairman_20001216': ('2000-12-16', 'bare_title_signature', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_about_page_chairman_gaidar_2001': (None, 'undated_attestation', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_x_congress_self_dissolution_decision_2001': (None, 'dissolution_decision', 'RU-PTY-10', 'ru_dvr_chairman'),
    'ru_dvr_x_congress_gaidar_speech_dissolution_2001': (None, 'dissolution_speech', 'RU-PTY-10', 'ru_dvr_chairman'),
}
# Exact holder observations per role, in chronological order: (name, attested_on, from, until).
HOLDERS = {
    'ru_kprf_chairman': [
        ('Геннадий Андреевич Зюганов', '1998-05-23', None, None),
        ('Геннадий Андреевич Зюганов', '2025-07-05', None, None),
        ('Геннадий Андреевич Зюганов', '2026-08-28', None, None),
    ],
    'ru_ldpr_chairman': [
        ('Владимир Вольфович Жириновский', '1996-11-23', None, None),
        ('Владимир Вольфович Жириновский', '2022-03-30', None, None),
        ('Леонид Эдуардович Слуцкий', '2022-05-27', None, None),
        ('Леонид Эдуардович Слуцкий', '2025-10-02', None, None),
        ('Леонид Эдуардович Слуцкий', '2026-08-11', None, None),
    ],
    'ru_yabloko_chairman': [
        ('Григорий Алексеевич Явлинский', '1998-03-14', None, None),
        ('Григорий Алексеевич Явлинский', '2001-12-23', None, None),
        ('Григорий Алексеевич Явлинский', '2004-07-04', None, None),
        ('Сергей Сергеевич Митрохин', '2008-06-22', None, None),
        ('Сергей Сергеевич Митрохин', '2015-12-19', None, None),
        ('Эмилия Эдгардовна Слабунова', '2015-12-20', None, None),
        ('Николай Игоревич Рыбаков', '2019-12-16', None, None),
        ('Николай Игоревич Рыбаков', '2023-12-13', None, None),
        ('Николай Игоревич Рыбаков', '2026-08-19', None, None),
    ],
    'ru_apr_chairman': [
        ('Михаил Иванович Лапшин', '2003-09-09', None, None),
        ('Владимир Николаевич Плотников', '2004-05-28', None, None),
        ('Владимир Николаевич Плотников', '2008-09-26', None, None),
    ],
    'ru_dvr_chairman': [
        ('Егор Тимурович Гайдар', '1994-07-10', None, None),
        ('Егор Тимурович Гайдар', '1995-06-18', None, None),
        ('Егор Тимурович Гайдар', '1996-09-22', None, None),
        ('Егор Тимурович Гайдар', '1997-12-16', None, None),
    ],
}
HOLDER_CLAIMS = {
    'ru_kprf_chairman': [['ru_kprf_v_congress_zyuganov_reports_as_chairman_19980523'], ['ru_kprf_i_plenum_chairman_closing_word_20250705'], ['ru_kprf_zyuganov_chairman_greeting_20260828']],
    'ru_ldpr_chairman': [['ru_ldpr_plenum_resolution_chairman_report_19961123'], ['ru_ldpr_newspaper_chairman_zhirinovsky_contacts_20220330'], ['ru_ldpr_news_chairman_speaks_after_election_20220527'], ['ru_ldpr_xxxvii_congress_chairman_presents_strategy_20251002', 'ru_ldpr_slutsky_post_signed_as_chairman_20251002'], ['ru_ldpr_slutsky_chairman_proposal_20260811']],
    'ru_yabloko_chairman': [['ru_yabloko_yavlinsky_chairman_report_vi_congress_19980314'], ['ru_yabloko_chairman_speech_page_20011223', 'ru_yabloko_x_congress_party_chairman_speech_20011223'], ['ru_yabloko_yavlinsky_proposes_term_limit_on_his_post_20040704'], ['ru_yabloko_mitrokhin_chairman_joins_political_committee_20080622'], ['ru_yabloko_mitrokhin_holds_chairmanship_20151219'], ['ru_yabloko_slabunova_new_chairman_deputies_20151220'], ['ru_yabloko_rybakov_plans_as_chairman_20191216'], ['ru_yabloko_rybakov_chairman_appeal_20231213'], ['ru_yabloko_rybakov_chairman_appeal_20260819']],
    'ru_apr_chairman': [['ru_apr_xi_congress_lapshin_reports_as_chairman_20030909'], ['ru_apr_plenum_plotnikov_reports_as_chairman_20040528'], ['ru_apr_plenum_plotnikov_reports_as_chairman_20080926']],
    'ru_dvr_chairman': [['ru_dvr_council_resolution_signed_by_chairman_19940710'], ['ru_dvr_ii_congress_approves_chairman_work_19950618', 'ru_dvr_ii_congress_decision_signed_by_chairman_19950618'], ['ru_dvr_v_congress_decision_signed_by_chairman_19960922'], ['ru_dvr_politsovet_decision_signed_by_chairman_19971216']],
}
ROLES = {
    'ru_kprf_chairman': ('ru_duma_ballot_list_2021_01', 'Председатель Центрального Комитета КПРФ — Chairman of the Central Committee of the KPRF'),
    'ru_ldpr_chairman': ('ru_duma_ballot_list_2021_03', 'Председатель ЛДПР — Chairman of the LDPR'),
    'ru_yabloko_chairman': ('ru_duma_ballot_list_2021_07', 'Председатель партии «ЯБЛОКО» — Chairman of Yabloko'),
    'ru_apr_chairman': ('ru_apr_party_self_record', 'Председатель Аграрной партии России — Chairman of the Agrarian Party of Russia'),
    'ru_dvr_chairman': ('ru_dvr_party_self_record', 'Председатель партии «Демократический выбор России» — Chairman of Democratic Choice of Russia'),
}
SOURCE_TOTAL, CLAIM_TOTAL = 68, 137
# Every dated claim day that is not a holder day (generated from the claims above).
CLAIM_DATES_OFF_HOLDERS = {
    '1990-03-31', '1991-04-12', '1993-02-13', '1993-02-26', '1993-04-09', '1993-10-17', '1994-05-19', '1994-06-12',
    '1994-08-09', '1994-11-19', '1995-02-10', '1995-08-26', '1996-09-21', '1997-04-20', '1998-02-26', '1998-05-29',
    '1999-03-20', '2000-12-16', '2001-03-24', '2001-12-08', '2001-12-22', '2002-05-31', '2004-04-26', '2004-04-28',
    '2004-10-09', '2004-10-12', '2008-09-12', '2008-10-10', '2019-12-15', '2021-04-24', '2022-04-06', '2022-05-06',
    '2022-05-26', '2022-06-27', '2023-12-09', '2023-12-11', '2026-06-20',
}
# Rows whose source names nobody: they carry no holder_name and no printed form.
NAMELESS = {
    'ru_kprf_v_congress_held_19980523',
    'ru_kprf_retro_ii_extraordinary_congress_opened_19930213',
    'ru_kprf_retro_congress_renamed_party_1993',
    'ru_kprf_retro_congress_elected_cec_1993',
    'ru_kprf_reference_registered_since_ii_congress_1993',
    'ru_kprf_xviii_congress_held_20210424',
    'ru_kprf_xix_congress_held_20250705',
    'ru_kprf_xix_congress_second_stage_20260620',
    'ru_ldpr_history2010_ldpss_founding_congress_19900331',
    'ru_ldpr_history2010_ussr_justice_registration_19910412',
    'ru_ldpr_history2010_iii_congress_founds_ldpr_1992',
    'ru_ldpr_newspaper_interim_issue_no_chairman_20220506',
    'ru_ldpr_news_xxxiv_congress_announced',
    'ru_ldpr_news_xxxiv_congress_opened_20220527',
    'ru_yabloko_reference_bloc_lists_autumn_1993',
    'ru_yabloko_reference_former_names_1993_1994',
    'ru_yabloko_reference_founding_congress_19950105_19950106',
    'ru_yabloko_reference_registration_19950210',
    'ru_yabloko_x_congress_dates_20011222_20011223',
    'ru_yabloko_association_transformed_into_party_20011222',
    'ru_yabloko_chairman_vote_scheduled_2004',
    'ru_yabloko_party_named_successor_of_1995_association',
    'ru_yabloko_report_2008_xii_congress_2004',
    'ru_yabloko_report_2008_xiii_congress_2006',
    'ru_yabloko_xv_congress_dates_20080621_20080622',
    'ru_yabloko_xviii_congress_dates_20151219_20151220',
    'ru_yabloko_xxi_congress_dates_20191214_20191215',
    'ru_apr_history_founding_congress_19930226',
    'ru_apr_history_charter_registered_19930409',
    'ru_apr_history_x_congress_transformation_20011208',
    'ru_apr_history_charter_changes_registered_19980529',
    'ru_apr_history_party_registration_20020531',
    'ru_apr_xi_congress_opened_20030909',
    'ru_apr_xii_congress_second_stage_ended_20041009',
    'ru_apr_memorandum_with_united_russia_20080912',
    'ru_apr_xv_congress_held_20081010',
    'ru_apr_xv_congress_accession_to_united_russia_20081010',
    'ru_vybor_rossii_programme_adopted_founding_congress_19931017',
    'ru_dvr_supporters_meeting_working_name_19940519',
    'ru_dvr_supporters_founding_congress_planned',
    'ru_dvr_founding_declaration_19940612',
    'ru_dvr_charter_registered_19940809',
    'ru_dvr_v_congress_chairman_ballot_protocol_approved_19960922',
    'ru_dvr_history_created_on_basis_of_movement',
    'ru_dvr_history_founded_and_named_1994',
    'ru_dvr_history_registered_19940809',
    'ru_dvr_history_programme_adopted_19941119',
    'ru_dvr_history_ii_congress_19950618',
    'ru_dvr_history_iii_congress_19950826',
    'ru_dvr_x_congress_self_dissolution_decision_2001',
}
NEW_SOURCES = list(RESPONSES)
PREFIXES = ('ru_kprf_', 'ru_ldpr_', 'ru_yabloko_', 'ru_apr_', 'ru_dvr_', 'ru_vybor_rossii_')
ROLE_PREFIX = {'ru_kprf_chairman': 'ru_kprf_', 'ru_ldpr_chairman': 'ru_ldpr_', 'ru_yabloko_chairman': 'ru_yabloko_',
               'ru_apr_chairman': 'ru_apr_', 'ru_dvr_chairman': 'ru_dvr_'}
BALLOT = {'ru_duma_ballot_list_2021_01': 'ru_kprf_chairman', 'ru_duma_ballot_list_2021_03': 'ru_ldpr_chairman',
          'ru_duma_ballot_list_2021_07': 'ru_yabloko_chairman'}
NEW_ORGS = [('ru_apr_party_self_record', 'Аграрная партия России', 'political_party_self_record_observation', ['ru_apr_chairman']),
            ('ru_vybor_rossii_bloc_1993', 'Избирательное объединение «Выбор России»',
             'electoral_association_self_record_observation', []),
            ('ru_dvr_party_self_record', 'Демократический выбор России', 'political_party_self_record_observation',
             ['ru_dvr_chairman'])]
BLOC_CLAIMS = ['ru_vybor_rossii_programme_adopted_founding_congress_19931017']
# The one event kind that may date a holder; every other kind never does.
HOLDER_KINDS = {'in_office_attestation'}
ELECTION_KINDS = {'leader_election', 'leader_election_reported', 'leader_election_retrospective', 'leader_ballot_record'}
ORGANIZATION_KINDS = {'congress_dates', 'congress_dates_retrospective', 'congress_dates_planned', 'registration_statement',
                      'registration_statement_retrospective', 'reorganisation_decision', 'renaming_retrospective',
                      'retrospective_name_lineage', 'party_working_name', 'founding_declaration', 'founding_retrospective',
                      'programme_adopted', 'merger_memorandum', 'merger_decision', 'dissolution_decision',
                      'dissolution_speech'}
CONTEXT_KINDS = {'continuation_attestation', 'undated_attestation', 'retrospective_list', 'retrospective_statement',
                 'retrospective_term_statement', 'death_statement', 'death_retrospective', 'death_reference',
                 'interim_period_record', 'leader_nomination', 'election_schedule', 'party_body_election_retrospective',
                 'bare_title_signature', 'conflicting_date_attestation'}
NEVER_HOLDER = tuple(c for c, v in EVENTS.items() if v[1] not in HOLDER_KINDS)
UNDATED = tuple(c for c, v in EVENTS.items() if v[0] is None)
PEOPLE = {'Геннадий Андреевич Зюганов', 'Владимир Вольфович Жириновский', 'Леонид Эдуардович Слуцкий',
          'Григорий Алексеевич Явлинский', 'Сергей Сергеевич Митрохин', 'Эмилия Эдгардовна Слабунова',
          'Николай Игоревич Рыбаков', 'Михаил Иванович Лапшин', 'Владимир Николаевич Плотников', 'Егор Тимурович Гайдар'}
# Dates that are never any holder's attested_on, start or end: every dated claim's day that is not itself a holder's
# attested day (elections, congresses, deaths, registrations, reorganisation, accession and dissolution decisions,
# retrospective, continuation and interim records, nominations), plus days a reader could derive from spans and relative
# words (the other days of multi-day congresses, a statement's disagreeing adoption day, notices' publication days).
DERIVED_NEVER_HOLDER_DATE = {
    '1992-04-18', '1992-04-19', '1993-02-14', '1993-04-24', '1993-04-25', '1994-06-13', '1994-10-26', '1994-10-27',
    '1995-01-05', '1995-01-06', '1995-06-17', '1997-03-22', '1997-03-23', '2001-05-19', '2004-04-27', '2008-06-21',
    '2015-12-18', '2019-12-14', '2019-12-15', '2021-04-26', '2023-12-10', '2025-07-07',
    '2025-07-08', '2026-06-23', '2026-09-07'}
NEVER_HOLDER_DATE = DERIVED_NEVER_HOLDER_DATE | CLAIM_DATES_OFF_HOLDERS
# Claims that name a day and are tempting as an end; none states that a party office ended.
TEMPTING_ENDS = ('ru_ldpr_history2026_zhirinovsky_death_20220406', 'ru_ldpr_news_zhirinovsky_died_today_20220406',
                 'ru_ldpr_newspaper_interim_issue_no_chairman_20220506', 'ru_apr_old_leadership_under_lapshin_20040426',
                 'ru_apr_xii_congress_plotnikov_elected_20040428', 'ru_apr_xv_congress_accession_to_united_russia_20081010',
                 'ru_yabloko_rybakov_incumbent_candidate_20231209')
# Leads not imported (news agencies, encyclopaedias, live and search pages, a charter, reposted news); never a source.
LEAD_URL_MARKERS = ('wikipedia.org', 'tass.ru', 'ria.ru', 'kommersant.ru', 'interfax.ru', 'rbc.ru', 'lenta.ru', 'vedomosti.ru',
                    'mosyabloko.ru', 'Press/2008/080622.html', 'upload/grain.tables', 'members/moscow', 'live.search',
                    'personae/gaidar', 'inf_soob.html', 'releases/1/168', 'cheef/index.php', 'official/2021/04/24/i-organ',
                    'htm/prezid.htm', 'databasedocuments', 'youtube.com', 'event/214130',
                    'file/3797', 'file/3799', 'file/3736', 'releases/3/212')
# URL shapes of pages generated per request, growing searches or unresolved captures; never allowed.
VOLATILE_URL = re.compile(r'(ysclid=|sessid=|PHPSESSID|[?&]cb=|nocache|token=|utm_|fbclid|yclid|cdx/search|/search\b|'
                          r'ajax\.php|/web/\d{4}id_/|/web/\d{14}/)')
REPORT = research.RESEARCH / 'russia-party-leaders-1990-2026-28.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-28.md'
FACTION_HEADS = {'ru_duma_faction_20211012_er_head': 'Владимир Васильев', 'ru_duma_faction_20211012_kprf_head': 'Геннадий Зюганов',
                 'ru_duma_faction_20211012_srzp_head': 'Сергей Миронов', 'ru_duma_faction_20211012_ldpr_head': 'Владимир Жириновский',
                 'ru_duma_faction_20211012_nl_head': 'Алексей Нечаев'}


def all_roles(packet):
    return {r['id']: (e, r) for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles']}


def party_invariants(russia, ussr):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in russia['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in russia['sources'] for c in s['claims']}
    orgs = russia['organizations']
    # Fourteen ballot-list observations unchanged in order, then exactly the three new observations.
    assert [o['id'] for o in orgs[:14]] == [f'ru_duma_ballot_list_2021_{n:02d}' for n in range(1, 15)]
    assert [(o['id'], o['name'], o['kind'], [r['id'] for r in o['roles']]) for o in orgs[14:]] == NEW_ORGS
    for o in orgs[:14]:
        assert o['kind'] == 'federal_election_ballot_party_list' and len(o['claim_ids']) == 1, o['id']
        assert o['sources'] == ['ru_cec_ballot_order_20210816'], o['id']
        assert [r['id'] for r in o['roles']] == ([BALLOT[o['id']]] if o['id'] in BALLOT else []), o['id']
    for o in russia['institutions']:
        assert all(r['id'] not in ROLES for r in o['roles']), o['id']
    for o in orgs + russia['institutions']:
        assert o['represented_party_ids'] == [] and o.get('reconciled_organization_id') is None, o['id']
        assert o['lifecycle']['from'] is None and o['lifecycle']['until'] is None, o['id']
    roles = all_roles(russia)
    party = {rid: roles[rid][1] for rid in ROLES}
    kinds = sorted(r['id'] for e, r in roles.values() if r['kind'] == 'party_leader')
    assert kinds == sorted(ROLES), kinds
    ours = set(EVENTS)
    people = set()
    for rid, (obs, title) in ROLES.items():
        entry, role = roles[rid]
        assert (entry['id'], role['title'], role['kind']) == (obs, title, 'party_leader'), rid
        expected_claims = [c for c, v in EVENTS.items() if v[3] == rid]
        assert role['claim_ids'] == expected_claims, rid
        expected_sources = []
        for cid in expected_claims:
            if owner[cid] not in expected_sources:
                expected_sources.append(owner[cid])
        assert role['sources'] == expected_sources, rid
        assert all(c.startswith(ROLE_PREFIX[rid]) for c in role['claim_ids']), rid
        holders = role['holder_claims']
        assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS[rid], rid
        assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS[rid], rid
        previous = None
        for h in holders:
            people.add(h['name'])
            assert h['from'] is None and h['until'] is None, h['name']
            assert h['attested_on'] is not None and h['attested_on'] not in NEVER_HOLDER_DATE, h['name']
            assert previous is None or h['attested_on'] > previous, h['name']
            previous = h['attested_on']
            assert not re.search(r'(?i)исполняющ|и\. ?о\.|acting|interim|совет', h['name']), h['name']
            srcs = []
            for cid in h['claim_ids']:
                assert cid in role['claim_ids'] and cid not in NEVER_HOLDER, cid
                assert EVENTS[cid][1] in HOLDER_KINDS and claims[cid]['attested_on'] == h['attested_on'], cid
                if owner[cid] not in srcs:
                    srcs.append(owner[cid])
            assert h['sources'] == srcs, h['name']
    assert people == PEOPLE and len(people) <= 10
    # No other role, entry or institution in either packet cites a claim or source of this packet.
    new_sources = set(RESPONSES)
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                for r in entry['roles']:
                    cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    if r['id'] in ROLES:
                        assert cited <= ours and set(r['sources']) <= new_sources, r['id']
                    else:
                        assert not cited & ours and not set(r['sources']) & new_sources, r['id']
                if entry['id'] in {'ru_apr_party_self_record', 'ru_dvr_party_self_record', 'ru_vybor_rossii_bloc_1993'}:
                    assert set(entry['claim_ids']) <= ours, entry['id']
                else:
                    assert not set(entry['claim_ids']) & ours and not set(entry['sources']) & new_sources, entry['id']
    # The 1993 bloc keeps its own claim and no role; the two new party observations cite their organization claims.
    bloc = next(o for o in orgs if o['id'] == 'ru_vybor_rossii_bloc_1993')
    assert bloc['claim_ids'] == BLOC_CLAIMS and bloc['roles'] == []
    assert all(EVENTS[c][3] is None for c in BLOC_CLAIMS)
    for oid, rid in (('ru_apr_party_self_record', 'ru_apr_chairman'), ('ru_dvr_party_self_record', 'ru_dvr_chairman')):
        entry = next(o for o in orgs if o['id'] == oid)
        assert set(entry['claim_ids']) <= set(party[rid]['claim_ids']), oid
        assert all(EVENTS[c][1] in ORGANIZATION_KINDS for c in entry['claim_ids']), oid
    # The 2021 faction heads are unchanged and never cite party records.
    for rid, name in FACTION_HEADS.items():
        role = roles[rid][1]
        assert role['kind'] == 'parliamentary_leader' and [h['name'] for h in role['holder_claims']] == [name], rid
        assert [h['attested_on'] for h in role['holder_claims']] == ['2021-10-12'], rid
    # The presidency, the Government and the USSR roles keep their holders and share nothing with the party roles.
    for rid in ('ru_rsfsr_president', 'ru_rsfsr_vice_president', 'ru_president', 'ru_government_chairman'):
        role = roles[rid][1]
        assert not (set(role['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}) & ours, rid
    assert [h['name'] for h in roles['ru_government_chairman'][1]['holder_claims']][-1] == 'Михаил Владимирович Мишустин'
    assert len(roles['ru_president'][1]['holder_claims']) == 7
    # Distinct dated events stay distinct: an election is never the attestation that dates its holder.
    for election, attestation in (
            ('ru_kprf_i_plenum_zyuganov_elected_chairman_20250705', 'ru_kprf_i_plenum_chairman_closing_word_20250705'),
            ('ru_ldpr_xxxvii_congress_slutsky_reelected_20251002', 'ru_ldpr_xxxvii_congress_chairman_presents_strategy_20251002'),
            ('ru_yabloko_yavlinsky_elected_party_chairman_20011223', 'ru_yabloko_x_congress_party_chairman_speech_20011223'),
            ('ru_apr_xii_congress_plotnikov_elected_20040428', 'ru_apr_plenum_plotnikov_reports_as_chairman_20040528')):
        assert EVENTS[election][1] in ELECTION_KINDS and EVENTS[attestation][1] in HOLDER_KINDS
        assert claims[election]['attested_on'] <= claims[attestation]['attested_on'], election
    for cid in TEMPTING_ENDS:
        assert claims[cid]['attested_on'] in NEVER_HOLDER_DATE, cid
    for cid in UNDATED:
        assert 'attested_on' not in claims[cid], cid


class RussianPartyLeadersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.ussr = json.loads((research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8'))
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.roles = {rid: r for rid, (e, r) in all_roles(cls.packet).items()}
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Russia'}, {'Russia': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        new_claims = [c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']]
        self.assertEqual((len(NEW_SOURCES), len(new_claims)), (SOURCE_TOTAL, CLAIM_TOTAL))
        self.assertEqual(new_claims, list(EVENTS))
        self.assertEqual([s['id'] for s in self.packet['sources'][EARLIER_SOURCE_COUNT:]], NEW_SOURCES)
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])),
                         (EARLIER_SOURCE_COUNT + SOURCE_TOTAL, 295 + CLAIM_TOTAL, 24, 14))
        vocabulary = HOLDER_KINDS | ELECTION_KINDS | ORGANIZATION_KINDS | CONTEXT_KINDS
        self.assertEqual({v[1] for v in EVENTS.values()}, vocabulary)
        self.assertEqual(len(vocabulary), sum(map(len, (HOLDER_KINDS, ELECTION_KINDS, ORGANIZATION_KINDS, CONTEXT_KINDS))))
        holder_claims = {cid for role in HOLDER_CLAIMS.values() for ids_ in role for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertTrue(holder_claims <= {c for c, v in EVENTS.items() if v[1] in HOLDER_KINDS})
        # Every new source and claim belongs to one party prefix; every claim is cited by its role or the bloc.
        for sid in NEW_SOURCES:
            self.assertTrue(sid.startswith(PREFIXES), sid)
        for cid, (day, kind, obs, role) in EVENTS.items():
            self.assertTrue(cid.startswith(PREFIXES), cid)
            if role is None:
                self.assertIn(cid, BLOC_CLAIMS)
            else:
                self.assertIn(cid, self.roles[role]['claim_ids'], cid)
                self.assertTrue(cid.startswith(ROLE_PREFIX[role]), cid)
            if day is not None:
                self.assertRegex(cid, day.replace('-', '') + '$', cid)
        observations = re.findall(r'^### (RU-PTY-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'RU-PTY-{n:02d}' for n in range(1, 11)])
        self.assertEqual({v[2] for v in EVENTS.values()}, set(observations))

    def test_holders_are_exactly_as_intended(self):
        party_invariants(self.packet, self.ussr)
        for rid in ROLES:
            role = self.roles[rid]
            for holder in role['holder_claims']:
                self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
                for cid in holder['claim_ids']:
                    row = self.rows[cid]
                    self.assertEqual((row['holder_name'], row['role_id'], row['event_kind']),
                                     (holder['name'], rid, 'in_office_attestation'), cid)
                    self.assertEqual(row['role_title'], ROLES[rid][1], cid)
        # One canonical name per person; the printed form stays beside it; rows naming nobody carry no holder.
        for cid, row in self.rows.items():
            self.assertNotIn('name', row)
            self.assertIn(row['holder_name'], PEOPLE | {None}, cid)
            self.assertEqual('printed_name' in row, row['holder_name'] is not None, cid)
        self.assertEqual({cid for cid, row in self.rows.items() if row['holder_name'] is None}, NAMELESS)
        self.assertTrue(all(EVENTS[c][1] not in HOLDER_KINDS for c in NAMELESS))
        # Two earlier titles are kept on their own retrospective rows only.
        self.assertEqual({cid: row['role_title'] for cid, row in self.rows.items()
                          if row['role_id'] and row['role_title'] != ROLES[row['role_id']][1]},
                         {'ru_kprf_retro_zyuganov_elected_cec_chairman_1993':
                              'Председатель ЦИК КПРФ — Chairman of the Central Executive Committee of the KPRF (as printed)',
                          'ru_ldpr_history2010_ldpss_chairman_elected_19900331':
                              'Председатель ЛДПСС — Chairman of the LDPSS (as printed)'})

    def test_no_start_or_end_is_stated_or_inferred(self):
        for rid in ROLES:
            self.assertEqual({(h['from'], h['until']) for h in self.roles[rid]['holder_claims']}, {(None, None)}, rid)
            scope = self.roles[rid]['scope_note']
            for phrase in ('Acting and interim service are claims only, never holders', 'procedure only, never a date',
                           'No holder has a from or an until', 'a name match is never a game mapping'
                           if rid in ('ru_apr_chairman', 'ru_dvr_chairman') else 'does not reconcile'):
                self.assertIn(phrase, scope, rid)
        for cid, (day, kind, obs, role) in EVENTS.items():
            uncertainty = self.claims[cid]['uncertainty']
            if kind in ELECTION_KINDS:
                self.assertRegex(uncertainty, r"(?i)never (a holder date or start|the holder's `from`)", cid)
            if kind in HOLDER_KINDS:
                self.assertRegex(uncertainty, r'(?i)attestation of \S+ |naming \S+ chairman', cid)
                self.assertRegex(uncertainty, r'(?i)not a start', cid)
            if day is None:
                self.assertNotIn('attested_on', self.claims[cid], cid)
                self.assertIsNone(self.rows[cid]['attested_on'], cid)
                self.assertTrue(self.rows[cid]['printed_date'], cid)
            else:
                self.assertNotIn('printed_date', self.rows[cid], cid)
        for cid in TEMPTING_ENDS:
            self.assertRegex(self.claims[cid]['uncertainty'], r"(?i)(not an end|not the end|no `?until|never a holder|not "
                                                             r"the holder's `until`|never .*boundary|not a statement of the day|never \S+ end)", cid)
        death = self.claims['ru_ldpr_history2026_zhirinovsky_death_20220406']
        self.assertIn('an integrator ruling is requested', death['uncertainty'])
        self.assertIn('Retrospective lists are claims', self.section('Integration notes') + self.section('Outcome'))

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual((extract['scope_note'], extract['access_method'], extract['published_date'],
                              extract['document_date']), (source['scope_note'], source['access_method'],
                                                          source['published_date'], source['document_date']))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-28', '2026-09-28'))
            self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid], sid)
            self.assertEqual(extract['source_response_url'], source['url'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('recorded identity is the uncompressed body', extract['provenance_note'])
            self.assertIn('served without Content-Encoding', extract['source_response_encoding'])
            self.assertEqual('equals the capture index digest' in extract['provenance_note'],
                             extract['source_response_sha1_base32'] == extract['archive_index_digest'], sid)
            self.assertRegex(extract['stability_check'],
                             r'This packet: re-downloaded at 2026-09-28T\d\d:\d\d:\d\dZ and again at 2026-09-28T\d\d:\d\d:\d\dZ '
                             r'\((3\d|[4-9]\d|\d{3}) minutes later\)')
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('for CLAUDE-C01-28 only', extract['bounded_scope'])
            url = urlsplit(source['url'])
            stamp = ARCHIVED[sid]
            self.assertEqual((url.scheme, url.hostname), ('https', 'web.archive.org'))
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
            self.assertEqual((extract['original_url'], extract['archive_capture_utc'].replace('-', '').replace(':', '')
                              .replace('T', '').rstrip('Z')), (source['original_url'], stamp))
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/russia-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim.get('attested_on')), claim['id'])
                self.assertNotIn('name', row)
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['role_id'])
                          for cid, row in self.rows.items()}, EVENTS)
        # Two 1998 dvr.ru captures were stored chunked: their body SHA-1 differs from the index digest, as disclosed.
        self.assertEqual({sid for sid in NEW_SOURCES if self.extracts[sid]['source_response_sha1_base32'] !=
                          self.extracts[sid]['archive_index_digest']},
                         {'ru_dvr_politsovet_decision_19971216', 'ru_dvr_history_page_1998'})

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        urls = [u for sid in NEW_SOURCES for u in (self.sources[sid]['url'], self.sources[sid]['original_url'],
                                                    self.extracts[sid]['source_response_url'])]
        for url in urls:
            self.assertIsNone(VOLATILE_URL.search(url), url)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url)
        lowered = json.dumps([self.sources[sid] for sid in NEW_SOURCES], ensure_ascii=False).lower()
        for marker in LEAD_URL_MARKERS:
            self.assertNotIn(marker.lower(), lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('Press/2008/080622.html', 'upload/grain.tables', 'members/moscow', 'personae/gaidar', 'releases/1/168',
                       'event/214130', 'Wikipedia'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_party_and_state_offices_stay_separate(self):
        party_invariants(self.packet, self.ussr)
        for sid in NEW_SOURCES:
            self.assertNotIn(f'"{sid}"', json.dumps(self.ussr, ensure_ascii=False))
        # Faction and state offices printed beside a party office are never claims of a party role.
        state = re.compile(r'(?i)(ru_duma_faction|ru_government|ru_president|ru_rsfsr|ru_cec_|ru_pub_|ru_ukaz)')
        for rid in ROLES:
            for cid in self.roles[rid]['claim_ids']:
                self.assertIsNone(state.search(cid), cid)
        for cid in ('ru_ldpr_newspaper_chairman_zhirinovsky_contacts_20220330', 'ru_ldpr_newspaper_chairman_slutsky_contacts_20220627'):
            self.assertIn('faction', self.claims[cid]['uncertainty'], cid)
        self.assertIn('not a claim here', self.claims['ru_dvr_political_council_gaidar_chairman_1994']['uncertainty'])
        for org_id in BALLOT:
            entry = next(o for o in self.packet['organizations'] if o['id'] == org_id)
            self.assertIn('CLAUDE-C01-28', entry['coverage']['unresolved'][-1])
            self.assertIn('does not establish', entry['coverage']['unresolved'][-1])

    def test_new_organization_observations_are_unmapped_primary_records(self):
        for org_id, name, kind, roles in NEW_ORGS:
            entry = next(o for o in self.packet['organizations'] if o['id'] == org_id)
            self.assertEqual((entry['represented_party_ids'], entry['reconciled_organization_id']), ([], None))
            self.assertEqual((entry['lifecycle']['status'], entry['lifecycle']['from'], entry['lifecycle']['until']),
                             ('unknown', None, None))
            self.assertEqual(entry['coverage']['status'], 'partial')
            self.assertIn('name match is never a game mapping', entry['coverage']['unresolved'][-1])
            for sid in entry['sources']:
                self.assertTrue(self.sources[sid]['source_type'].startswith('primary_'), sid)
        self.assertEqual(sum('CLAUDE-C01-28' in u for u in self.packet['coverage']['unresolved']), 1)
        note = self.packet['coverage']['unresolved'][-1]
        self.assertTrue(note.startswith('CLAUDE-C01-28 adds five party-leader roles'))
        self.assertIn(f"{sum(len(v) for v in HOLDERS.values())} holder observations of ten people", note)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'russia.json').read_bytes()
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

        def role(packet, rid):
            return all_roles(packet)[rid][1]

        def holder(packet, rid, index):
            return role(packet, rid)['holder_claims'][index]

        def org(packet, oid):
            return next(o for o in packet['organizations'] if o['id'] == oid)

        validator_cases = [
            (lambda p: source(p, 'ru_kprf_zyuganov_greeting_20260828')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'ru_ldpr_newspaper_no04_20220330')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 'ru_yabloko_chairman', 8).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'ru_ldpr_slutsky_chairman_proposal_20260811').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 'ru_apr_chairman', 1).update({'from': '2004-05-28', 'until': '2004-04-28'}), 'Reversed historical interval'),
            (lambda p: holder(p, 'ru_kprf_chairman', 0)['claim_ids'].append('ru_kprf_zyuganov_chairman_greeting_20260828'), 'cited source'),
            (lambda p: source(p, 'ru_dvr_politsovet_decision_19971216').update(url='http://www.dvr.ru/politsovet_16-12-97.htm'), 'Invalid public source URL'),
            (lambda p: org(p, 'ru_apr_party_self_record').update(represented_party_ids=['Russia/ru_apr']), 'represented party mapping'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def add_holder(rid, name, day, sid, cid, index=0):
            return lambda p: role(p, rid)['holder_claims'].insert(index, {'name': name, 'attested_on': day, 'from': None,
                                                                          'until': None, 'sources': [sid], 'claim_ids': [cid]})

        invariant_cases = [
            ('successor election used as an end (Zhirinovsky)', lambda p: holder(p, 'ru_ldpr_chairman', 1).update(until='2022-05-27')),
            ('death used as an end (Zhirinovsky)', lambda p: holder(p, 'ru_ldpr_chairman', 1).update(until='2022-04-06')),
            ('successor election used as an end (Yavlinsky)', lambda p: holder(p, 'ru_yabloko_chairman', 2).update(until='2008-06-22')),
            ('successor attestation used as an end (Mitrokhin)', lambda p: holder(p, 'ru_yabloko_chairman', 4).update(until='2015-12-20')),
            ('contemporaneous death statement used as an end (Zhirinovsky)', lambda p: holder(p, 'ru_ldpr_chairman', 1).update(until='2022-04-06')),
            ('nomination used as from (Slutsky 2022)', lambda p: holder(p, 'ru_ldpr_chairman', 2).update({'from': '2022-05-26'})),
            ('successor election used as an end (Lapshin)', lambda p: holder(p, 'ru_apr_chairman', 0).update(until='2004-04-28')),
            ('accession decision used as an end (Plotnikov)', lambda p: holder(p, 'ru_apr_chairman', 2).update(until='2008-10-10')),
            ('latest attestation used as an end (Zyuganov)', lambda p: holder(p, 'ru_kprf_chairman', 2).update(until='2026-08-28')),
            ('election date used as from (Zyuganov 2025)', lambda p: holder(p, 'ru_kprf_chairman', 1).update({'from': '2025-07-05', 'attested_on': None})),
            ('election date used as from (Slutsky 2025)', lambda p: holder(p, 'ru_ldpr_chairman', 3).update({'from': '2025-10-02'})),
            ('election date used as from (Yavlinsky 2001)', lambda p: holder(p, 'ru_yabloko_chairman', 1).update({'from': '2001-12-23'})),
            ('election date used as the attested day (Plotnikov)', lambda p: holder(p, 'ru_apr_chairman', 1).update(attested_on='2004-04-28')),
            ('congress day used as the attested day (Mitrokhin)', lambda p: holder(p, 'ru_yabloko_chairman', 3).update(attested_on='2008-06-21')),
            ('election report used as the attested day (Rybakov 2023)', lambda p: holder(p, 'ru_yabloko_chairman', 7).update(attested_on='2023-12-11')),
            ('interim body added as a holder (LDPR 2022)', add_holder('ru_ldpr_chairman', 'Высший Совет ЛДПР', '2022-05-06',
                                                                      'ru_ldpr_newspaper_no05_20220506',
                                                                      'ru_ldpr_newspaper_interim_issue_no_chairman_20220506', 2)),
            ('retrospective election added as a holder (Zhirinovsky 1990)', add_holder('ru_ldpr_chairman', 'Владимир Вольфович Жириновский',
                                                                                    '1990-03-31', 'ru_ldpr_party_history_20100922',
                                                                                    'ru_ldpr_history2010_ldpss_founding_congress_chairman_19900331')),
            ('election cited by a holder (KPRF 1997)', lambda p: (holder(p, 'ru_kprf_chairman', 0)['claim_ids'].append('ru_kprf_i_plenum_zyuganov_elected_chairman_19970420'),
                                                                    holder(p, 'ru_kprf_chairman', 0)['sources'].append('ru_kprf_i_plenum_notice_19970420'))),
            ('undated attestation cited by a holder (Gaidar)', lambda p: (holder(p, 'ru_dvr_chairman', 0)['claim_ids'].append('ru_dvr_statement_signed_by_chairman_199412'),
                                                                           holder(p, 'ru_dvr_chairman', 0)['sources'].append('ru_dvr_statement_grozny_199412'))),
            ('continuation claim cited by a holder (Rybakov)', lambda p: holder(p, 'ru_yabloko_chairman', 7)['claim_ids'].append('ru_yabloko_rybakov_incumbent_candidate_20231209')),
            ('nomination cited by a holder (Slutsky)', lambda p: holder(p, 'ru_ldpr_chairman', 2)['claim_ids'].append('ru_ldpr_news_supreme_council_recommends_slutsky_20220526')),
            ('faction head added to a party role (Zyuganov 2021)', lambda p: role(p, 'ru_kprf_chairman')['holder_claims'].insert(
                2, copy.deepcopy(holder(p, 'ru_duma_faction_20211012_kprf_head', 0)))),
            ('faction claim cited by a party role', lambda p: role(p, 'ru_ldpr_chairman')['claim_ids'].append(
                role(p, 'ru_duma_faction_20211012_ldpr_head')['claim_ids'][0])),
            ('state-office claim feeding a party role', lambda p: role(p, 'ru_kprf_chairman')['claim_ids'].append(
                'ru_pub_gov_res_1125_mishustin_signs_as_chairman_20260903')),
            ('party claim cited by the Government role', lambda p: role(p, 'ru_government_chairman')['claim_ids'].append(
                'ru_kprf_zyuganov_chairman_greeting_20260828')),
            ('party holder added to a faction role', lambda p: role(p, 'ru_duma_faction_20211012_ldpr_head')['holder_claims'].append(
                copy.deepcopy(holder(p, 'ru_ldpr_chairman', 2)))),
            ('cross-role claim between parties', lambda p: role(p, 'ru_yabloko_chairman')['claim_ids'].append(
                'ru_kprf_zyuganov_chairman_greeting_20260828')),
            ('game mapping on a new observation', lambda p: org(p, 'ru_dvr_party_self_record').update(reconciled_organization_id='Russia/ru_vybor')),
            ('lifecycle start from the founding congress (APR)', lambda p: org(p, 'ru_apr_party_self_record')['lifecycle'].update({'from': '1993-02-26'})),
            ('lifecycle end from the dissolution (bloc)', lambda p: org(p, 'ru_vybor_rossii_bloc_1993')['lifecycle'].update(until='2001-05-19')),
            ('bloc given a role', lambda p: org(p, 'ru_vybor_rossii_bloc_1993')['roles'].append(copy.deepcopy(role(p, 'ru_dvr_chairman')))),
            ('role kind changed', lambda p: role(p, 'ru_ldpr_chairman').update(kind='party_chair')),
            ('role removed', lambda p: org(p, 'ru_duma_ballot_list_2021_07')['roles'].pop()),
            ('organization removed', lambda p: p['organizations'].pop()),
            ('ballot-list claims changed', lambda p: org(p, 'ru_duma_ballot_list_2021_01')['claim_ids'].append('ru_kprf_v_congress_held_19980523')),
            ('holders reordered', lambda p: role(p, 'ru_yabloko_chairman')['holder_claims'].reverse()),
            ('election re-dated after its attestation', lambda p: claim(p, 'ru_apr_xii_congress_plotnikov_elected_20040428').update(attested_on='2004-05-29')),
            ('undated claim given a date', lambda p: claim(p, 'ru_dvr_x_congress_self_dissolution_decision_2001').update(attested_on='2001-05-19')),
        ]
        party_invariants(self.packet, self.ussr)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration)):
                party_invariants(mutated(change), self.ussr)
        # A party holder added to the USSR Presidency is a cross-institution holder.
        ussr = copy.deepcopy(self.ussr)
        su = next(r for e in ussr['institutions'] for r in e['roles'] if r['id'] == 'su_president')
        su['holder_claims'].append(copy.deepcopy(self.roles['ru_kprf_chairman']['holder_claims'][0]))
        with self.assertRaises(AssertionError):
            party_invariants(self.packet, ussr)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for n in range(1, 11):
            row, = [line for line in table.splitlines() if line.startswith(f'| RU-PTY-{n:02d} ')]
            self.assertRegex(row, r'\*\*(Accepted|Accepted in part):\*\*')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        self.assertIn('\n### Date ledger\n', self.report)
        for heading in ('Sources added', 'Response identities and stability checks', 'Leads not imported',
                        'Sources attempted', 'Suggested next work orders', 'Checks'):
            self.section(heading)
        notes = self.section('Integration notes')
        for text in ('research-index.json', 'test_russia_research_s10h.py', 'test_ussr_russia_transition_c01_05.py',
                     'test_russia_presidents_c01_14.py', 'test_russia_heads_of_government_c01_19.py', 'not stacked'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'russia-party-leaders-1990-2026-28.md', 'claude/c01-ru-28', '2c4d5bd7',
                     'test_russia_party_leaders_c01_28.py', 'pending Codex acceptance'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Russia')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['role_observations'], country['source_claims'], country['mapping_pending']),
                         (14, 295 + CLAIM_TOTAL, 24))
        self.assertEqual([len(w['members']) for w in index['work_orders'] if w['nation'] == 'Russia'], [10, 10, 4])
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Russia'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
