"""CLAUDE-C01-35: the leaders of the Democratic Russia movement and of the Soyuz deputies' group, 1990-1991.

Co-chairs are co-leadership: each holder observation is one person, never a collapsed list, and none is a sole leader. A
holder rests only on a record printing the person in the office on that record's own day (his own words in a Congress
stenogram, the co-chairs' signatures on the Coordinating Council's letter, the RSFSR President's office list); every holder
carries attested_on only, with no from and no until. Speaking or relaying for a group, other deputies' descriptions
('руководитель', 'лидеры', 'координатор', 'председатель оргкомитета'), membership statements, congresses, appeals,
registrations and intermediate attestations stay separate dated claims and never feed a holder. The movement and the
deputies' group stay separate from every state office, from russia.json and from the simulation's party rows."""
import copy
from datetime import datetime
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# The 16 sources this packet appends, in packet order (document dates ascending; the undated invitation by its host date).
NEW_SOURCES = [
    'su_snd3_steno_vol1',
    'su_snd3_steno_vol3',
    'su_rsfsr_snd1_sten_v5',
    'su_rsfsr_snd2_sten_v3',
    'su_snd4_steno_vol1',
    'su_snd4_steno_vol3_soyuz',
    'su_snd4_steno_vol2',
    'su_rsfsr_snd3_sten_v1',
    'su_rsfsr_snd3_sten_v5',
    'su_rsfsr_snd5_sten_v1',
    'su_vs_bulletin1_soyuz_19910826',
    'su_vs_bulletin2_soyuz_19910826',
    'su_snd5_bulletin3_19910903',
    'su_yc_f6_d104_l1_18_19910912',
    'su_yc_f6_d222_l129_199111',
    'su_yc_f6_d104_l128_134_19911212',
]
# Recorded response identity of each new source: (bytes, sha256), downloaded identical twice by this packet at least 30
# minutes apart (and by the research dossier or the independent check).
RESPONSES = {
    'su_snd3_steno_vol1': (14627192, '3339b40a557e54787fd6c9186f81557aaddd02a878d4981e52f6ac56c40acfdb'),
    'su_snd3_steno_vol3': (12922126, 'ae895c021cd21ea893c0c97a5386a434eff9d7c681721162c1999dfa0b07b345'),
    'su_rsfsr_snd1_sten_v5': (18684503, 'da3383db9bcde8f07829b38441970ecf10a31d1706f3ba70c9b8943c389eeb24'),
    'su_rsfsr_snd2_sten_v3': (19806883, 'ccb68d98f52437a491c566152ef65f425052e2c41b7f314f0c331a8324ab794a'),
    'su_snd4_steno_vol1': (25557430, 'a0e2b8470edec278d89323da0e65d71a48c30b55ff5634e8a1a76b9c7bf3ace1'),
    'su_snd4_steno_vol3_soyuz': (12653342, '398d2fa58abd61bfb5e466841d3ba18de6aa0c9170443f73123a4df2a349b897'),
    'su_snd4_steno_vol2': (18853969, 'ee347c29174109f450ddd7d0b40d3a0206e4b1289d92c802d557235d7c820246'),
    'su_rsfsr_snd3_sten_v1': (18252813, '0b65f35688e13fff9fef958b1951d01738513825b44f1159c5576ea3fa3ba076'),
    'su_rsfsr_snd3_sten_v5': (18615464, '65ff1f36f4ecfbc2d8954898f5d9bd0bd78a0b6a4b2b550fa7720baee44eb54d'),
    'su_rsfsr_snd5_sten_v1': (36840143, '392cfe46497cbd1d2937c094c2cbbe6a83ebaf307f6649f7a7aab941a0fc1805'),
    'su_vs_bulletin1_soyuz_19910826': (2252696, '849b9dae8c5d3580df911819ff466f9e6997e3119c326b1cf638b6e86239776a'),
    'su_vs_bulletin2_soyuz_19910826': (1530044, '22562b5be16890047e3f62b4cf253ca9daac0fa374ca3870a63bee62723fd5bd'),
    'su_snd5_bulletin3_19910903': (2146995, '8eeae4731e35ac7e25d099bb85a95610382cbe215ab6b49820a57675dcd9ac26'),
    'su_yc_f6_d104_l1_18_19910912': (4143217, 'ed39047202ab5b24b0469a1268055b18e2453c2f92acfba159292f6912b5e765'),
    'su_yc_f6_d222_l129_199111': (106744, 'a553a66a958da6d99f2f02ab5f9a0d8889c07b0e8e92d953d27f16bc2d8819d2'),
    'su_yc_f6_d104_l128_134_19911212': (4402082, '37c8a1415f12ff66f105d3146c78ebc6b41a6ace5fea11c39dff9b35a11293d8'),
}
# Every new identity is a raw Internet Archive capture made before the cutoff.
ARCHIVED = {
    'su_snd3_steno_vol1': '20241206074206',
    'su_snd3_steno_vol3': '20240903210942',
    'su_rsfsr_snd1_sten_v5': '20220313025049',
    'su_rsfsr_snd2_sten_v3': '20220313024936',
    'su_snd4_steno_vol1': '20240902002244',
    'su_snd4_steno_vol3_soyuz': '20241124015727',
    'su_snd4_steno_vol2': '20240901144454',
    'su_rsfsr_snd3_sten_v1': '20220313024757',
    'su_rsfsr_snd3_sten_v5': '20220313030126',
    'su_rsfsr_snd5_sten_v1': '20220313024839',
    'su_vs_bulletin1_soyuz_19910826': '20250718143607',
    'su_vs_bulletin2_soyuz_19910826': '20250718143607',
    'su_snd5_bulletin3_19910903': '20240901234307',
    'su_yc_f6_d104_l1_18_19910912': '20230622210558',
    'su_yc_f6_d222_l129_199111': '20230623225515',
    'su_yc_f6_d104_l128_134_19911212': '20230622210559',
}
# Byte-identical live static files attached to a capture (the RSFSR stenogram host has none that verifies).
LIVE = {
    'su_snd3_steno_vol1': ('https://snd.sssr.su/III/I.pdf', 14627192),
    'su_snd3_steno_vol3': ('https://snd.sssr.su/III/III.pdf', 12922126),
    'su_snd4_steno_vol1': ('https://snd.sssr.su/IV/I.pdf', 25557430),
    'su_snd4_steno_vol3_soyuz': ('https://snd.sssr.su/IV/III.pdf', 12653342),
    'su_snd4_steno_vol2': ('https://snd.sssr.su/IV/II.pdf', 18853969),
    'su_vs_bulletin1_soyuz_19910826': ('https://sten.vs.sssr.su/12/6/1.pdf', 2252696),
    'su_vs_bulletin2_soyuz_19910826': ('https://sten.vs.sssr.su/12/6/2.pdf', 1530044),
    'su_snd5_bulletin3_19910903': ('https://snd.sssr.su/V/3.pdf', 2146995),
    'su_yc_f6_d104_l1_18_19910912': ('https://yeltsin.ru/uploads/upload/2015/03/27/f6_o1_d104_001_aJ7dx2A.pdf', 4143217),
    'su_yc_f6_d222_l129_199111': ('https://yeltsin.ru/uploads/upload/2015/08/20/f6_o1_d222_060_0WLtD2F.pdf', 106744),
    'su_yc_f6_d104_l128_134_19911212': ('https://yeltsin.ru/uploads/upload/2015/03/27/f6_o1_d104_006_oscl6gt.pdf', 4402082),
}
# Every claim of this packet: (attested_on or None, event kind, observation, role or None, organization).
EVENTS = {
    'su_snd3_deputy_speaks_for_soyuz_19900312': ('1990-03-12', 'speaks_on_behalf_of_group', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd3_blokhin_delivers_soyuz_statement_19900313': ('1990-03-13', 'speaks_on_behalf_of_group', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd3_blokhin_reports_soyuz_pre_congress_meeting_19900313': ('1990-03-13', 'group_decision_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd3_blokhin_soyuz_membership_19900313': ('1990-03-13', 'membership_size_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd3_alksnis_relays_soyuz_presidential_nominations_19900314': ('1990-03-14', 'group_nomination_relayed', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd3_presiding_officer_soyuz_over_300_19900314': ('1990-03-14', 'membership_size_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd3_kim_relays_soyuz_chair_nominations_19900315': ('1990-03-15', 'group_nomination_relayed', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_dr_bloc_appeal_calls_for_movement_19900622': ('1990-06-22', 'call_to_found_movement', 'SU-DR-01', None, 'su_democratic_russia'),
    'su_dr_representatives_council_election_announced_19901207': ('1990-12-07', 'movement_body_election_announced', 'SU-DR-01', None, 'su_democratic_russia'),
    'su_snd4_gninenko_reports_soyuz_meeting_decision_19901217': ('1990-12-17', 'group_decision_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd4_soyuz_commission_nominees_general_meeting_19901217': ('1990-12-17', 'group_decision_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd4_golyakov_murashev_dr_orgcommittee_chair_19901217': ('1990-12-17', 'organizing_committee_chair_reference', 'SU-DR-02', None, 'su_democratic_russia'),
    'su_snd4_bisher_refers_to_soyuz_leader_19901218': ('1990-12-18', 'leadership_reference_by_another_deputy', 'SU-SOYUZ-04', 'su_soyuz_co_chair', 'su_soyuz_deputies_group'),
    'su_snd4_zaslavsky_murashev_dr_orgcommittee_chair_19901219': ('1990-12-19', 'organizing_committee_chair_reference', 'SU-DR-02', None, 'su_democratic_russia'),
    'su_snd4_zaslavsky_dr_council_member_19901219': ('1990-12-19', 'member_self_description', 'SU-DR-02', None, 'su_democratic_russia'),
    'su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220': ('1990-12-20', 'in_office_attestation', 'SU-SOYUZ-02', 'su_soyuz_co_chair', 'su_soyuz_deputies_group'),
    'su_snd4_res_1842i_names_soyuz_nominees_19901217': ('1990-12-17', 'group_nominees_elected', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd4_sazonov_refers_to_soyuz_leaders_19901226': ('1990-12-26', 'leadership_reference_by_another_deputy', 'SU-SOYUZ-04', 'su_soyuz_co_chair', 'su_soyuz_deputies_group'),
    'su_snd4_kogan_reports_soyuz_rotation_list_19901226': ('1990-12-26', 'group_decision_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd4_chekhoev_reports_soyuz_meeting_19901227': ('1990-12-27', 'group_decision_reported', 'SU-SOYUZ-01', None, 'su_soyuz_deputies_group'),
    'su_snd4_soyuz_registration_roll_19901225': ('1990-12-25', 'group_registration_reported', 'SU-SOYUZ-03', None, 'su_soyuz_deputies_group'),
    'su_dr_coordinating_council_march_application_19910328': ('1991-03-28', 'movement_body_act_reported', 'SU-DR-01', None, 'su_democratic_russia'),
    'su_dr_dmitriev_one_of_co_chairs_19910405': ('1991-04-05', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_dr_afanasyev_co_chair_statement_19910715': ('1991-07-15', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_vs26_bogdanov_refers_to_soyuz_leaders_19910826': ('1991-08-26', 'leadership_reference_by_another_deputy', 'SU-SOYUZ-04', 'su_soyuz_co_chair', 'su_soyuz_deputies_group'),
    'su_vs26_boyars_proposal_names_soyuz_leaders_19910826': ('1991-08-26', 'leadership_reference_by_another_deputy', 'SU-SOYUZ-04', 'su_soyuz_co_chair', 'su_soyuz_deputies_group'),
    'su_vs26_kogan_active_member_of_soyuz_19910826': ('1991-08-26', 'member_self_description', 'SU-SOYUZ-04', None, 'su_soyuz_deputies_group'),
    'su_snd5_kraiko_refers_to_soyuz_leaders_19910903': ('1991-09-03', 'leadership_reference_by_another_deputy', 'SU-SOYUZ-04', 'su_soyuz_co_chair', 'su_soyuz_deputies_group'),
    'su_dr_afanasyev_signs_as_co_chair_19910912': ('1991-09-12', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_dr_ponomarev_signs_as_co_chair_19910912': ('1991-09-12', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_dr_murashev_signs_as_co_chair_19910912': ('1991-09-12', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915': ('1991-09-15', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_dr_representatives_council_plenum_19910915': ('1991-09-15', 'movement_body_meeting_reported', 'SU-DR-01', None, 'su_democratic_russia'),
    'su_dr_second_congress_invitation_undated': (None, 'congress_scheduled', 'SU-DR-04', None, 'su_democratic_russia'),
    'su_dr_yakunin_listed_as_co_chair_19911212': ('1991-12-12', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
    'su_dr_ponomarev_listed_as_co_chair_19911212': ('1991-12-12', 'in_office_attestation', 'SU-DR-03', 'su_dr_co_chair', 'su_democratic_russia'),
}
# Exact holders, in chronological order, as (name, attested_on, from, until), and the claim each rests on.
HOLDERS = {
    'su_dr_co_chair': [
        ('Виктор Владимирович Дмитриев', '1991-04-05', None, None),
        ('Юрий Николаевич Афанасьев', '1991-07-15', None, None),
        ('Юрий Николаевич Афанасьев', '1991-09-12', None, None),
        ('Лев Александрович Пономарев', '1991-09-12', None, None),
        ('А. Мурашев', '1991-09-12', None, None),
        ('Глеб Павлович Якунин', '1991-12-12', None, None),
        ('Лев Александрович Пономарев', '1991-12-12', None, None),
    ],
    'su_soyuz_co_chair': [
        ('Анатолий Георгиевич Чехоев', '1990-12-20', None, None),
    ],
}
HOLDER_CLAIMS = {
    'su_dr_co_chair': [['su_dr_dmitriev_one_of_co_chairs_19910405'], ['su_dr_afanasyev_co_chair_statement_19910715'], ['su_dr_afanasyev_signs_as_co_chair_19910912'], ['su_dr_ponomarev_signs_as_co_chair_19910912'], ['su_dr_murashev_signs_as_co_chair_19910912'], ['su_dr_yakunin_listed_as_co_chair_19911212'], ['su_dr_ponomarev_listed_as_co_chair_19911212']],
    'su_soyuz_co_chair': [['su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220']],
}
# Every extract row that names a holder, exactly; every other row has holder_name None.
ROW_HOLDERS = {
    'su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220': 'Анатолий Георгиевич Чехоев',
    'su_dr_dmitriev_one_of_co_chairs_19910405': 'Виктор Владимирович Дмитриев',
    'su_dr_afanasyev_co_chair_statement_19910715': 'Юрий Николаевич Афанасьев',
    'su_dr_afanasyev_signs_as_co_chair_19910912': 'Юрий Николаевич Афанасьев',
    'su_dr_ponomarev_signs_as_co_chair_19910912': 'Лев Александрович Пономарев',
    'su_dr_murashev_signs_as_co_chair_19910912': 'А. Мурашев',
    'su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915': 'Лев Александрович Пономарев',
    'su_dr_yakunin_listed_as_co_chair_19911212': 'Глеб Павлович Якунин',
    'su_dr_ponomarev_listed_as_co_chair_19911212': 'Лев Александрович Пономарев',
}
# Rendered and read pages recorded in each extract (PDF pages, or facsimile pages for the Yeltsin Center scans).
PAGES = {
    'su_snd3_steno_vol1': ([3, 60, 100, 106, 135, 136, 137, 138, 139, 424], []),
    'su_snd3_steno_vol3': ([3, 5, 6, 24, 56, 84, 338], []),
    'su_rsfsr_snd1_sten_v5': ([1, 336, 337], []),
    'su_rsfsr_snd2_sten_v3': ([68, 239, 240], []),
    'su_snd4_steno_vol1': ([3, 5, 16, 66, 78, 120, 121, 130, 171, 188, 189, 327, 363, 372, 405, 406, 617], []),
    'su_snd4_steno_vol3_soyuz': ([3, 68, 90, 93, 138, 139, 208, 233, 293, 294, 295, 401], []),
    'su_snd4_steno_vol2': ([1, 3, 368, 423, 424, 544, 546], []),
    'su_rsfsr_snd3_sten_v1': ([30, 36, 214, 215, 216], []),
    'su_rsfsr_snd3_sten_v5': ([1, 27, 222, 224], []),
    'su_rsfsr_snd5_sten_v1': ([1, 156, 368], []),
    'su_vs_bulletin1_soyuz_19910826': ([1, 3, 22], []),
    'su_vs_bulletin2_soyuz_19910826': ([1, 3, 22, 27, 28, 29, 30], []),
    'su_snd5_bulletin3_19910903': ([1, 3, 24], []),
    'su_yc_f6_d104_l1_18_19910912': ([], [1, 5, 6]),
    'su_yc_f6_d222_l129_199111': ([], [1]),
    'su_yc_f6_d104_l128_134_19911212': ([], [1]),
}
# SHA-256 of every existing USSR role and holder (name, attested_on, from, until) at the base 44098c5a; unchanged here.
USSR_BASE_HOLDERS_SHA256 = '3e3571c3a64375cb81fb1b5e7605e36baebb5f74524fd9e90fcd908ef3b77506'
# CLAUDE-C01-41 later appended dated su_cpsu holders resting on these sources (pinned in test_ussr_cpsu_general_secretary_c01_41);
# the guard skips only those holders of the two CPSU roles and still covers every holder that existed at 44098c5a, exactly as before.
C01_41_SOURCES = {'su_pravda_no37_19900206', 'su_pravda_no192_19900711', 'su_pravda_no193_19900712', 'su_pravda_no194_19900713',
                  'su_pravda_no195_19900714', 'su_izv_tsk_1991_08', 'su_pravda_no201_19910822', 'su_vs_bulletin1_cpsu_19910826',
                  'su_ved_1991_35_cpsu', 'su_snd5_bulletin3_cpsu_19910903', 'su_ved_1991_36_cpsu',
                  'su_kremlin_rsfsr_ukaz_169_19911106'}
# CLAUDE-C01-49 later appended su_president and su_government_head holders resting only on these sources (pinned in
# test_ussr_government_president_c01_49); the guard skips only those and still covers every holder present at 44098c5a.
C01_49_SOURCES = {'su_snd3_steno_vol3_president', 'su_pravda_no13_19910115', 'su_pravda_no20_19910123', 'su_izv_197_19910820',
                  'su_ved_1991_35_president', 'su_ved_1991_41_president'}

DR, DR_ROLE, SOYUZ, SOYUZ_ROLE = 'su_democratic_russia', 'su_dr_co_chair', 'su_soyuz_deputies_group', 'su_soyuz_co_chair'
DR_NAME = 'Движение «Демократическая Россия» — Democratic Russia movement'
SOYUZ_NAME = ("Депутатская группа «Союз» — Soyuz deputies' group (a group of USSR people's deputies in the Congress of People's "
              "Deputies)")
ORGS = {DR: (DR_NAME, 'political_movement', 'republic'), SOYUZ: (SOYUZ_NAME, 'deputies_group', 'union')}
ORG_ROLES = {DR: [DR_ROLE], SOYUZ: [SOYUZ_ROLE]}
ROLES = {
    DR_ROLE: ('Сопредседатель движения «Демократическая Россия» (Координационного совета) — Co-chair of the Democratic Russia movement',
              'party_leader'),
    SOYUZ_ROLE: ("Сопредседатель депутатской группы «Союз» — Co-chair of the Soyuz deputies' group", 'parliamentary_leader'),
}
CO_ROLES = (DR_ROLE, SOYUZ_ROLE)
INSTITUTIONS = ['su_presidency', 'su_congress_peoples_deputies', 'su_supreme_soviet', 'su_government']
UNDATED = ('su_dr_second_congress_invitation_undated',)
# The people whose leadership this packet reviews: the six holders and the four other deputies an accusing proposal of
# 26 August 1991 calls the Soyuz group's 'лидеры' (claims only). At most ten.
PEOPLE = {'Виктор Владимирович Дмитриев', 'Юрий Николаевич Афанасьев', 'Лев Александрович Пономарев', 'А. Мурашев',
          'Глеб Павлович Якунин', 'Анатолий Георгиевич Чехоев', 'Коган Е. В.', 'Алкснис В. И.', 'Блохин Ю. В.', 'Петрушенко Н. С.'}
# Only in-office attestations may feed a holder, and only the claims pinned in HOLDER_CLAIMS do.
HOLDER_KINDS = {'in_office_attestation'}
REFERENCE_KINDS = {'leadership_reference_by_another_deputy', 'organizing_committee_chair_reference', 'member_self_description'}
BODY_KINDS = {'speaks_on_behalf_of_group', 'group_decision_reported', 'membership_size_reported', 'group_nomination_relayed',
              'group_nominees_elected', 'group_registration_reported', 'call_to_found_movement', 'movement_body_election_announced',
              'movement_body_act_reported', 'movement_body_meeting_reported', 'congress_scheduled'}
VOCABULARY = HOLDER_KINDS | REFERENCE_KINDS | BODY_KINDS
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in {c for ids in HOLDER_CLAIMS.values() for group in ids for c in group})
# Days a holder must never start or end on: every dated claim of this packet, the founding congress and the second congress
# named only in leads or without a year, the meeting of 12 October 1991 and the Union's end.
NEVER_BOUNDARY = {v[0] for v in EVENTS.values() if v[0]} | {'1990-10-20', '1990-10-21', '1991-10-12', '1991-11-09', '1991-11-10',
                                                            '1991-12-25', '1991-12-26'}
# Distinct events that must stay distinct and in order.
ORDERED = (
    ('su_snd3_deputy_speaks_for_soyuz_19900312', 'su_snd3_blokhin_delivers_soyuz_statement_19900313'),
    ('su_snd3_blokhin_soyuz_membership_19900313', 'su_snd4_soyuz_registration_roll_19901225'),
    ('su_snd4_soyuz_commission_nominees_general_meeting_19901217', 'su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220'),
    ('su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220', 'su_vs26_boyars_proposal_names_soyuz_leaders_19910826'),
    ('su_vs26_boyars_proposal_names_soyuz_leaders_19910826', 'su_snd5_kraiko_refers_to_soyuz_leaders_19910903'),
    ('su_dr_bloc_appeal_calls_for_movement_19900622', 'su_dr_representatives_council_election_announced_19901207'),
    ('su_snd4_golyakov_murashev_dr_orgcommittee_chair_19901217', 'su_snd4_zaslavsky_murashev_dr_orgcommittee_chair_19901219'),
    ('su_snd4_zaslavsky_murashev_dr_orgcommittee_chair_19901219', 'su_dr_murashev_signs_as_co_chair_19910912'),
    ('su_dr_coordinating_council_march_application_19910328', 'su_dr_dmitriev_one_of_co_chairs_19910405'),
    ('su_dr_afanasyev_co_chair_statement_19910715', 'su_dr_afanasyev_signs_as_co_chair_19910912'),
    ('su_dr_ponomarev_signs_as_co_chair_19910912', 'su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915'),
    ('su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915', 'su_dr_ponomarev_listed_as_co_chair_19911212'),
)
# Leads (encyclopaedias, wikis, directories, newspapers, dynamic item pages, searches, non-official transcriptions and
# documents after the period): never a recorded identity URL.
LEAD_URL_MARKERS = ('wikipedia', 'wikimedia', 'panorama', 'nasledie', 'yandex', 'izvestia', 'izvestija', 'politika.su',
                    'archive/paperwork', 'archive/search', 'sten.sr.vs.sssr.su', 'sten.vs.sssr.su/13/', 'vedomosti.rsfsr',
                    'vedomosti.sssr.su', 'VIsnd', 'elib.shpl.ru', 'prlib', 'rusneb', 'garant', 'consultant', 'alexanderyakovlev')
VOLATILE_URL = re.compile(r'([?&](cb|_cb|_chk|nocache|exp|sig|token|q|__ctx)=|sessid|PHPSESSID|ysclid|/web/\d{4}id_/|/web/\d{14}/|'
                          r'/cdx/|/search|advancedsearch)')
REPORT = research.RESEARCH / 'ussr-democratic-russia-soyuz-1990-1991-35.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-35.md'
DECISIONS = {'SU-DR-01': 'Accepted in part', 'SU-DR-02': 'Accepted in part', 'SU-DR-03': 'Accepted in part',
             'SU-DR-04': 'Accepted in part', 'SU-DR-05': 'Not found', 'SU-SOYUZ-01': 'Accepted', 'SU-SOYUZ-02': 'Accepted in part',
             'SU-SOYUZ-03': 'Accepted', 'SU-SOYUZ-04': 'Accepted in part', 'SU-SOYUZ-05': 'Not found'}


def roles_of(packet):
    return {r['id']: (e, r) for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles']}


def c35_invariants(ussr, russia):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in ussr['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in ussr['sources'] for c in s['claims']}
    orgs = {e['id']: e for e in ussr['organizations']}
    assert list(orgs) == ['su_cpsu', DR, SOYUZ], list(orgs)
    assert [e['id'] for e in ussr['institutions']] == INSTITUTIONS
    for oid, (name, kind, level) in ORGS.items():
        org = orgs[oid]
        assert (org['name'], org['kind'], org['jurisdiction']['level']) == (name, kind, level), oid
        assert org['jurisdiction']['nation'] == 'USSR' and org['jurisdiction']['automatic_successor_mapping'] is False, oid
        assert org['represented_party_ids'] == [] and 'reconciled_organization_id' not in org, oid
        assert (org['lifecycle']['from'], org['lifecycle']['until']) == (None, None), oid
        assert [r['id'] for r in org['roles']] == ORG_ROLES[oid], oid
    roles = roles_of(ussr)
    for role_id, (title, kind) in ROLES.items():
        assert (roles[role_id][1]['title'], roles[role_id][1]['kind']) == (title, kind), role_id
    # No state-office kind among this packet's roles; the Union's state roles are unchanged in kind and number.
    kinds = {}
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                for r in entry['roles']:
                    kinds.setdefault(r['kind'], []).append(r['id'])
    assert [r for r in kinds['head_of_government'] if r.startswith('su_')] == ['su_government_head']
    assert [r for r in kinds['head_of_state'] if r.startswith('su_')] == ['su_president']
    assert [r for r in kinds['party_leader'] if r.startswith('su_')] == ['su_cpsu_general_secretary', DR_ROLE]
    assert [r for r in kinds['parliamentary_leader'] if r.startswith('su_')] == [SOYUZ_ROLE]
    # Every existing USSR role and holder is unchanged.
    base = [(r['id'], [(h['name'], h.get('attested_on'), h['from'], h['until']) for h in r['holder_claims']
                       if isinstance(h, dict) and not (r['id'] in ('su_cpsu_general_secretary', 'su_cpsu_deputy_general_secretary')
                                                       and set(h['sources']) <= C01_41_SOURCES)
                       and not (r['id'] in ('su_president', 'su_government_head') and h['sources']
                                and set(h['sources']) <= C01_49_SOURCES)])
            for g in ('organizations', 'institutions') for e in ussr[g] if e['id'] not in (DR, SOYUZ) for r in e['roles']]
    assert hashlib.sha256(json.dumps(base, ensure_ascii=False).encode('utf-8')).hexdigest() == USSR_BASE_HOLDERS_SHA256
    # Each holder rests on one in-office claim of its own role, dated that day, with no from and no until (rules first, so a
    # mutation meets the rule it breaks; the exact lists are compared at the end).
    for role_id in HOLDERS:
        role = roles[role_id][1]
        holders = role['holder_claims']
        days = [h['attested_on'] for h in holders]
        assert all(days), role_id
        assert days == sorted(days), role_id
        for h in holders:
            assert not {h['from'], h['until']} & NEVER_BOUNDARY, (role_id, h['name'])
            assert h['from'] is None and h['until'] is None, h['name']
            assert not re.search(r'(?i)acting|исполняющ|временно|и\. о\.', h['name']), h['name']
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert all(cid in role['claim_ids'] for cid in h['claim_ids']), h['name']
            assert all(claims[cid]['attested_on'] == h['attested_on'] for cid in h['claim_ids']), h['name']
            assert h['sources'] == list(dict.fromkeys(owner[cid] for cid in h['claim_ids'])), h['name']
            assert all(EVENTS[cid][1] in HOLDER_KINDS and EVENTS[cid][3] == role_id for cid in h['claim_ids']), h['name']
    # Co-leadership is never collapsed: each observation is one person (no list of names in one holder).
    for role_id in CO_ROLES:
        for h in roles[role_id][1]['holder_claims']:
            assert not re.search(r',| и | and |/|;', h['name']), h['name']
            assert len(h['claim_ids']) == 1, h['name']
    # Holders of one role never appear on the other.
    names = {rid: {h['name'] for h in roles[rid][1]['holder_claims']} for rid in CO_ROLES}
    assert not names[DR_ROLE] & names[SOYUZ_ROLE]
    # The exact holders, in chronological order, and the claim each rests on.
    for role_id, expected in HOLDERS.items():
        holders = roles[role_id][1]['holder_claims']
        assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == expected, role_id
        assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS[role_id], role_id
    # Every claim of this packet is cited by exactly its own role or organization and by nothing else in either packet.
    ours = set(EVENTS)
    expected = {key: [] for key in list(ROLES) + [DR, SOYUZ]}
    for cid, (_, _, _, role_id, entry) in EVENTS.items():
        expected[role_id or entry].append(cid)
    for key, ids in expected.items():
        target = roles[key][1] if key in ROLES else orgs[key]
        assert target['claim_ids'] == ids, key
        assert target['sources'] == list(dict.fromkeys(owner[c] for c in ids)), key
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                assert not (set(entry['claim_ids']) & ours) - set(expected.get(entry['id'], [])), entry['id']
                for r in entry['roles']:
                    cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    assert not (cited & ours) - set(expected.get(r['id'], [])), r['id']
    # Party (movement, group) office and state office stay separate both ways.
    for e in ussr['institutions'] + [orgs['su_cpsu']]:
        for r in e['roles']:
            cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
            assert not cited & ours and not set(r['sources']) & set(NEW_SOURCES), r['id']
        assert not set(e['claim_ids']) & ours and not set(e['sources']) & set(NEW_SOURCES), e['id']
    for role_id in ROLES:
        assert all(owner[c] in NEW_SOURCES for c in roles[role_id][1]['claim_ids']), role_id
    # Dates that must never be a holder boundary stay out of every lifecycle bound (holder bounds are checked above).
    for oid in (DR, SOYUZ):
        assert not {orgs[oid]['lifecycle']['from'], orgs[oid]['lifecycle']['until']} & NEVER_BOUNDARY, oid
    for earlier, later in ORDERED:
        assert claims[earlier]['attested_on'] < claims[later]['attested_on'], (earlier, later)
    for cid in ours:
        if cid in UNDATED:
            assert 'attested_on' not in claims[cid] and 'period' not in claims[cid], cid
            continue
        assert '1990-01-01' <= claims[cid]['attested_on'] <= '1991-12-25', cid
        assert claims[cid]['attested_on'] <= research.CUTOFF, cid


class UssrDemocraticRussiaSoyuzTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8')
        cls.russia_raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.packet, cls.russia = json.loads(cls.raw), json.loads(cls.russia_raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.roles = roles_of(cls.packet)
        cls.orgs = {e['id']: e for e in cls.packet['organizations']}
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
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), (16, 36))
        # CLAUDE-C01-41 appended 12 sources after these 16 (positions 56-67), pinned in test_ussr_cpsu_general_secretary_c01_41,
        # and CLAUDE-C01-49 6 more (positions 68-73), pinned in test_ussr_government_president_c01_49.
        self.assertEqual([s['id'] for s in self.packet['sources'][40:56]], NEW_SOURCES)
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), (74, 174, 7, 8))
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], list(EVENTS))
        dates = [self.sources[sid]['document_date'] or '1991-11-09' for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        self.assertEqual({v[1] for v in EVENTS.values()}, VOCABULARY)
        self.assertEqual(len(VOCABULARY), sum(map(len, (HOLDER_KINDS, REFERENCE_KINDS, BODY_KINDS))))
        self.assertEqual({v[2] for v in EVENTS.values()},
                         {f'SU-DR-0{n}' for n in range(1, 5)} | {f'SU-SOYUZ-0{n}' for n in range(1, 5)})
        self.assertEqual(re.findall(r'^### (SU-(?:DR|SOYUZ)-\d\d)\b', self.report, re.M),
                         [f'SU-DR-0{n}' for n in range(1, 6)] + [f'SU-SOYUZ-0{n}' for n in range(1, 6)])
        # At most ten people: the holders and the four deputies named as the Soyuz group's 'лидеры' in a claim.
        self.assertLessEqual(len(PEOPLE), 10)
        holders = {h['name'] for rid in CO_ROLES for h in self.roles[rid][1]['holder_claims']}
        self.assertLessEqual(holders, PEOPLE)
        named = set(self.rows['su_vs26_boyars_proposal_names_soyuz_leaders_19910826']['persons_named'])
        self.assertLessEqual(PEOPLE - holders, named)
        for sid in NEW_SOURCES:
            self.assertTrue(sid.startswith('su_'))
            for c in self.sources[sid]['claims']:
                self.assertTrue(c['id'].startswith('su_'))

    def test_holders_are_exactly_as_intended(self):
        c35_invariants(self.packet, self.russia)
        # Co-chairs share days: three sign the same letter, two appear in the same list; each is still one observation.
        dr = self.roles[DR_ROLE][1]['holder_claims']
        self.assertEqual([h['name'] for h in dr if h['attested_on'] == '1991-09-12'],
                         ['Юрий Николаевич Афанасьев', 'Лев Александрович Пономарев', 'А. Мурашев'])
        self.assertEqual([h['name'] for h in dr if h['attested_on'] == '1991-12-12'],
                         ['Глеб Павлович Якунин', 'Лев Александрович Пономарев'])
        for role_id in CO_ROLES:
            role = self.roles[role_id][1]
            self.assertIn('co-leadership', role['scope_note'].lower())
            for h in role['holder_claims']:
                self.assertIn('Co-leadership', h['uncertainty'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        for role_id in CO_ROLES:
            for h in self.roles[role_id][1]['holder_claims']:
                self.assertIsNone(h['from'])
                self.assertIsNone(h['until'])
        for oid in (DR, SOYUZ):
            lifecycle = self.orgs[oid]['lifecycle']
            self.assertEqual((lifecycle['status'], lifecycle['from'], lifecycle['until'], lifecycle['precision']),
                             ('documented_at_specific_observations', None, None, 'unknown'))
        # The undated invitation carries no structured date; the letter keeps the date it bears, not its registration day.
        invitation = self.claims['su_dr_second_congress_invitation_undated']
        self.assertNotIn('attested_on', invitation)
        self.assertNotIn('period', invitation)
        self.assertIn('no year', invitation['uncertainty'].lower())
        for cid in ('su_dr_afanasyev_signs_as_co_chair_19910912', 'su_dr_ponomarev_signs_as_co_chair_19910912',
                    'su_dr_murashev_signs_as_co_chair_19910912'):
            self.assertEqual(self.claims[cid]['attested_on'], '1991-09-12')
            self.assertIn('12,10,1991', self.claims[cid]['text'])
            self.assertIn('12 October 1991', self.claims[cid]['uncertainty'])
        self.assertNotIn('"1991-10-12"', self.raw)
        # Descriptions by other deputies, relayed decisions and membership statements are claims, never holders.
        for cid, (_, kind, _, role_id, _) in EVENTS.items():
            if kind in REFERENCE_KINDS | BODY_KINDS:
                self.assertIn(cid, NEVER_HOLDER)
            if kind == 'in_office_attestation':
                self.assertIn(role_id, CO_ROLES)
        self.assertIn('never a holder', self.claims['su_vs26_boyars_proposal_names_soyuz_leaders_19910826']['uncertainty'].lower())

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual((extract['access_method'], extract['published_date'], extract['document_date']),
                             (source['access_method'], source['published_date'], source['document_date']))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-29', '2026-09-29'))
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid], sid)
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('recorded identity is the uncompressed body', extract['provenance_note'])
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('no source artwork or portrait copied', extract['rights_note'])
            self.assertIn('CLAUDE-C01-35 only', extract['bounded_scope'])
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
            if source['source_type'].endswith('_non_official_host'):
                self.assertIn('non-official', source['publisher'], sid)
            snapshot = source['snapshot']
            self.assertEqual(snapshot['kind'], 'derived_factual_extract')
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/ussr-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            # Every identity is a raw pre-cutoff capture; attached live files are byte-identical.
            url = urlsplit(source['url'])
            self.assertEqual((url.scheme, url.hostname), ('https', 'web.archive.org'))
            self.assertTrue(url.path.startswith(f'/web/{ARCHIVED[sid]}id_/http'), sid)
            self.assertLess(ARCHIVED[sid], '20260907')
            self.assertEqual(source['url'].split('id_/', 1)[1], source['original_url'])
            self.assertEqual((extract['source_response_url'], extract['original_url']), (source['url'], source['original_url']))
            self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
            if sid in LIVE:
                live = extract['live_file_response']
                self.assertEqual((live['url'], live['bytes'], live['sha256']), (*LIVE[sid], RESPONSES[sid][1]))
                self.assertEqual(live['url'], source['original_url'])
            else:
                self.assertNotIn('live_file_response', extract)
            # This packet downloaded every response twice, at least 30 minutes apart, with identical bytes.
            downloads = extract['downloads']
            self.assertEqual([d['response'] for d in downloads], ['source_response'] + (['live_file_response'] if sid in LIVE else []))
            for d in downloads:
                first, second = (datetime.fromisoformat(t.replace('Z', '+00:00')) for t in d['downloaded_at'])
                self.assertGreaterEqual((second - first).total_seconds(), 1800, (sid, d['response']))
                self.assertEqual((d['bytes'], d['sha256']), RESPONSES[sid], sid)
                self.assertEqual(d['result'], 'identical on both downloads', sid)
            self.assertRegex(extract['stability_check'], r'This packet: downloaded at 2026-09-29T\d\d:\d\d:\d\dZ and again at 2026-09-29T')
            self.assertEqual((extract['visual_review']['pdf_pages_one_based'], extract['visual_review']['facsimile_pages_one_based']),
                             PAGES[sid], sid)
            self.assertIn('read from the', extract['visual_review']['method'])
            # Rows repeat the packet claims exactly, in order, keyed by claim_id.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row.get('attested_on')),
                                 (claim['text'], claim['locator'], claim.get('attested_on')))
                self.assertNotIn('name', row)
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['role_id'], row['observation_id'])
                          for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual({cid: row['holder_name'] for cid, row in self.rows.items() if row['holder_name']}, ROW_HOLDERS)
        for cid, row in self.rows.items():
            if cid not in ROW_HOLDERS:
                self.assertIsNone(row['holder_name'], cid)

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        urls = []
        for sid in NEW_SOURCES:
            extract = self.extracts[sid]
            urls += [self.sources[sid]['url'], extract['source_response_url'], self.sources[sid]['original_url']]
            urls += [d['url'] for d in extract['downloads']]
            if 'live_file_response' in extract:
                urls.append(extract['live_file_response']['url'])
        for url in urls:
            self.assertIsNone(VOLATILE_URL.search(url), url)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url, (marker, url))
        # No lead URL is written anywhere in the packet file.
        for marker in ('wikipedia.org', 'wikimedia.org', 'panorama.ru', 'nasledie.ru', 'yeltsin.ru/archive/search',
                       'sten.sr.vs.sssr.su', 'sssr.su/VIsnd.pdf'):
            self.assertNotIn(marker, self.raw, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('nasledie.ru', 'Wikipedia', 'Commons', 'VIsnd.pdf', 'Панорама'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_separation_from_state_offices_russia_and_the_simulation(self):
        for sid in NEW_SOURCES:
            self.assertNotIn(f'"{sid}"', self.russia_raw)
        for cid in EVENTS:
            self.assertNotIn(f'"{cid}"', self.russia_raw)
        for oid in (DR, SOYUZ):
            self.assertNotIn(f'"{oid}"', self.russia_raw)
            self.assertIn('not mapped', self.orgs[oid]['identity_note'])
        self.assertIn('USSR/su_dr', self.orgs[DR]['identity_note'])
        self.assertIn('USSR/su_soyuz', self.orgs[SOYUZ]['identity_note'])
        self.assertIn('russia.json', self.orgs[DR]['jurisdiction']['note'])
        self.assertIn('separate identities', self.orgs[DR]['identity_note'])
        # The existing records of this packet are untouched: the first 40 sources and the five earlier entries.
        orgs = [e['id'] for e in self.packet['organizations']]
        self.assertEqual(orgs[0], 'su_cpsu')
        # CLAUDE-C01-41 appended 12 sources (positions 56-67), pinned in test_ussr_cpsu_general_secretary_c01_41, and
        # CLAUDE-C01-49 6 (positions 68-73), pinned in test_ussr_government_president_c01_49.
        self.assertEqual(len(self.packet['sources']), 74)
        # The RSFSR deputies' faction and bloc records are not imported as claims on the movement.
        self.assertNotIn('su_rsfsr_dr_group_registered_19900525', self.raw)
        self.assertNotIn('su_rsfsr_dr_faction_coordinator_19911211', self.raw)

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

        def role(packet, role_id=DR_ROLE):
            return roles_of(packet)[role_id][1]

        def holder(packet, index, role_id=DR_ROLE):
            return role(packet, role_id)['holder_claims'][index]

        def org(packet, oid=DR):
            return next(e for e in packet['organizations'] if e['id'] == oid)

        validator_cases = [
            (lambda p: source(p, 'su_yc_f6_d104_l1_18_19910912')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'su_snd4_steno_vol1')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 0).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 1).update({'from': '1991-09-12', 'until': '1991-07-15'}), 'Reversed historical interval'),
            (lambda p: holder(p, 0)['claim_ids'].append('su_dr_yakunin_listed_as_co_chair_19911212'), 'cited source'),
            (lambda p: source(p, 'su_yc_f6_d222_l129_199111').update(url='http://yeltsin.ru/x.pdf'), 'Invalid public source URL'),
            (lambda p: org(p).update(represented_party_ids=['USSR/su_dr']), 'foreign represented party mapping'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def insert_in_order(holders, new):
            # Insert in chronological order, so that a mutation meets the rule it breaks rather than the ordering guard.
            holders.insert(sum(1 for h in holders if h['attested_on'] <= new['attested_on']), new)

        def added_holder(name, day, sid, cid, role_id=DR_ROLE):
            return lambda p: insert_in_order(role(p, role_id)['holder_claims'], {
                'name': name, 'attested_on': day, 'from': None, 'until': None, 'sources': [sid], 'claim_ids': [cid], 'note': 'x',
                'uncertainty': 'Co-leadership: x'})

        invariant_cases = [
            ("a successor's attestation used as an end (Дмитриев ends when Афанасьев is attested)", lambda p: holder(p, 0).update(until='1991-07-15')),
            ("a later co-chair's list used as an end (Мурашев ends 12 Dec 1991)", lambda p: holder(p, 4).update(until='1991-12-12')),
            ('the letter used as a start (Пономарев from 12 Sep 1991)', lambda p: holder(p, 3).update({'from': '1991-09-12'})),
            ('a congress date used as from without a stated assumption', lambda p: holder(p, 1).update({'from': '1990-10-21', 'attested_on': None})),
            ('the registration day used instead of the date the letter bears', lambda p: holder(p, 2).update(attested_on='1991-10-12')),
            ('the second congress used as an end', lambda p: holder(p, 5).update(until='1991-11-10')),
            ("the Union's end used as an until", lambda p: holder(p, 0, SOYUZ_ROLE).update(until='1991-12-25')),
            ('an acting holder added', added_holder('Исполняющий обязанности сопредседателя', '1991-09-12', 'su_yc_f6_d104_l1_18_19910912', 'su_dr_afanasyev_signs_as_co_chair_19910912')),
            ('co-chairs collapsed into one holder', lambda p: holder(p, 2).update(name='Ю. Афанасьев, Л. Пономарев, А. Мурашев')),
            ('three co-chairs rested on one observation', lambda p: holder(p, 2)['claim_ids'].extend(['su_dr_ponomarev_signs_as_co_chair_19910912', 'su_dr_murashev_signs_as_co_chair_19910912'])),
            ('a Coordinating Council act promoted to a holder (28 Mar 1991)', added_holder('Лев Александрович Пономарев', '1991-03-28', 'su_rsfsr_snd3_sten_v1', 'su_dr_coordinating_council_march_application_19910328')),
            ("the organizing-committee description promoted to a holder (Мурашев)", added_holder('А. Мурашев', '1990-12-17', 'su_snd4_steno_vol1', 'su_snd4_golyakov_murashev_dr_orgcommittee_chair_19901217')),
            ('a council member promoted to a holder (Заславский)', added_holder('Заславский И. И.', '1990-12-19', 'su_snd4_steno_vol1', 'su_snd4_zaslavsky_dr_council_member_19901219')),
            ('the intermediate list promoted to a holder (Пономарев 15 Sep 1991)', added_holder('Лев Александрович Пономарев', '1991-09-15', 'su_yc_f6_d104_l1_18_19910912', 'su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915')),
            ("an opponent's 'лидеры' promoted to a holder (Алкснис 26 Aug 1991)", added_holder('Алкснис В. И.', '1991-08-26', 'su_vs_bulletin2_soyuz_19910826', 'su_vs26_boyars_proposal_names_soyuz_leaders_19910826', SOYUZ_ROLE)),
            ("another deputy's 'руководитель' promoted to a holder (Алкснис 18 Dec 1990)", added_holder('Алкснис В. И.', '1990-12-18', 'su_snd4_steno_vol1', 'su_snd4_bisher_refers_to_soyuz_leader_19901218', SOYUZ_ROLE)),
            ("speaking for the group promoted to a holder (Блохин 13 Mar 1990)", added_holder('Блохин Ю. В.', '1990-03-13', 'su_snd3_steno_vol1', 'su_snd3_blokhin_delivers_soyuz_statement_19900313', SOYUZ_ROLE)),
            ("a member's own statement promoted to a holder (Коган 26 Aug 1991)", added_holder('Коган Е. В.', '1991-08-26', 'su_vs_bulletin2_soyuz_19910826', 'su_vs26_kogan_active_member_of_soyuz_19910826', SOYUZ_ROLE)),
            ('a Soyuz co-chair moved to the movement role (cross-role holder)', lambda p: insert_in_order(role(p)['holder_claims'], copy.deepcopy(holder(p, 0, SOYUZ_ROLE)))),
            ('a movement co-chair on the Soyuz role (cross-role holder)', lambda p: insert_in_order(role(p, SOYUZ_ROLE)['holder_claims'], copy.deepcopy(holder(p, 0)))),
            ('a cross-role claim (a Soyuz claim on the movement role)', lambda p: role(p)['claim_ids'].append('su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220')),
            ('a state-office claim feeding the role (the Supreme Soviet chair)', lambda p: role(p)['claim_ids'].append('su_lukyanov_signs_presidium_2353i_2354i_19910822')),
            ('a movement claim feeding a state office (the Presidency)', lambda p: role(p, 'su_president')['claim_ids'].append('su_dr_afanasyev_co_chair_statement_19910715')),
            ('a movement co-chair added to the CPSU leadership', lambda p: role(p, 'su_cpsu_general_secretary')['holder_claims'].append(copy.deepcopy(holder(p, 1)))),
            ('a body claim moved onto the role (the Council of Representatives plenum)', lambda p: role(p)['claim_ids'].append('su_dr_representatives_council_plenum_19910915')),
            ('an existing USSR holder changed', lambda p: role(p, 'su_government_head')['holder_claims'][0].update(attested_on='1990-01-13')),
            ('an until on any holder', lambda p: holder(p, 6).update(until='1991-12-24')),
            ('a holder dated by a claim of another day', lambda p: holder(p, 0).update(claim_ids=['su_dr_afanasyev_co_chair_statement_19910715'], sources=['su_rsfsr_snd5_sten_v1'])),
            ('a lifecycle start taken from the bloc appeal', lambda p: org(p)['lifecycle'].update({'from': '1990-06-22'})),
            ('a lifecycle end taken from the latest record', lambda p: org(p, SOYUZ)['lifecycle'].update(until='1991-09-03')),
            ('a successor mapping to Russia', lambda p: org(p)['jurisdiction'].update(automatic_successor_mapping=True)),
            ('the movement filed as union-level', lambda p: org(p)['jurisdiction'].update(level='union')),
            ('the role removed', lambda p: org(p)['roles'].pop()),
            ('an organization removed', lambda p: p['organizations'].pop()),
            ('the role given a state-office kind', lambda p: role(p).update(kind='head_of_state')),
            ('the undated invitation given a date', lambda p: claim(p, 'su_dr_second_congress_invitation_undated').update(attested_on='1991-11-09')),
            ('events collapsed out of order', lambda p: claim(p, 'su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915').update(attested_on='1991-09-11')),
            ('a claim before the period', lambda p: claim(p, 'su_snd3_deputy_speaks_for_soyuz_19900312').update(attested_on='1989-12-31')),
        ]
        c35_invariants(self.packet, self.russia)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, StopIteration)):
                c35_invariants(mutated(change), self.russia)
        # A Russia role citing a claim of this packet fails too.
        russia = copy.deepcopy(self.russia)
        next(r for e in russia['institutions'] for r in e['roles'] if r['id'] == 'ru_government_chairman')['claim_ids'].append('su_dr_yakunin_listed_as_co_chair_19911212')
        with self.assertRaises(AssertionError):
            c35_invariants(self.packet, russia)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for number, decision in DECISIONS.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| {number} ')]
            self.assertIn(f'**{decision}:**', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('research-index.json', 'test_ussr_research_s10h.py', 'test_ussr_government_supreme_soviet_c01_26.py',
                     'campaign_census.py', '262d5f61', 'test_certified_gap_ledger.py', 'test_certified_boundary_matrix.py',
                     'COMMIT_PACKETS', 'boundary-matrix', 'not stacked'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'ussr-democratic-russia-soyuz-1990-1991-35.md', 'claude/c01-su-35', 'c1475ada', '44098c5a',
                     'test_ussr_democratic_russia_soyuz_c01_35.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'USSR')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['organization_observations'], country['institution_observations'], country['role_observations'],
                          country['source_claims'], country['mapping_pending']), (3, 4, 8, 174, 7))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'USSR'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
