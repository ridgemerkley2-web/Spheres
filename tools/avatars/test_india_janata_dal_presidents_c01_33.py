"""CLAUDE-C01-33: the Presidents of the Janata Dal, 1990-2026, are one party role on a new Janata Dal recognition
observation (row 6 of the Election Commission's national-party table of 10 January 1998), kept apart from the
prime-ministership, the presidency and the Congress and BJP presidencies both ways. In-office attestations, continuations,
a working presidency, statements that do not name the party (leads), rival claims, a removal claimed, recollections, spans
and the Commission's statement of its records stay separate claims; the 1993 and 1999 disputes and the Janata Dal (A),
(Secular) and (United) groups are claims about the organisation, never merged identities or inherited leaders. No source
states a day of assumption or end, so every holder is a dated in-office observation."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import test_india_bjp_presidents_c01_27 as c01_27
import test_india_inc_presidents_c01_20 as c01_20
import test_india_presidents_c01_15 as c01_15
import test_india_prime_ministers_c01_11 as c01_11

ORG = 'in_eci_19980110_np_06'
ROLE = 'in_jd_president'
TITLE = 'President of the Janata Dal'
REVIEW = [f'JD-PRES-{n:02d}' for n in range(1, 9)]
VPS, SRB, LPY, SY, HDG, AJS = ('Vishwanath Pratap Singh', 'S. R. Bommai', 'Laloo Prasad Yadav', 'Sharad Yadav', 'H. D. Deve Gowda',
                               'Ajit Singh')
# The surname that every claim naming that person must carry in its text; at most ten people in the packet.
SURNAMES = {VPS: 'Singh', SRB: 'Bommai', LPY: 'Laloo', SY: 'Yadav', HDG: 'Deve Gowda', AJS: 'Ajit Singh'}
HOLDER_NAMES = {SRB, LPY}

# Original response identity recorded in each extract: (bytes, sha256), in packet order. Every source is reproducible.
RESPONSES = {
    'in_rs_debate_19891228_motion_of_thanks':
        (3544025, '3fdddf91d8e022da393d663f2061e738227ad6fb48a71103d46c8b1bc61644c2'),
    'in_rs_debate_19900518_meham_countermanding':
        (815707, '6dd6c4707d9adb96ed96251d70c9c4ab32f46d8971aac9e1c916f1da11774e50'),
    'in_rs_written_answers_19900828_usq2406_pm_letters':
        (105687, '94845f4a2e27e15432cdb79e9a69dfe532554ef0a584254407725ec0f8840788'),
    'in_gazette_welfare_resolution_19910329':
        (663246, 'db48f4f7c1d139680cb394cdd8cf0df75ac3c519bd97f171f9b65a4440783fd5'),
    'in_ls_written_answers_19910829_usq5063_nic':
        (312005, 'ee5547287325ffcad6b34ec2acab1e78ff4fe162050879f93859780e2077ddf1'),
    'in_gazette_welfare_resolution_19920214':
        (424598, '59b93fa4f3702717f3246ca28b74ba7c61d6cbfe4211f7d5b53a4d094edb5f79'),
    'in_eci_on8e_19930117_janata_dal_dispute':
        (104268, '1d0a0b07556664d8c2cdca5cd2024d1374a8b269949f8900c6014fd7d45a7b32'),
    'in_gazette_ls_speaker_decision_19930601':
        (2879963, 'd0ceaea1a826bd8e4f0808ca458be7eaf6d866c9941ad176a03d833e8698d59f'),
    'in_goa_gazette_eci_notification_56_93_6_19930909':
        (549444, '27990864cd1bf4f60d8df443c297ffa8b7f8763a7826275fb22e72e0888e463a'),
    'in_eci_on11e_19960205_national_parties':
        (2694875, '0189f4e8e71fde7fcd11bdc1f2a86818e79d7d804542405e511a4c27f83677c8'),
    'in_ls_debate_19960715_petroleum_prices':
        (13860933, 'ca995bb48dc8b52a998694c8454ea1850afbd4f80162f324203b23985fa8bf59'),
    'in_gazette_hrd_resolution_19961014':
        (435535, '11eab2c61a70181f0120f195210c8e33c3d2c81f936018fc1de6764e16aeb07f'),
    'in_ls_debate_19970317_environment_appellate_authority':
        (8195221, 'f8ce7f3ea2f80086fc8bf4a7ff5a9a46e8fba0d7417b80331ffde6c0f09a664c'),
    'in_ls_debate_19970422_confidence_motion':
        (937125, '1cb7dcbb11e90f68af7cf8109ef4459e9483963676f627b22a2a170e97c3f160'),
    'in_ls_debate_19970729_bihar_situation':
        (30276934, '735dc385eb052fcb14cafb895da33eddd50d48bddf52995b968565e01898be65'),
    'in_rs_debate_19970805_bihar_situation':
        (449285, '19d3c3f03f9fba7ba4753b7c52889ca9ddeff5f1143b0fbfccda237a3e279192'),
    'in_rs_debate_19970826_human_development_discussion':
        (753976, '8ff848856e1583368e978ab29200306c7a0ffc38dd6e8540a00d4b9fdb5e4fe2'),
    'in_gazette_hrd_resolution_19970929':
        (186543, 'eac780ae235103cfb1c2e917b7b166cf21827ffbb731e182eafc71399404ec6e'),
    'in_eci_on18e_19980110_national_parties':
        (4063921, '2e68126f22e1540315405b8b5ee978c35e6bcc4a7f2f23629c236b2d2e5b7744'),
    'in_eci_dispute_case_1_of_1999_order_19990807':
        (28440, '2fa846930f971347601f0f0719954f2d49bc717ad5d54e68e6349fb92e1cb404'),
    'in_eci_on28e_19990809_janata_dal_dispute':
        (107430, '2c2f57244d870630f325d3e3ec287e76209cc4b0f798c495e64b63f234150164'),
    'in_rs_synopsis_20071115_obituary_bommai':
        (103683, '712056ec84df5a2623fa2b46b4f6c8293139ca83ff2411055e67a25decd3e4c7'),
}
NEW_SOURCES = list(RESPONSES)
# Files on the Rajya Sabha Secretariat's debates store, with their handles.
STORE = {'in_rs_debate_19891228_motion_of_thanks': 'ID_152_28121989_01_p202-356_1.pdf',
         'in_rs_debate_19900518_meham_countermanding': 'ID_154_18051990_01_p170-242_1.pdf',
         'in_rs_written_answers_19900828_usq2406_pm_letters': 'IQ_155_28081990_U2406_p181-183.pdf',
         'in_rs_debate_19970805_bihar_situation': 'ID_181_05081997_01_p239-310_1.pdf',
         'in_rs_debate_19970826_human_development_discussion': 'ID_181_26081997_01_p72-220_1.pdf'}
# Official eGazette files, each equal to the stored file of an Internet Archive Gazette of India item.
EGAZETTE = {'in_gazette_welfare_resolution_19910329': 'E-0528-1991-0076-21565',
            'in_gazette_welfare_resolution_19920214': 'E-0491-1992-0033-19183',
            'in_eci_on8e_19930117_janata_dal_dispute': 'E-0459-1993-0000-17812',
            'in_gazette_ls_speaker_decision_19930601': 'E-0460-1993-0320-18126',
            'in_eci_on11e_19960205_national_parties': 'E-0312-1996-0004-11878',
            'in_gazette_hrd_resolution_19961014': 'E-0298-1996-0190-10568',
            'in_gazette_hrd_resolution_19970929': 'E-0243-1997-0194-8273'}
# Stored files in Internet Archive items (archive.org/download/<item>/<file>): Gazette copies and Parliament Digital
# Library copies; mirror-only provenance, recorded as such.
IA_ITEMS = {'in_ls_written_answers_19910829_usq5063_nic': 'eparlib.nic.in.17679',
            'in_goa_gazette_eci_notification_56_93_6_19930909': 'in.goa.egaz.9394-24.SI',
            'in_ls_debate_19960715_petroleum_prices': 'eparlib.nic.in.10951',
            'in_ls_debate_19970317_environment_appellate_authority': 'eparlib.nic.in.10363',
            'in_ls_debate_19970729_bihar_situation': 'eparlib.nic.in.8795',
            'in_eci_on18e_19980110_national_parties': 'in.gazette.central.e.1998-01-15.7285',
            'in_eci_on28e_19990809_janata_dal_dispute': 'in.gazette.central.e.1999-08-09.4826'}
# The one raw Internet Archive capture (id_ form) of an official file, with its capture timestamp; before the cutoff.
WAYBACK = {'in_ls_debate_19970422_confidence_motion':
           ('20211202091920', 'https://eparlib.nic.in/bitstream/123456789/6598/1/11_IV_22041997_p9_p23_t14.pdf')}
IFES = {'in_eci_dispute_case_1_of_1999_order_19990807': '16008740057920ye3h4iaksq.pdf'}
CMS = {'in_rs_synopsis_20071115_obituary_bommai': '/UploadedFiles/Synopsis/SynopsisUpload/212/15112007.pdf'}
# PDF pages rendered or read (one-based), per source.
PDF_PAGES = {
    'in_rs_debate_19891228_motion_of_thanks': [10],
    'in_rs_debate_19900518_meham_countermanding': [18, 19],
    'in_rs_written_answers_19900828_usq2406_pm_letters': [1, 2],
    'in_gazette_welfare_resolution_19910329': [1, 9, 10],
    'in_ls_written_answers_19910829_usq5063_nic': [1],
    'in_gazette_welfare_resolution_19920214': [6, 9],
    'in_eci_on8e_19930117_janata_dal_dispute': [1, 2],
    'in_gazette_ls_speaker_decision_19930601': [21, 23, 33],
    'in_goa_gazette_eci_notification_56_93_6_19930909': [1, 7, 8],
    'in_eci_on11e_19960205_national_parties': [34],
    'in_ls_debate_19960715_petroleum_prices': [11],
    'in_gazette_hrd_resolution_19961014': [5, 7],
    'in_ls_debate_19970317_environment_appellate_authority': [5],
    'in_ls_debate_19970422_confidence_motion': [6],
    'in_ls_debate_19970729_bihar_situation': [23, 24],
    'in_rs_debate_19970805_bihar_situation': [5, 24],
    'in_rs_debate_19970826_human_development_discussion': [40, 41],
    'in_gazette_hrd_resolution_19970929': [4, 5],
    'in_eci_on18e_19980110_national_parties': [82, 83],
    'in_eci_dispute_case_1_of_1999_order_19990807': [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    'in_eci_on28e_19990809_janata_dal_dispute': [1, 2, 3],
    'in_rs_synopsis_20071115_obituary_bommai': [5],
}
# Every new claim's (attested_on, event_kind, review observation, holder_name, role_id), exactly: distinct dated events
# are never re-dated, relabelled, moved to another observation, given another holder or moved between the organisation
# (role_id null) and the party role.
EVENTS = {
    'in_rs_anand_sharma_vp_singh_president_of_janata_dal_19891228':
        ('1989-12-28', 'in_office_before_period', 'JD-PRES-01', VPS, ROLE),
    'in_rs_raj_mohan_gandhi_pm_wrote_as_janata_dal_president_recalled_1990':
        (None, 'unnamed_recollection', 'JD-PRES-01', None, ROLE),
    'in_rs_pm_letter_to_bommai_president_janata_dal_19900714':
        ('1990-07-14', 'in_office_attestation', 'JD-PRES-02', SRB, ROLE),
    'in_rs_pm_answer_styles_bommai_president_janata_dal_19900828':
        ('1990-08-28', 'in_office_continuation_attestation', 'JD-PRES-02', SRB, ROLE),
    'in_gazette_welfare_resolution_bommai_president_janata_dal_19910329':
        ('1991-03-29', 'in_office_continuation_attestation', 'JD-PRES-02', SRB, ROLE),
    'in_ls_nic_statement_lists_bommai_president_janata_dal_1990_composition':
        (None, 'stale_composition_list', 'JD-PRES-02', SRB, ROLE),
    'in_gazette_welfare_resolution_bommai_president_janata_dal_19920214':
        ('1992-02-14', 'in_office_continuation_attestation', 'JD-PRES-03', SRB, ROLE),
    'in_eci_janata_dal_symbol_and_name_frozen_order_19930114':
        ('1993-01-14', 'dispute_name_and_symbol_frozen', 'JD-PRES-03', None, None),
    'in_eci_janata_dal_two_groups_interim_recognition_order_19930114':
        ('1993-01-14', 'interim_split_recognition', 'JD-PRES-03', None, None),
    'in_eci_janata_dal_a_and_b_named_further_order_199301':
        (None, 'interim_split_recognition', 'JD-PRES-03', None, None),
    'in_eci_on8e_table_entry_janata_dal_dispute_pending_19930117':
        ('1993-01-17', 'recognition_row_under_dispute', 'JD-PRES-03', None, None),
    'in_ls_speaker_decision_ajit_singh_expelled_by_bommai_recalled_19911226':
        ('1991-12-26', 'recalled_act_dated', 'JD-PRES-03', SRB, ROLE),
    'in_ls_speaker_decision_four_members_expelled_by_bommai_recalled_19920719':
        ('1992-07-19', 'recalled_act_dated', 'JD-PRES-03', SRB, ROLE),
    'in_ls_speaker_decision_ajit_singh_endorsed_as_president_claimed_19920205':
        ('1992-02-05', 'rival_presidency_claim', 'JD-PRES-03', AJS, ROLE),
    'in_eci_final_order_bommai_group_recognised_as_janata_dal_19930722':
        ('1993-07-22', 'dispute_final_decision', 'JD-PRES-03', None, None),
    'in_eci_final_order_janata_dal_a_interim_recognition_withdrawn_19930722':
        ('1993-07-22', 'interim_recognition_withdrawn', 'JD-PRES-03', None, None),
    'in_eci_notification_56_93_6_table_entry_janata_dal_19930723':
        ('1993-07-23', 'recognition_row', 'JD-PRES-07', None, None),
    'in_eci_19960205_np_row_05_janta_dal':
        ('1996-02-05', 'recognition_row', 'JD-PRES-07', None, None),
    'in_ls_panigrahi_laloo_prasad_yadav_party_president_19960715':
        ('1996-07-15', 'in_office_attestation', 'JD-PRES-04', LPY, ROLE),
    'in_gazette_hrd_resolution_laloo_under_presidents_of_major_parties_19961014':
        ('1996-10-14', 'in_office_continuation_attestation', 'JD-PRES-04', LPY, ROLE),
    'in_ls_ram_naik_sharad_yadav_working_president_19970317':
        ('1997-03-17', 'working_presidency', 'JD-PRES-05', SY, ROLE),
    'in_ls_sushma_swaraj_your_president_laloo_prasad_yadav_19970422':
        ('1997-04-22', 'in_office_continuation_attestation', 'JD-PRES-04', LPY, ROLE),
    'in_ls_ram_naik_party_president_sharad_yadav_19970729':
        ('1997-07-29', 'party_not_named_lead', 'JD-PRES-05', SY, ROLE),
    'in_ls_adsul_democratically_elected_president_sharad_yadav_19970729':
        ('1997-07-29', 'party_not_named_lead', 'JD-PRES-05', SY, ROLE),
    'in_ls_virendra_kumar_singh_laloo_rjd_party_president_19970729':
        ('1997-07-29', 'other_party_office_reference', 'JD-PRES-04', LPY, ROLE),
    'in_rs_joyanta_roy_laloo_as_president_of_janata_dal_cmp_recalled':
        (None, 'recalled_act_undated', 'JD-PRES-04', LPY, ROLE),
    'in_rs_som_pal_president_of_jd_charge_framed_19970805':
        ('1997-08-05', 'unnamed_holder_attestation', 'JD-PRES-05', None, ROLE),
    'in_rs_bommai_as_janata_dal_president_report_1989_recalled':
        (None, 'retrospective_statement', 'JD-PRES-02', SRB, ROLE),
    'in_gazette_hrd_resolution_sharad_yadav_under_presidents_of_major_parties_19970929':
        ('1997-09-29', 'in_office_continuation_attestation', 'JD-PRES-05', SY, ROLE),
    'in_eci_19980110_np_row_06':
        ('1998-01-10', 'recognition_row', 'JD-PRES-07', None, None),
    'in_eci_dispute_1999_janata_dal_recognised_national_party_chakra_19990807':
        ('1999-08-07', 'recognition_statement', 'JD-PRES-06', None, None),
    'in_eci_dispute_1999_commission_records_sharad_yadav_president_19990807':
        ('1999-08-07', 'registration_record_statement', 'JD-PRES-06', SY, ROLE),
    'in_eci_dispute_1999_deve_gowda_application_as_president_19990722':
        ('1999-07-22', 'rival_presidency_claim', 'JD-PRES-06', HDG, ROLE),
    'in_eci_dispute_1999_pac_removal_of_sharad_yadav_claimed_19990721':
        ('1999-07-21', 'removal_claimed', 'JD-PRES-06', SY, ROLE),
    'in_eci_dispute_1999_pac_elected_deve_gowda_president_claimed':
        (None, 'rival_presidency_claim', 'JD-PRES-06', HDG, ROLE),
    'in_eci_dispute_1999_national_executive_endorsement_claimed_19990729':
        ('1999-07-29', 'endorsement_claimed', 'JD-PRES-06', SY, ROLE),
    'in_eci_dispute_1999_ad_hoc_recognition_of_both_groups_19990807':
        ('1999-08-07', 'interim_split_recognition', 'JD-PRES-06', None, None),
    'in_eci_dispute_1999_compiler_note_jdu_and_jds_named':
        ('1999-08-07', 'successor_group_recognition', 'JD-PRES-08', None, None),
    'in_eci_on28e_janata_dal_name_and_symbol_under_dispute_19990809':
        ('1999-08-09', 'recognition_row_under_dispute', 'JD-PRES-08', None, None),
    'in_eci_on28e_janata_dal_secular_and_united_rows_19990809':
        ('1999-08-09', 'successor_group_recognition', 'JD-PRES-08', None, None),
    'in_rs_obituary_bommai_president_all_india_janta_dal_1990_to_1996':
        (None, 'retrospective_span', 'JD-PRES-04', SRB, ROLE),
}
ORG_CLAIMS = tuple(cid for cid, e in EVENTS.items() if e[4] is None)
ROLE_CLAIMS = tuple(cid for cid, e in EVENTS.items() if e[4] == ROLE)
ORG_KINDS = {'dispute_name_and_symbol_frozen', 'interim_split_recognition', 'recognition_row_under_dispute',
             'dispute_final_decision', 'interim_recognition_withdrawn', 'recognition_row', 'recognition_statement',
             'successor_group_recognition'}
HOLDER_KINDS = {'in_office_attestation'}
FROM_KINDS = {'assumption_statement'}
UNTIL_KINDS = {'end_of_office_stated'}
# Role claims that never feed a holder, by kind.
CONTINUATION_KINDS = {'in_office_continuation_attestation'}
BEFORE_PERIOD_KINDS = {'in_office_before_period'}
UNNAMED_KINDS = {'unnamed_recollection', 'unnamed_holder_attestation'}
RETROSPECTIVE_KINDS = {'recalled_act_dated', 'recalled_act_undated', 'retrospective_statement', 'retrospective_span',
                       'stale_composition_list'}
DISPUTE_KINDS = {'rival_presidency_claim', 'removal_claimed', 'endorsement_claimed', 'registration_record_statement'}
WORKING_KINDS = {'working_presidency'}
OTHER_PARTY_KINDS = {'other_party_office_reference'}
# Statements that do not name the party (it is read only from context), by speakers not identified in the record as
# Janata Dal officers: leads that never feed a holder (Sharad Yadav, 29 July 1997).
LEAD_KINDS = {'party_not_named_lead'}
NEVER_KINDS = (CONTINUATION_KINDS | BEFORE_PERIOD_KINDS | UNNAMED_KINDS | RETROSPECTIVE_KINDS | DISPUTE_KINDS
               | WORKING_KINDS | OTHER_PARTY_KINDS | LEAD_KINDS)
HOLDERS = [
    (SRB, '1990-07-14', None, None),
    (LPY, '1996-07-15', None, None),
]
HOLDER_CLAIMS = [
    ['in_rs_pm_letter_to_bommai_president_janata_dal_19900714'],
    ['in_ls_panigrahi_laloo_prasad_yadav_party_president_19960715'],
]
HOLDER_REVIEW = ['JD-PRES-02', 'JD-PRES-04']
# The leads of 29 July 1997 that once made Sharad Yadav a holder; restoring that holder must fail.
SY_LEADS = ['in_ls_ram_naik_party_president_sharad_yadav_19970729',
            'in_ls_adsul_democratically_elected_president_sharad_yadav_19970729']
HOLDER_OBSERVATIONS = tuple(cid for cid, e in EVENTS.items() if e[1] in HOLDER_KINDS)
NEVER_HOLDER = tuple(cid for cid in ROLE_CLAIMS if cid not in HOLDER_OBSERVATIONS)
# No source states the day any President assumed or left the office.
STARTS, ENDS = [], []
# The working presidency, the rival claims and the leads that do not name the party are never holders; V. P. Singh is
# attested only before the period.
WORKING = {SY: ('1997-03-17', '1997-03-17')}
NOT_HOLDERS = {VPS, HDG, AJS, SY}
HOLDER_DATES = {d for h in HOLDERS for d in h[1:] if d}
# Dates that are never any holder's attested_on, start or end: continuations, recollections, rival claims, the removal
# claimed, the endorsement claimed, the Commission's statement of its records, the working presidency, the leads that do
# not name the party, the attestation before the period, the Rashtriya Janata Dal presidency and every organisation event;
# and rejected candidate days (the printed header '[13 MAY 1990]', the Bihar Assembly statement of 23 July 1996 and the
# claimed election of 3 July 1997).
NEVER_HOLDER_DATE = sorted(({e[0] for cid, e in EVENTS.items() if cid not in HOLDER_OBSERVATIONS and e[0]} - HOLDER_DATES)
                           | {'1990-05-13', '1996-07-23', '1997-07-03'})
UNDATED = tuple(cid for cid, e in EVENTS.items() if e[0] is None)
NAMELESS = tuple(cid for cid, e in EVENTS.items() if e[3] is None)
# Leads, declined records and secondary sources: never a source URL.
LEAD_URL_MARKERS = ('wikipedia', 'sikhheritageeducation', 'business-standard', 'rsdebate.nic.in', 'ID_155_04101990',
                    'ID_178_15071996', 'E-0297-1996-0220-10496', 'E-0492-1992-0085-19235', 'eparlib.nic.in.5888',
                    'eparlib.nic.in.9786', 'eparlib.nic.in.3579', 'eparlib.nic.in.3670', 'eparlib.nic.in.6036',
                    'eparlib.nic.in.56238', 'in.gazette.central.e.2001-02-15', 'csl_extraordinary', 'eci.gov.in',
                    'eparlib.sansad.in/bitstream', 'archive.org/download/eparlib.nic.in.6598')
# URL patterns of responses generated per request, growing listings, searches, cache-busting queries or session state.
PER_REQUEST = re.compile(r'[?&](cb|_|s|q|token|sessionid|search|checkcb|searchTerm)=|X-Amz-|Signature=|cdx/search|'
                         r'/search|advancedsearch|page_production|nocache|__VIEWSTATE|ASPSESSION|fetch/all', re.I)
REPORT = research.RESEARCH / 'india-janata-dal-presidents-1990-2026-33.md'
# CLAUDE-C01-40 adds one party role, in_cpm_general_secretary, to the existing Communist Party of India (Marxist)
# recognition observation and appends its sources after this packet's, with this many sources and claims; its own test
# pins them. This packet's assertions are unchanged for its own records.
C01_40_ORG = 'in_eci_20240323_np_04'
C01_40_ROLE = 'in_cpm_general_secretary'
C01_40_COUNTS = (20, 26)
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-33.md'


def jd_org(packet):
    org, = [o for o in packet['organizations'] if o['id'] == ORG]
    return org


def jd_role(packet):
    return jd_org(packet)['roles'][0]


def other_role(packet, org_id):
    org, = [o for o in packet['organizations'] if o['id'] == org_id]
    return org['roles'][0]


def load_rows(packet):
    rows = {}
    for source in packet['sources']:
        if source['id'] in RESPONSES:
            extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
            rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def jd_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    orgs = [o for o in packet['organizations'] if o['id'] == ORG]
    assert len(orgs) == 1
    org = orgs[0]
    # The new observation is a recognition row only: no lifespan, no successor and no game mapping.
    assert (org['name'], org['kind'], org['jurisdiction']) == ('Janata Dal',
                                                               'political_party_national_recognition_observation', 'India')
    assert org['source_identifier'] == {'kind': 'notification_recognition_row', 'value': ORG,
                                        'note': 'Local research locator for this notification and jurisdiction, not an '
                                                'official legal party identifier.'}
    assert org['recognition'] == {'level': 'national', 'attested_on': '1998-01-10', 'from': None, 'until': None,
                                  'source_qualifications': [], 'consolidated_current_status': None}
    assert (org['lifecycle']['status'], org['lifecycle']['from'], org['lifecycle']['until']) == ('unresearched', None, None)
    assert org['represented_party_ids'] == [] and org['reconciled_organization_id'] is None
    assert org['coverage']['status'] == 'reporting_identity_only'
    assert packet['organizations'][-1] is org, 'the new observation is appended after the 2024 rows'
    assert [r['id'] for r in org['roles']] == [ROLE], 'exactly one role on the Janata Dal observation'
    role = org['roles'][0]
    assert (role['title'], role['kind']) == (TITLE, 'party_leader')
    leaders = [(e['id'], r['id']) for e in packet['organizations'] + packet['institutions'] for r in e['roles']
               if r['kind'] == 'party_leader' or r['id'] == ROLE]
    # CLAUDE-C01-40's party role on the Communist Party of India (Marxist) observation, added after this packet, sits
    # between the BJP and Congress roles in observation order.
    assert leaders == [(c01_27.ORG, c01_27.ROLE), (C01_40_ORG, C01_40_ROLE), (c01_20.ORG, c01_20.ROLE), (ORG, ROLE)], \
        'the four party-leader roles only, and no copy of this one'
    assert set(role['claim_ids']) <= set(org['claim_ids']) and set(role['sources']) <= set(org['sources'])
    # Organisation claims never sit on the role, and role claims always do.
    assert not set(role['claim_ids']) & set(ORG_CLAIMS), 'an organisation claim moved onto the role'
    for cid in org['claim_ids']:
        assert (rows[cid]['role_id'] is None) == (cid not in role['claim_ids']), cid
        assert rows[cid]['observation_id'] == ORG, cid
        if rows[cid]['role_id'] is None:
            assert rows[cid]['holder_name'] is None and rows[cid]['role_title'] is None, cid
            assert rows[cid]['event_kind'] in ORG_KINDS, cid
    # Party office and state office never feed each other, and the Congress and BJP roles share nothing with this one.
    jd_claims = set(org['claim_ids']) | {c for h in role['holder_claims'] for c in h['claim_ids']}
    jd_sources = set(org['sources']) | {s for h in role['holder_claims'] for s in h['sources']}
    # This role's holders hold no other role or institution here. (V. P. Singh and H. D. Deve Gowda, named in claims of
    # this role, are Prime Minister holders of in_prime_minister; the claims and sources stay disjoint both ways.)
    names = HOLDER_NAMES
    for inst in packet['institutions']:
        i_claims = set(inst['claim_ids']) | {c for r in inst['roles'] for c in r['claim_ids']} | {
            c for r in inst['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
        i_sources = set(inst['sources']) | {s for r in inst['roles'] for s in r['sources']} | {
            s for r in inst['roles'] for h in r['holder_claims'] for s in h['sources']}
        assert not jd_claims & i_claims and not jd_sources & i_sources, inst['id']
        for r in inst['roles']:
            assert not {h['name'] for h in r['holder_claims']} & names, (inst['id'], 'cross-institution holder')
    for entry in packet['organizations']:
        if entry is not org:
            assert not set(entry['claim_ids']) & jd_claims and not set(entry['sources']) & jd_sources, entry['id']
            for r in entry['roles']:
                assert not set(r['claim_ids']) & jd_claims and not set(r['sources']) & jd_sources, r['id']
                assert not {h['name'] for h in r['holder_claims']} & names, (r['id'], 'cross-party holder')
    previous = ''
    for holder in role['holder_claims']:
        name = holder['name']
        assert isinstance(holder, dict) and name in HOLDER_NAMES, name
        assert name not in NOT_HOLDERS, (name, 'a working presidency, a rival claim, a lead or a pre-period attestation')
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (name, 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] >= previous, (name, 'holders stay in chronological order')
        assert '1990-01-01' <= dated[0] <= research.CUTOFF, (name, 'inside the period')
        previous = dated[0]
        assert not {holder['attested_on'], holder['from'], holder['until']} & set(NEVER_HOLDER_DATE), name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        assert not set(holder['claim_ids']) & set(ORG_CLAIMS), name
        if name in WORKING:
            assert not WORKING[name][0] <= dated[0] <= WORKING[name][1], (name, 'a working presidency is never a holder')
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
        # Each observation cites only in-office attestations of its own day.
        assert attest_days == ({holder['attested_on']} if holder['attested_on'] else set()), (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and dated[0] <= holder['until']
    # Claims that never feed a holder stay on the role with their own kinds; undated claims carry no structured date.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] in NEVER_KINDS, cid
    for cid in UNDATED:
        assert not {'attested_on', 'period', 'attested_period'} & set(claims[cid]), cid
    for cid in org['claim_ids']:
        assert 'period' not in claims[cid] and 'attested_period' not in claims[cid], cid
        assert rows[cid]['review_observation'] in REVIEW, cid


def jd_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    jd_rules(packet, rows)
    role = jd_role(packet)
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    assert role['claim_ids'] == list(ROLE_CLAIMS)
    assert jd_org(packet)['claim_ids'] == list(EVENTS)
    # Distinct dated events stay distinct and keep their own days.
    for cid, (day, _kind, _obs, _name, _role) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # The stacked prime-minister, president, Congress-president and BJP-president holders are unchanged.
    pm, presidency = packet['institutions']
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in pm['roles'][0]['holder_claims']] == c01_11.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until'])
            for h in presidency['roles'][0]['holder_claims']] == c01_15.HOLDERS
    inc = other_role(packet, c01_20.ORG)
    bjp = other_role(packet, c01_27.ORG)
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in inc['holder_claims']] == c01_20.HOLDERS
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in bjp['holder_claims']] == c01_27.HOLDERS
    assert inc['claim_ids'] == list(c01_20.EVENTS) and bjp['claim_ids'] == list(c01_27.EVENTS)


class IndiaJanataDalPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'india.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.org = jd_org(cls.packet)
        cls.role = cls.org['roles'][0]
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (22, 41))
        pm, presidency = self.packet['institutions']
        inc, bjp = other_role(self.packet, c01_20.ORG), other_role(self.packet, c01_27.ORG)
        cpm = other_role(self.packet, C01_40_ORG)
        self.assertEqual((cpm['id'], len(cpm['sources']), len(cpm['claim_ids'])), (C01_40_ROLE,) + C01_40_COUNTS)
        self.assertEqual([s['id'] for s in self.packet['sources']],
                         list(c01_27.ORIGINAL_SOURCES) + pm['sources'] + presidency['sources'] + inc['sources']
                         + bjp['sources'] + NEW_SOURCES + cpm['sources'])
        self.assertEqual((pm['sources'], presidency['sources'], inc['sources'], bjp['sources']),
                         (c01_11.NEW_SOURCES, c01_15.NEW_SOURCES, c01_20.NEW_SOURCES, c01_27.NEW_SOURCES))
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (85, 6))
        self.assertEqual(len(self.packet['organizations']), 83)
        # Every new claim is an organisation claim, a holder claim or a role claim that never feeds a holder; never two.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertFalse((set(holder_claims) | set(NEVER_HOLDER)) & set(ORG_CLAIMS))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER) | set(ORG_CLAIMS), set(self.new_claims))
        self.assertEqual(sorted(holder_claims), sorted(HOLDER_OBSERVATIONS))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER), len(ORG_CLAIMS), len(HOLDERS)), (2, 25, 14, 2))
        self.assertEqual(list(EVENTS), self.new_claims)
        # Every new claim and source is cited by the Janata Dal observation (and, for role claims, its role) only.
        self.assertEqual(self.org['claim_ids'], self.new_claims)
        self.assertEqual(self.org['sources'], NEW_SOURCES)
        self.assertEqual(self.role['claim_ids'], list(ROLE_CLAIMS))
        self.assertEqual(self.role['sources'], [sid for sid in NEW_SOURCES
                                                if any(EVENTS[c['id']][4] for c in self.sources[sid]['claims'])])
        self.assertEqual(len(self.role['sources']), 17)
        for entry in self.packet['organizations'] + self.packet['institutions']:
            if entry is not self.org:
                self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
                self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
                for role in entry['roles']:
                    self.assertFalse(set(self.new_claims) & set(role['claim_ids']), role['id'])
        # At most ten observations, JD-PRES-01..08, each reported and carrying rows; no row reuses another packet's numbers.
        observations = re.findall(r'^### (JD-PRES-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({self.rows[cid]['review_observation'] for cid in self.new_claims}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], HOLDER_REVIEW)
        for cid in self.new_claims:
            self.assertNotRegex(self.rows[cid]['review_observation'], r'^(IN-(PRES|PM)|INC-PRES|BJP-PRES)-', cid)
        kinds = {self.rows[cid]['event_kind'] for cid in self.new_claims}
        self.assertEqual(kinds, HOLDER_KINDS | NEVER_KINDS | ORG_KINDS)
        self.assertFalse(set(self.sources) & set(self.claims), 'no id is both a source and a claim')
        # At most ten people are named as holder_name, and they are exactly the six this packet researched.
        self.assertEqual({self.rows[cid]['holder_name'] for cid in self.new_claims} - {None}, set(SURNAMES))
        self.assertLessEqual(len(SURNAMES), 10)

    def test_holders_are_exactly_as_intended(self):
        jd_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            self.assertIn('No start', holder['uncertainty'], holder['name'])
            self.assertIn('No end', holder['uncertainty'], holder['name'])
            for cid in holder['claim_ids']:
                self.assertIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        for cid in NEVER_HOLDER + ORG_CLAIMS:
            self.assertNotIn('Dates the holder', self.claims[cid]['uncertainty'], cid)
        for cid in self.new_claims:
            row = self.rows[cid]
            self.assertEqual((row['observation_id'], row['role_id']), (ORG, EVENTS[cid][4]), cid)
            self.assertEqual(row['role_title'], TITLE if EVENTS[cid][4] else None, cid)
            self.assertEqual(row['holder_name'], EVENTS[cid][3], cid)
            if row['holder_name']:
                self.assertIn(SURNAMES[row['holder_name']], self.claims[cid]['text'], cid)
        # Rows whose source names nobody, and every organisation row, have holder_name null and never feed a holder.
        self.assertEqual(len(NAMELESS), 16)
        for cid in NAMELESS:
            self.assertIn(self.rows[cid]['event_kind'], NEVER_KINDS | ORG_KINDS, cid)
        # The printed forms stay in the claim texts.
        printed = {
            'in_eci_19960205_np_row_05_janta_dal': "'Janta Dal' (so printed)",
            'in_rs_obituary_bommai_president_all_india_janta_dal_1990_to_1996': 'All India Janta Dal',
            'in_ls_ram_naik_sharad_yadav_working_president_19970317': 'working President of Janta Dal',
            'in_rs_anand_sharma_vp_singh_president_of_janata_dal_19891228': "'Today he; is the Prime Minister",
            'in_rs_raj_mohan_gandhi_pm_wrote_as_janata_dal_president_recalled_1990': 'arepoll',
        }
        for cid, text in printed.items():
            self.assertIn(text, self.claims[cid]['text'], cid)

    def test_starts_and_ends_only_where_a_source_states_one(self):
        claims, rows = self.claims, self.rows
        self.assertFalse([cid for cid in self.new_claims if rows[cid]['event_kind'] in FROM_KINDS | UNTIL_KINDS])
        self.assertEqual([h for h in self.role['holder_claims'] if h['from'] or h['until']], [])
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
        for cid in NEVER_HOLDER:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)never (a|an|dates? a|feeds)|claim only|claims only|'
                                                         r'no structured date', cid)
        for cid in ('in_eci_dispute_1999_pac_removal_of_sharad_yadav_claimed_19990721',):
            self.assertEqual(rows[cid]['event_kind'], 'removal_claimed')
            self.assertIn('never an until', claims[cid]['uncertainty'])
        for cid in ('in_eci_dispute_1999_deve_gowda_application_as_president_19990722',
                    'in_eci_dispute_1999_pac_elected_deve_gowda_president_claimed',
                    'in_ls_speaker_decision_ajit_singh_endorsed_as_president_claimed_19920205'):
            self.assertEqual(rows[cid]['event_kind'], 'rival_presidency_claim', cid)
        self.assertIn('never a holder', claims['in_ls_ram_naik_sharad_yadav_working_president_19970317']['uncertainty'])
        self.assertIn('before the period', claims['in_rs_anand_sharma_vp_singh_president_of_janata_dal_19891228'][
            'uncertainty'])
        self.assertIn('[13 MAY 1990]', self.sources['in_rs_debate_19900518_meham_countermanding']['scope_note'])
        scope = self.role['scope_note']
        for phrase in ('no in_prime_minister or in_presidency claim or source feeds this role',
                       'none of its claims feeds either institution', 'in_inc_president and in_bjp_president are untouched',
                       'Do not fill the interval', "infer an outgoing holder's last day from a successor's observation",
                       'is a claim only, never a holder', 'procedure only, never a date', 'holder_name null',
                       'Parliament records and Government resolutions are used only where they record the party office',
                       'never merged identities or inherited leaders', 'no holder is tied to the Janata Dal name after'):
            self.assertIn(phrase, scope)
        self.assertEqual(self.org['coverage']['unresolved'][:-1], [
            'Reconcile exact organization identity, jurisdiction and name variants before mapping to the game.',
            'Research founding, splits, mergers, dissolution and separately dated party offices from 1990 through the fixed cutoff.',
            'Review later amendments and any identity or status qualifications; no current legal conclusion is drawn.'])
        self.assertTrue(self.org['coverage']['unresolved'][-1].startswith('Janata Dal Presidents 1990-2026 (CLAUDE-C01-33)'))
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-33' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][11].startswith('Janata Dal Presidents 1990-2026 (CLAUDE-C01-33, '
                                                              'JD-PRES-01..08)'))
        self.assertTrue(coverage['unresolved'][10].startswith('BJP Presidents 1990-2026 (CLAUDE-C01-27, BJP-PRES-01..10)'))
        self.assertTrue(coverage['unresolved'][-1].startswith('CPI(M) General Secretaries 1990-2026 (CLAUDE-C01-40, '
                                                              'CPM-GS-01..06)'))
        self.assertEqual(len(coverage['unresolved']), 13)
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [6, 76])

    def test_splits_and_successors_are_claims_about_the_organisation_only(self):
        # The row that establishes the observation is the 10 January 1998 table's row 6.
        row6 = self.claims['in_eci_19980110_np_row_06']
        self.assertEqual((row6['attested_on'], row6['locator']['party_row_number']), ('1998-01-10', 6))
        self.assertEqual(self.org['recognition']['attested_on'], row6['attested_on'])
        # The dispute claims name groups, not offices, and sit on the observation, not the role.
        for cid in ORG_CLAIMS:
            self.assertIsNone(self.rows[cid]['role_id'], cid)
            self.assertNotIn(cid, self.role['claim_ids'], cid)
        for cid in ('in_eci_on28e_janata_dal_secular_and_united_rows_19990809',
                    'in_eci_dispute_1999_compiler_note_jdu_and_jds_named'):
            self.assertIn('Janata Dal (Secular)', self.claims[cid]['text'])
            self.assertIn('Janata Dal (United)', self.claims[cid]['text'])
            self.assertRegex(self.claims[cid]['uncertainty'], r'no identity, lifecycle or leader passes|never merged')
        # No other observation (the 2024 Janata Dal (Secular), Janata Dal (United) and Rashtriya Janata Dal rows among them)
        # gains a role, a claim, a source or a reconciliation from this packet.
        for entry in self.packet['organizations']:
            if entry is self.org:
                continue
            if 'Janata Dal' in entry['name']:
                self.assertEqual((entry['roles'], entry['reconciled_organization_id'], entry['represented_party_ids']),
                                 ([], None, []), entry['id'])
            self.assertFalse(set(entry['claim_ids']) & set(self.new_claims), entry['id'])
        self.assertEqual(sum(o['name'] == 'Janata Dal' for o in self.packet['organizations']), 1)
        self.assertEqual(self.org['lifecycle']['status'], 'unresearched')
        self.assertIn('no successor is merged into this observation', self.org['lifecycle']['note'])

    def test_party_and_state_offices_stay_separate(self):
        pm, presidency = self.packet['institutions']
        for entry in (pm, presidency):
            self.assertFalse(set(entry['claim_ids']) & set(self.new_claims), entry['id'])
            self.assertFalse(set(entry['sources']) & set(NEW_SOURCES), entry['id'])
            for role in entry['roles']:
                for holder in role['holder_claims']:
                    self.assertFalse(set(holder['claim_ids']) & set(self.new_claims), holder['name'])
                    self.assertNotIn(holder['name'], HOLDER_NAMES)
        # Vishwanath Pratap Singh and H. D. Deve Gowda hold the prime-ministership there and are named here only in claims: the state
        # office never supplies a holder to the party office, nor the reverse.
        pm_names = {h['name'] for h in pm['roles'][0]['holder_claims']}
        self.assertLessEqual({VPS, HDG}, pm_names)
        self.assertFalse(pm_names & {h['name'] for h in self.role['holder_claims']})
        for org_id in (c01_20.ORG, c01_27.ORG):
            role = other_role(self.packet, org_id)
            self.assertFalse(set(role['claim_ids']) & set(self.new_claims), org_id)
            self.assertFalse(set(role['sources']) & set(NEW_SOURCES), org_id)
            self.assertFalse({h['name'] for h in role['holder_claims']} & set(SURNAMES), org_id)
        # State, parliamentary and other-party offices named in these records are context only.
        self.assertIn('chief ministership of Bihar is a state office outside this role',
                      self.claims['in_ls_panigrahi_laloo_prasad_yadav_party_president_19960715']['uncertainty'])
        self.assertIn('Leader of the House is a parliamentary office outside this role',
                      self.claims['in_ls_ram_naik_party_president_sharad_yadav_19970729']['uncertainty'])
        self.assertIn('Rashtriya Janata Dal is another organisation',
                      self.claims['in_ls_virendra_kumar_singh_laloo_rjd_party_president_19970729']['uncertainty'])
        self.assertIn('prime-ministership is a separate office',
                      self.claims['in_rs_raj_mohan_gandhi_pm_wrote_as_janata_dal_president_recalled_1990']['uncertainty'])
        # The statements of 29 July 1997 do not name the party and come from speakers not identified as Janata Dal
        # officers: leads only.
        for cid in SY_LEADS:
            self.assertEqual(self.rows[cid]['event_kind'], 'party_not_named_lead', cid)
            for phrase in ('A lead only, never a holder', 'does not name the party', 'read only from context',
                           'not identified in the record as a Janata Dal officer'):
                self.assertIn(phrase, self.claims[cid]['uncertainty'], cid)
        self.assertIn('names no party office', self.claims[SY_LEADS[1]]['uncertainty'])
        # Laloo Prasad Yadav's observation rests on a non-member's statement naming the party, disclosed as such; Bommai's
        # on a party letter that the House printed without deciding the office.
        bommai, laloo = self.role['holder_claims']
        disclosure = 'The speaker (Deogarh) is not a Janata Dal officer'
        self.assertIn(disclosure, laloo['note'])
        self.assertIn(disclosure, self.claims[laloo['claim_ids'][0]]['uncertainty'])
        self.assertIn('decided nothing about the office', bommai['note'])

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
                           'at least 30 minutes apart', 'Accept-Encoding: identity', 'default User-Agent'):
                self.assertIn(phrase, extract['provenance_note'], sid)
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertIn('CLAUDE-C01-33 only', extract['bounded_scope'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES[sid])
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
            self.assertEqual('original_url' in source, sid in WAYBACK, sid)
            self.assertEqual('archive_capture_utc' in extract, sid in WAYBACK, sid)
            self.assertEqual('mirror_of' in extract, sid in IA_ITEMS, sid)
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'], row['holder_name'],
                                row['role_id']) for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual(set(STORE) | set(EGAZETTE) | set(IA_ITEMS) | set(WAYBACK) | set(IFES) | set(CMS), set(NEW_SOURCES))
        self.assertEqual((len(STORE), len(EGAZETTE), len(IA_ITEMS), len(WAYBACK), len(IFES), len(CMS)), (5, 7, 7, 1, 1, 1))
        for sid, (stamp, original) in WAYBACK.items():
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(source['url'], f'https://web.archive.org/web/{stamp}id_/{original}')
            self.assertEqual((source['original_url'], extract['original_url']), (original, original))
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('different 15,195,535-byte rendering', source['scope_note'])
        for sid in STORE:
            self.assertIn('ETag equals the MD5 of the body', self.extracts[sid]['provenance_note'])
        for sid, item in IA_ITEMS.items():
            self.assertIn(item, self.extracts[sid]['provenance_note'])
            self.assertIn('mirror', self.extracts[sid]['provenance_note'])
        for sid in EGAZETTE:
            self.assertIn('static egazette.gov.in object', self.extracts[sid]['provenance_note'])
            self.assertIn('in.gazette.central.e.', self.extracts[sid]['provenance_note'])
        for sid in IFES:
            self.assertIn('third-party repository copy', self.sources[sid]['scope_note'])
            self.assertIn('answered HTTP 406', self.sources[sid]['scope_note'])
        for sid in IA_ITEMS:
            if IA_ITEMS[sid].startswith('eparlib'):
                self.assertTrue(self.sources[sid]['scope_note'].startswith('Mirror-only provenance'), sid)
                self.assertEqual(self.sources[sid]['source_type'], 'primary_parliamentary_debate_pdf_third_party_mirror')
                self.assertTrue(self.extracts[sid]['mirror_of'].startswith(
                    'https://eparlib.sansad.in/bitstream/123456789/' + IA_ITEMS[sid].rsplit('.', 1)[1] + '/1/'), sid)

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertIsNone(PER_REQUEST.search(url), url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if sid in STORE:
                self.assertEqual(parts.hostname, 'bucketapi.rajyasabha.digital', sid)
                self.assertRegex(parts.path, r'^/public-bucket/rsdebate/assetstore/(\d\d/){3}\d+$', sid)
                # The store's query only sets the served content type and file name.
                self.assertEqual(sorted(k.split('=')[0] for k in parts.query.split('&')),
                                 ['response-content-disposition', 'response-content-type'])
                self.assertIn(STORE[sid], parts.query)
            elif sid in EGAZETTE:
                self.assertEqual(parts.hostname, 'egazette.gov.in', sid)
                self.assertEqual(parts.path, f'/WriteReadData/{EGAZETTE[sid][7:11]}/{EGAZETTE[sid]}.pdf', sid)
                self.assertEqual(parts.query, '', sid)
            elif sid in IA_ITEMS:
                self.assertEqual(parts.hostname, 'archive.org', sid)
                self.assertRegex(parts.path, rf'^/download/{re.escape(IA_ITEMS[sid])}/[\w.-]+\.pdf$', sid)
                self.assertEqual(parts.query, '', sid)
            elif sid in WAYBACK:
                self.assertEqual(parts.hostname, 'web.archive.org', sid)
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https://eparlib\.nic\.in/bitstream/', sid)
            elif sid in IFES:
                self.assertEqual((parts.hostname, parts.path), ('electionjudgments.org', f'/api/files/{IFES[sid]}'))
                self.assertEqual(parts.query, '', sid)
            else:
                self.assertEqual((parts.hostname, parts.path), ('cms.rajyasabha.nic.in', CMS[sid]))
        # The official eparlib host is recorded only as what the mirrors copy or the capture replays, never directly.
        self.assertFalse([sid for sid in NEW_SOURCES if 'eparlib' in urlsplit(self.sources[sid]['url']).hostname])

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, self.sources[sid]['url'], sid)
        lowered = json.dumps([self.sources[s] for s in NEW_SOURCES], ensure_ascii=False).lower()
        for marker in ('wikipedia', 'britannica', 'the hindu', 'indianexpress', 'ndtv', 'hindustantimes', 'timesofindia',
                       'business-standard', 'sikhheritageeducation', 'rediff', 'india today', 'frontline'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('sikhheritageeducation', 'business-standard', 'ID_155_04101990', 'ID_178_15071996',
                       'E-0297-1996-0220-10496', 'E-0492-1992-0085-19235', 'eparlib.nic.in.5888', 'eparlib.nic.in.9786',
                       'eparlib.nic.in.3579', 'eparlib.nic.in.3670', 'eparlib.nic.in.6036', 'eparlib.nic.in.56238',
                       'in.gazette.central.e.2001-02-15'):
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
            return jd_org(packet)

        def role(packet):
            return jd_role(packet)

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
                if cid not in role(packet)['claim_ids']:
                    role(packet)['claim_ids'].append(cid)
                if sid not in role(packet)['sources']:
                    role(packet)['sources'].append(sid)
            return change

        def extra_holder(name, day, cid, until=None, start=None):
            return {'name': name, 'attested_on': day, 'from': start, 'until': until, 'sources': [self.claim_source[cid]],
                    'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        validator_cases = [
            (lambda p: source(p, 'in_rs_written_answers_19900828_usq2406_pm_letters')['snapshot'].update(
                sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'in_ls_debate_19970729_bihar_situation')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'in_eci_on18e_19980110_national_parties')['snapshot'].update(path=REPORT.as_posix()),
             'escapes'),
            (lambda p: holder(p, 1).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 1).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'in_eci_19980110_np_row_06').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: org(p)['recognition'].update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'in_rs_obituary_bommai_president_all_india_janta_dal_1990_to_1996').update(
                period={'from': '1990-01-01', 'through': '2026-09-30'}), 'exceeds cutoff'),
            (lambda p: holder(p, 0).update({'from': '1990-07-14', 'until': '1990-01-01'}), 'Reversed historical interval'),
            (lambda p: holder(p, 1)['claim_ids'].append('in_rs_pm_letter_to_bommai_president_janata_dal_19900714'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('in_does_not_exist'), 'Unknown'),
            (lambda p: org(p).update(represented_party_ids=['India/in_jd']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        rule_cases = [
            # A successor's observation used as an end.
            ('successor observation used as an end (Bommai)', lambda p: holder(p, 0).update(until='1996-07-15')),
            ('successor observation cited as an end (Bommai)',
             cite(0, 'in_ls_panigrahi_laloo_prasad_yadav_party_president_19960715', until='1996-07-15')),
            ("successor's lead day used as an end (Laloo Prasad Yadav)",
             lambda p: holder(p, 1).update(until='1997-07-29')),
            ('an end invented at the cutoff (Laloo Prasad Yadav)', lambda p: holder(p, 1).update(until='2026-09-07')),
            # A removal claimed, a dispute order, a span or a recollection used as an end.
            ('removal claimed used as an end (Laloo Prasad Yadav)', lambda p: holder(p, 1).update(until='1999-07-21')),
            ('removal claimed cited as an end (Laloo Prasad Yadav)',
             cite(1, 'in_eci_dispute_1999_pac_removal_of_sharad_yadav_claimed_19990721', until='1999-07-21')),
            ('dispute order day used as an end (Laloo Prasad Yadav)', lambda p: holder(p, 1).update(until='1999-08-07')),
            ('span used as an end (Bommai)', lambda p: holder(p, 0).update(until='1996-12-31')),
            ('RJD presidency cited as an end (Laloo Prasad Yadav)',
             cite(1, 'in_ls_virendra_kumar_singh_laloo_rjd_party_president_19970729', until='1997-07-29')),
            # An election, a claimed election or a working presidency used as a start without a stated assumption.
            ('claimed election day used as a start (Sharad Yadav restored)',
             lambda p: role(p)['holder_claims'].append(extra_holder(SY, None, SY_LEADS[0], start='1997-07-03'))),
            ('working presidency used as a start (Sharad Yadav restored)',
             lambda p: role(p)['holder_claims'].append(extra_holder(
                 SY, None, 'in_ls_ram_naik_sharad_yadav_working_president_19970317', start='1997-03-17'))),
            ('observation day turned into a start (Laloo Prasad Yadav)',
             lambda p: holder(p, 1).update({'attested_on': None, 'from': '1996-07-15'})),
            ('recollection used as a start (Bommai 1989)',
             lambda p: holder(p, 0).update({'attested_on': None, 'from': '1989-12-31'})),
            ('span used as a start (Bommai)', lambda p: holder(p, 0).update({'attested_on': None, 'from': '1990-01-01'})),
            # Continuations, lists, recollections and the Commission's statement of its records cited by a holder.
            ('continuation cited by a holder (Bommai 1991)',
             cite(0, 'in_gazette_welfare_resolution_bommai_president_janata_dal_19910329')),
            ('stale composition list cited by a holder (Bommai)',
             cite(0, 'in_ls_nic_statement_lists_bommai_president_janata_dal_1990_composition')),
            ("Commission's records cited by a holder (Laloo Prasad Yadav)",
             cite(1, 'in_eci_dispute_1999_commission_records_sharad_yadav_president_19990807')),
            ('heading-derived list cited by a holder (Laloo Prasad Yadav)',
             cite(1, 'in_gazette_hrd_resolution_laloo_under_presidents_of_major_parties_19961014')),
            ('recollection cited by a holder (Laloo Prasad Yadav)',
             cite(1, 'in_rs_joyanta_roy_laloo_as_president_of_janata_dal_cmp_recalled')),
            ('continuation day used as an observation (Laloo Prasad Yadav)', lambda p: holder(p, 1).update(
                attested_on='1996-10-14')),
            # Leads, acting, working, rival and pre-period holders added.
            ('Sharad Yadav holder restored from a lead', lambda p: role(p)['holder_claims'].append({
                'name': SY, 'attested_on': '1997-07-29', 'from': None, 'until': None,
                'sources': ['in_ls_debate_19970729_bihar_situation'], 'claim_ids': list(SY_LEADS),
                'note': 'Observed on 29 July 1997.', 'uncertainty': 'No start. No end.'})),
            ('working presidency added as a holder (acting or working, Sharad Yadav)',
             lambda p: role(p)['holder_claims'].append(extra_holder(
                 SY, '1997-03-17', 'in_ls_ram_naik_sharad_yadav_working_president_19970317'))),
            ('rival claimant added as a holder (Deve Gowda)', lambda p: role(p)['holder_claims'].append(extra_holder(
                HDG, '1999-07-22', 'in_eci_dispute_1999_deve_gowda_application_as_president_19990722'))),
            ('rival claimant added as a holder (Ajit Singh)', lambda p: role(p)['holder_claims'].insert(1, extra_holder(
                AJS, '1992-02-05', 'in_ls_speaker_decision_ajit_singh_endorsed_as_president_claimed_19920205'))),
            ('attestation before the period added as a holder (V. P. Singh)',
             lambda p: role(p)['holder_claims'].insert(0, extra_holder(
                 VPS, '1989-12-28', 'in_rs_anand_sharma_vp_singh_president_of_janata_dal_19891228'))),
            ('unnamed recollection given a name and a holder (V. P. Singh 1990)',
             lambda p: role(p)['holder_claims'].insert(0, extra_holder(
                 VPS, '1990-05-18', 'in_rs_raj_mohan_gandhi_pm_wrote_as_janata_dal_president_recalled_1990'))),
            ('unnamed attestation added as a holder (1997)', lambda p: role(p)['holder_claims'].append(extra_holder(
                SY, '1997-08-05', 'in_rs_som_pal_president_of_jd_charge_framed_19970805'))),
            # Organisation claims moved onto the role or a holder.
            ('final dispute order cited by a holder (Bommai)',
             cite(0, 'in_eci_final_order_bommai_group_recognised_as_janata_dal_19930722')),
            ('group head of 1993 added as a holder (Bommai)', lambda p: role(p)['holder_claims'].insert(1, extra_holder(
                SRB, '1993-01-14', 'in_eci_janata_dal_two_groups_interim_recognition_order_19930114'))),
            ('successor row moved onto the role', lambda p: (
                role(p)['claim_ids'].append('in_eci_on28e_janata_dal_secular_and_united_rows_19990809'),
                role(p)['sources'].append('in_eci_on28e_19990809_janata_dal_dispute'))),
            # Cross-role, cross-party and cross-institution holders and claims.
            ('prime minister added as a party President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(pm_role(p)['holder_claims'][4]))),
            ('President of India added as a party President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][2]))),
            ('Congress President added as a Janata Dal President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(other_role(p, c01_20.ORG)['holder_claims'][2]))),
            ('BJP President added as a Janata Dal President', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(other_role(p, c01_27.ORG)['holder_claims'][3]))),
            ('party President moved into the prime-ministership',
             lambda p: pm_role(p)['holder_claims'].append(role(p)['holder_claims'].pop(1))),
            ('party President added to the presidency',
             lambda p: president_role(p)['holder_claims'].append(copy.deepcopy(holder(p, 0)))),
            ('party President added to the BJP role',
             lambda p: other_role(p, c01_27.ORG)['holder_claims'].append(copy.deepcopy(holder(p, 1)))),
            ('prime-ministership claim cited by a party President', cite(0, 'in_rao_appointed_pm_wef_19910621')),
            ('party claim moved onto the prime-ministership', lambda p: (
                pm_role(p)['claim_ids'].append('in_rs_pm_letter_to_bommai_president_janata_dal_19900714'),
                pm_role(p)['sources'].append('in_rs_written_answers_19900828_usq2406_pm_letters'))),
            ('party claim moved onto the Congress role', lambda p: (
                other_role(p, c01_20.ORG)['claim_ids'].append('in_ls_panigrahi_laloo_prasad_yadav_party_president_19960715'),
                other_role(p, c01_20.ORG)['sources'].append('in_ls_debate_19960715_petroleum_prices'))),
            ('party role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('party role copied onto an institution',
             lambda p: p['institutions'][0]['roles'].append(dict(copy.deepcopy(role(p)), id='in_jd_president_2'))),
            ('second role on the observation',
             lambda p: org(p)['roles'].append(dict(copy.deepcopy(role(p)), id='in_jd_general_secretary'))),
            ('role kind changed', lambda p: role(p).update(kind='head_of_government')),
            ('observation lifecycle given a start', lambda p: org(p)['lifecycle'].update({'from': '1988-10-11'})),
            ('observation lifecycle given an end at the split', lambda p: org(p)['lifecycle'].update({'until': '1999-08-07'})),
            ('observation mapped to a game party', lambda p: org(p).update(represented_party_ids=['India/in_jd'])),
            ('observation merged with a successor', lambda p: org(p).update(
                reconciled_organization_id='in_eci_20240323_sp_10_01')),
            ('observation coverage upgraded', lambda p: org(p)['coverage'].update(status='partial')),
            ('recognition given an interval', lambda p: org(p)['recognition'].update({'from': '1998-01-10'})),
            ('successor observation given a role',
             lambda p: next(o for o in p['organizations'] if o['id'] == 'in_eci_20240323_sp_10_01')['roles'].append(
                 dict(copy.deepcopy(role(p)), id='in_jds_president'))),
            # Structured dates added to undated claims, and order.
            ('span given a structured date', lambda p: claim(
                p, 'in_rs_obituary_bommai_president_all_india_janta_dal_1990_to_1996').update(attested_on='1990-01-01')),
            ('recollection given a day', lambda p: claim(
                p, 'in_rs_bommai_as_janata_dal_president_report_1989_recalled').update(attested_on='1997-08-26')),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        jd_rules(self.packet, self.rows)
        jd_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    jd_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    jd_invariants(packet, self.rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('in_ls_adsul_democratically_elected_president_sharad_yadav_19970729', '1997-07-28'),
                         ('in_eci_janata_dal_a_and_b_named_further_order_199301', '1993-01-17'),
                         ('in_eci_dispute_1999_pac_removal_of_sharad_yadav_claimed_19990721', '1999-07-22'),
                         ('in_eci_final_order_bommai_group_recognised_as_janata_dal_19930722', '1993-07-23'),
                         ('in_rs_pm_answer_styles_bommai_president_janata_dal_19900828', '1990-07-14')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                jd_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Declined', '02': 'Accepted in part', '03': 'Accepted in part', '04': 'Accepted in part',
                     '05': 'Accepted in part', '06': 'Accepted in part', '07': 'Accepted', '08': 'Accepted in part'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| JD-PRES-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('89decb6a', 'claude/c01-in-27', '3ec6e155', 'research-index.json', 'test_india_research_s10e.py',
                     'test_india_prime_ministers_c01_11.py', 'test_india_presidents_c01_15.py',
                     'test_india_inc_presidents_c01_20.py', 'test_india_bjp_presidents_c01_27.py',
                     'test_certified_gap_ledger', 'boundary matrix', 'No existing extract'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'india-janata-dal-presidents-1990-2026-33.md', 'claude/c01-in-27', '89decb6a',
                     'test_india_janata_dal_presidents_c01_33.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'India')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['organization_observations'], country['institution_observations'],
                          country['role_observations']), (83, 2, 6))
        self.assertEqual(country['mapping_pending'], 85)
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'India'}, {'open'})
        self.assertIn(ORG, next(w for w in index['work_orders'] if w['nation'] == 'India' and ORG in w['members'])[
            'members'])
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
