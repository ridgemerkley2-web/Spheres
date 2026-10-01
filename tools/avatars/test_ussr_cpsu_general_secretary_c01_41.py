"""CLAUDE-C01-41: the CPSU General Secretary and Deputy General Secretary, 1990-1991.

The existing su_cpsu roles keep their S10.h month observations unchanged and first; this packet appends dated observations from
the party's own records (Pravda, the Central Committee's organ; Известия ЦК КПСС) with attested_on only. A holder rests only on
a record printing the person in the office on that record's own day. The ballot, the vote, the election, the biography's dates,
the vote figures, intermediate attestations, the General Secretary's own words that he has laid down the office, another
deputy's reference to his resignation and every organization record (the property decree, the suspension, the RSFSR decree,
the proposal of self-dissolution) stay separate dated claims and never feed a holder, a from, an until or a lifecycle bound.
The existing 'Ivashkov' spelling is never reconciled with the Russian name. Party office and state office stay separate both
ways, and nothing reaches russia.json or the simulation's party rows."""
import copy
from datetime import datetime
import gzip
import hashlib
import json
import re
import unittest

import campaign_research as research

# The 12 sources this packet appends, in packet order (document dates ascending), at positions 56-67.
NEW_SOURCES = [
    'su_pravda_no37_19900206',
    'su_pravda_no192_19900711',
    'su_pravda_no193_19900712',
    'su_pravda_no194_19900713',
    'su_pravda_no195_19900714',
    'su_izv_tsk_1991_08',
    'su_pravda_no201_19910822',
    'su_vs_bulletin1_cpsu_19910826',
    'su_ved_1991_35_cpsu',
    'su_snd5_bulletin3_cpsu_19910903',
    'su_ved_1991_36_cpsu',
    'su_kremlin_rsfsr_ukaz_169_19911106',
]
FIRST = 56
COUNTS = (12, 26)
# CLAUDE-C01-49 later appended 6 sources and 17 claims (positions 68-73) to su_president and su_government_head.
TOTALS = (74, 174, 7, 8)
# Its holders rest only on these sources (pinned in test_ussr_government_president_c01_49) and are left out of the
# other-holders digest, which still covers every holder present at the integration base.
C01_49_SOURCES = {'su_snd3_steno_vol3_president', 'su_pravda_no13_19910115', 'su_pravda_no20_19910123', 'su_izv_197_19910820',
                  'su_ved_1991_35_president', 'su_ved_1991_41_president'}
FILES = {
    'su_pravda_no37_19900206': 'ussr-pravda-no37-cc-plenum-19900206-facts.json',
    'su_pravda_no192_19900711': 'ussr-pravda-no192-general-secretary-election-19900711-facts.json',
    'su_pravda_no193_19900712': 'ussr-pravda-no193-deputy-nominations-19900712-facts.json',
    'su_pravda_no194_19900713': 'ussr-pravda-no194-deputy-election-19900713-facts.json',
    'su_pravda_no195_19900714': 'ussr-pravda-no195-program-commission-19900714-facts.json',
    'su_izv_tsk_1991_08': 'ussr-izvestia-tsk-kpss-1991-08-masthead-19910710-facts.json',
    'su_pravda_no201_19910822': 'ussr-pravda-no201-secretariat-statement-19910822-facts.json',
    'su_vs_bulletin1_cpsu_19910826': 'ussr-sten-vs-bulletin1-cpsu-19910826-facts.json',
    'su_ved_1991_35_cpsu': 'ussr-ved-1991-35-cpsu-property-19910828-facts.json',
    'su_snd5_bulletin3_cpsu_19910903': 'ussr-snd5-bulletin3-cpsu-19910903-facts.json',
    'su_ved_1991_36_cpsu': 'ussr-ved-1991-36-cpsu-suspension-19910904-facts.json',
    'su_kremlin_rsfsr_ukaz_169_19911106': 'ussr-kremlin-rsfsr-ukaz-169-19911106-facts.json',
}
# Recorded response identity of each new source: (bytes, sha256), downloaded identical twice at least 30 minutes apart.
RESPONSES = {
    'su_pravda_no37_19900206': (8135736, '537821c7be713cd43c32c5c7fc2c600252b29c4e57ad803eac46493aee45175a'),
    'su_pravda_no192_19900711': (7723594, 'e9708e2a820511c812f197878881a4ccc62f09dae93bc92cfc457c57c8e3076c'),
    'su_pravda_no193_19900712': (7999931, 'd678165c22b0f0d80c56001928822ced823d76b08ea442c290f0fc51b98a5376'),
    'su_pravda_no194_19900713': (7671842, '8e39422b42c47382c353ae5a7450885bd5e51a7603c44867ce31b16af99ed363'),
    'su_pravda_no195_19900714': (7038476, 'c379779937798e7ffe508a2b143b5c4fb959be2c9a279b5e12c4405723b8cfe8'),
    'su_izv_tsk_1991_08': (107500579, '3cacb753dff04582732f7dc09ca3210ed46f2c80b4d6764b96e8929063424141'),
    'su_pravda_no201_19910822': (3962723, '3e5f059918ff0b91f82c86304e0ee6247cceb1f1f415672ea4225b82c8c5b610'),
    'su_vs_bulletin1_cpsu_19910826': (2252696, '849b9dae8c5d3580df911819ff466f9e6997e3119c326b1cf638b6e86239776a'),
    'su_ved_1991_35_cpsu': (618372, '5a8c0630da633ac68ef5c0514c05107497a9e84cda82541d561bc22013c55a63'),
    'su_snd5_bulletin3_cpsu_19910903': (2146995, '8eeae4731e35ac7e25d099bb85a95610382cbe215ab6b49820a57675dcd9ac26'),
    'su_ved_1991_36_cpsu': (1303663, '87abb4c154470c0681cd0867ef63119675b249d323372fdcbfe87a34ab075b6f'),
    'su_kremlin_rsfsr_ukaz_169_19911106': (11221, '58c2fd44df99196bba5250f6084315e67904849321c98f6ade12999681804f3d'),
}
# Internet Archive stored files: (item, file, archive.org's recorded SHA-1).
STORED = {
    'su_pravda_no37_19900206': ('199037_9972', 'Правда, 1990 , № 37.pdf', 'd12511893b72b8b3d5b68c75cdc308d00fdb24c5'),
    'su_pravda_no192_19900711': ('1990192_8379', 'Правда, 1990 , № 192.pdf', '6b90c03194611fe4e62df4323036b2bab3e87af5'),
    'su_pravda_no193_19900712': ('1990193_2350', 'Правда, 1990 , № 193.pdf', '09e9f05f03dcfd5dfacc18f40425249b79915c57'),
    'su_pravda_no194_19900713': ('1990194_6991', 'Правда, 1990 , № 194.pdf', '0829f20c55833c7a4d9a76bdd7361e6e00ea5412'),
    'su_pravda_no195_19900714': ('1990195_8575', 'Правда, 1990 , № 195.pdf', 'd3f7e91686cd5bfed903483984be3b14426d0e34'),
    'su_izv_tsk_1991_08': ('B-001-036-100-ALL', 'B-001-036-100-03.pdf', '042e2fb92b9821f1b4d647a66bd69342eab6c28f'),
    'su_pravda_no201_19910822': ('1991201_1702', 'Правда, 1991 , № 201.pdf', '7020a4968950e3af7cf83d761cbfa13dde9338ee'),
}
# Raw Internet Archive captures (all before the cutoff).
ARCHIVED = {
    'su_vs_bulletin1_cpsu_19910826': '20250718143607',
    'su_ved_1991_35_cpsu': '20211204065955',
    'su_snd5_bulletin3_cpsu_19910903': '20240901234307',
    'su_ved_1991_36_cpsu': '20250820135020',
    'su_kremlin_rsfsr_ukaz_169_19911106': '20260830202901',
}
LIVE = ('su_vs_bulletin1_cpsu_19910826', 'su_ved_1991_35_cpsu', 'su_snd5_bulletin3_cpsu_19910903', 'su_ved_1991_36_cpsu')
GZIPPED = {'su_kremlin_rsfsr_ukaz_169_19911106': (40450, 'aa81ecac32a6c358d140b15feade008a718466e909fa3e00bd25ccfdb3edf144')}
ORIGINAL_EDITION = (30481, '1c2da3eef0bc1df06504b652ec4232b33a080eaab4ba7a3a952da30372508a3b')
SAME_BYTES = {'su_vs_bulletin1_cpsu_19910826': 'su_vs_bulletin1_soyuz_19910826', 'su_ved_1991_35_cpsu': 'su_ved_1991_35', 'su_snd5_bulletin3_cpsu_19910903': 'su_snd5_bulletin3_19910903', 'su_ved_1991_36_cpsu': 'su_ved_1991_36'}
EARLIER_SNAPSHOTS = {'su_vs_bulletin1_soyuz_19910826': '787cc83aa9daa510230ea8231cf0a06b0caf4479ca0f81ee56f310167370f172', 'su_ved_1991_35': '0fcaa0721ac623d0131eea0f55d1b8fd0dcf88da9506ddc2ca7749a4832af8fc', 'su_snd5_bulletin3_19910903': '9e27cd412a582ab96195fd2e252528d1de35827513560e635f2554ca9904f578', 'su_ved_1991_36': 'acbdb003fc1cc99f244824614da524a17284a463640d8c391723df873838d0fb'}
PAGES = {
    'su_pravda_no37_19900206': [1],
    'su_pravda_no192_19900711': [1],
    'su_pravda_no193_19900712': [1],
    'su_pravda_no194_19900713': [1],
    'su_pravda_no195_19900714': [1, 2],
    'su_izv_tsk_1991_08': [5, 228],
    'su_pravda_no201_19910822': [1, 2],
    'su_vs_bulletin1_cpsu_19910826': [31, 32, 34],
    'su_ved_1991_35_cpsu': [23],
    'su_snd5_bulletin3_cpsu_19910903': [11, 12],
    'su_ved_1991_36_cpsu': [15, 17],
    'su_kremlin_rsfsr_ukaz_169_19911106': [],
}
# Every claim: (attested_on, event kind, review observation, role or None, entry).
EVENTS = {
    'su_pravda37_plenum_report_by_general_secretary_19900205': ('1990-02-05', 'in_office_attestation', 'SU-CPSU-01', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda192_general_secretary_ballot_19900710': ('1990-07-10', 'candidates_listed_for_ballot', 'SU-CPSU-02', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda192_general_secretary_vote_held_19900710': ('1990-07-10', 'secret_ballot_held', 'SU-CPSU-02', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda192_gorbachev_elected_general_secretary_19900710': ('1990-07-10', 'election_result_announced', 'SU-CPSU-02', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda192_gorbachev_addresses_as_general_secretary_19900710': ('1990-07-10', 'in_office_attestation', 'SU-CPSU-02', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda192_biography_reelected_general_secretary_19900710': ('1990-07-10', 'election_reported_in_biography', 'SU-CPSU-02', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda193_gorbachev_presides_as_general_secretary_19900711': ('1990-07-11', 'in_office_attestation', 'SU-CPSU-03', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda193_deputy_general_secretary_ballot_19900711': ('1990-07-11', 'candidates_listed_for_ballot', 'SU-CPSU-03', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda193_deputy_general_secretary_vote_held_19900711': ('1990-07-11', 'secret_ballot_held', 'SU-CPSU-03', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda194_ivashko_elected_deputy_general_secretary_19900712': ('1990-07-12', 'election_result_announced', 'SU-CPSU-03', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda194_deputy_general_secretary_vote_figures_19900712': ('1990-07-12', 'vote_figures_reported', 'SU-CPSU-03', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda194_biography_elected_deputy_general_secretary_19900711': ('1990-07-11', 'election_reported_in_biography', 'SU-CPSU-03', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda195_program_commission_chair_general_secretary_19900713': ('1990-07-13', 'in_office_attestation', 'SU-CPSU-03', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda195_program_commission_lists_deputy_general_secretary_19900713': ('1990-07-13', 'in_office_attestation', 'SU-CPSU-03', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_izv_tsk_1991_08_board_deputy_general_secretary_19910710': ('1991-07-10', 'in_office_attestation', 'SU-CPSU-04', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda201_dzasokhov_refers_to_general_secretary_19910821': ('1991-08-21', 'in_office_attestation', 'SU-CPSU-04', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_pravda201_ivashko_deputy_general_secretary_flew_to_crimea_19910821': ('1991-08-21', 'in_office_attestation', 'SU-CPSU-04', 'su_cpsu_deputy_general_secretary', 'su_cpsu'),
    'su_pravda201_secretariat_statement_names_general_secretary_19910822': ('1991-08-22', 'in_office_attestation', 'SU-CPSU-04', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_vs26_gorbachev_laid_down_general_secretary_duties_19910826': ('1991-08-26', 'resignation_stated_by_holder', 'SU-CPSU-05', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_vs26_gorbachev_proposed_cc_self_dissolution_19910826': ('1991-08-26', 'self_dissolution_proposed', 'SU-CPSU-06', None, 'su_cpsu'),
    'su_ukaz_up2460_cpsu_property_19910824': ('1991-08-24', 'party_property_decree', 'SU-CPSU-06', None, 'su_cpsu'),
    'su_snd5_medvedev_refers_to_general_secretary_resignation_19910903': ('1991-09-03', 'resignation_referenced_by_another_deputy', 'SU-CPSU-05', 'su_cpsu_general_secretary', 'su_cpsu'),
    'su_snd5_medvedev_cc_never_decided_self_dissolution_19910903': ('1991-09-03', 'self_dissolution_not_adopted_statement', 'SU-CPSU-06', None, 'su_cpsu'),
    'su_vs_2371i_cpsu_activity_suspended_19910829': ('1991-08-29', 'activity_suspended', 'SU-CPSU-06', None, 'su_cpsu'),
    'su_rsfsr_ukaz_169_cpsu_activity_terminated_19911106': ('1991-11-06', 'activity_terminated_in_republic', 'SU-CPSU-06', None, 'su_cpsu'),
    'su_rsfsr_ukaz_169_cpsu_property_to_state_19911106': ('1991-11-06', 'party_property_transferred_in_republic', 'SU-CPSU-06', None, 'su_cpsu'),
}
ROW_HOLDERS = {
    'su_pravda37_plenum_report_by_general_secretary_19900205': 'Михаил Сергеевич Горбачев',
    'su_pravda192_general_secretary_ballot_19900710': None,
    'su_pravda192_general_secretary_vote_held_19900710': None,
    'su_pravda192_gorbachev_elected_general_secretary_19900710': 'Михаил Сергеевич Горбачев',
    'su_pravda192_gorbachev_addresses_as_general_secretary_19900710': 'Михаил Сергеевич Горбачев',
    'su_pravda192_biography_reelected_general_secretary_19900710': 'Михаил Сергеевич Горбачев',
    'su_pravda193_gorbachev_presides_as_general_secretary_19900711': 'Михаил Сергеевич Горбачев',
    'su_pravda193_deputy_general_secretary_ballot_19900711': None,
    'su_pravda193_deputy_general_secretary_vote_held_19900711': None,
    'su_pravda194_ivashko_elected_deputy_general_secretary_19900712': 'Владимир Антонович Ивашко',
    'su_pravda194_deputy_general_secretary_vote_figures_19900712': 'Владимир Антонович Ивашко',
    'su_pravda194_biography_elected_deputy_general_secretary_19900711': 'Владимир Антонович Ивашко',
    'su_pravda195_program_commission_chair_general_secretary_19900713': 'Михаил Сергеевич Горбачев',
    'su_pravda195_program_commission_lists_deputy_general_secretary_19900713': 'Владимир Антонович Ивашко',
    'su_izv_tsk_1991_08_board_deputy_general_secretary_19910710': 'Владимир Антонович Ивашко',
    'su_pravda201_dzasokhov_refers_to_general_secretary_19910821': 'Михаил Сергеевич Горбачев',
    'su_pravda201_ivashko_deputy_general_secretary_flew_to_crimea_19910821': 'Владимир Антонович Ивашко',
    'su_pravda201_secretariat_statement_names_general_secretary_19910822': 'Михаил Сергеевич Горбачев',
    'su_vs26_gorbachev_laid_down_general_secretary_duties_19910826': 'Михаил Сергеевич Горбачев',
    'su_vs26_gorbachev_proposed_cc_self_dissolution_19910826': None,
    'su_ukaz_up2460_cpsu_property_19910824': None,
    'su_snd5_medvedev_refers_to_general_secretary_resignation_19910903': 'Михаил Сергеевич Горбачев',
    'su_snd5_medvedev_cc_never_decided_self_dissolution_19910903': None,
    'su_vs_2371i_cpsu_activity_suspended_19910829': None,
    'su_rsfsr_ukaz_169_cpsu_activity_terminated_19911106': None,
    'su_rsfsr_ukaz_169_cpsu_property_to_state_19911106': None,
}
# The exact holders: the unchanged S10.h month observation first, then the new observations in chronological order.
HOLDERS = {
    'su_cpsu_general_secretary': [
        ('Mikhail Gorbachev', None, None, None),
        ('Михаил Сергеевич Горбачев', '1990-02-05', None, None),
        ('Михаил Сергеевич Горбачев', '1990-07-10', None, None),
        ('Михаил Сергеевич Горбачев', '1991-08-22', None, None),
    ],
    'su_cpsu_deputy_general_secretary': [
        ('Vladimir Ivashkov (source spelling; identity reconciliation pending)', None, None, None),
        ('Владимир Антонович Ивашко', '1990-07-13', None, None),
        ('Владимир Антонович Ивашко', '1991-08-21', None, None),
    ],
}
HOLDER_CLAIMS = {
    'su_cpsu_general_secretary': [['su_mofa_gorbachev_general_secretary_199007'], ['su_pravda37_plenum_report_by_general_secretary_19900205'], ['su_pravda192_gorbachev_addresses_as_general_secretary_19900710'], ['su_pravda201_secretariat_statement_names_general_secretary_19910822']],
    'su_cpsu_deputy_general_secretary': [['su_mofa_deputy_general_secretary_199007'], ['su_pravda195_program_commission_lists_deputy_general_secretary_19900713'], ['su_pravda201_ivashko_deputy_general_secretary_flew_to_crimea_19910821']],
}
BASE_HOLDERS = {
    "su_cpsu_general_secretary": {
        "name": "Mikhail Gorbachev",
        "from": None,
        "until": None,
        "sources": [
            "su_japan_diplomatic_bluebook_1990"
        ],
        "claim_ids": [
            "su_mofa_gorbachev_general_secretary_199007"
        ],
        "observation_window": {
            "from": "1990-07-01",
            "through": "1990-07-31"
        },
        "precision": "month"
    },
    "su_cpsu_deputy_general_secretary": {
        "name": "Vladimir Ivashkov (source spelling; identity reconciliation pending)",
        "from": None,
        "until": None,
        "sources": [
            "su_japan_diplomatic_bluebook_1990"
        ],
        "claim_ids": [
            "su_mofa_deputy_general_secretary_199007"
        ],
        "observation_window": {
            "from": "1990-07-01",
            "through": "1990-07-31"
        },
        "precision": "month"
    }
}
ORG_FIXED = {
    "name": "Communist Party of the Soviet Union (CPSU)",
    "kind": "political_party",
    "represented_party_ids": [],
    "jurisdiction": {
        "nation": "USSR",
        "level": "union",
        "automatic_successor_mapping": False,
        "note": "Not the RSFSR/Russian Federation; no automatic transfer to Russia or a republic-level organization."
    },
    "lifecycle": {
        "status": "documented_at_specific_observations",
        "from": None,
        "until": None,
        "precision": "unknown",
        "note": "These observations do not establish founding, dissolution or a continuous lifespan. Later institutional changes remain unresolved."
    }
}
BASE_ORG_CLAIMS = ['su_article6_replacement_19900314', 'su_mofa_gorbachev_general_secretary_199007', 'su_mofa_deputy_general_secretary_199007']
BASE_ORG_SOURCES = ['su_presidency_law_19900314', 'su_japan_diplomatic_bluebook_1990']
BASE_ROLE_CLAIMS = {'su_cpsu_general_secretary': ['su_mofa_gorbachev_general_secretary_199007'], 'su_cpsu_deputy_general_secretary': ['su_mofa_deputy_general_secretary_199007']}
BASE_ROLE_SOURCES = {'su_cpsu_general_secretary': ['su_japan_diplomatic_bluebook_1990'], 'su_cpsu_deputy_general_secretary': ['su_japan_diplomatic_bluebook_1990']}
ROLES = {'su_cpsu_general_secretary': ('General Secretary', 'party_leader'), 'su_cpsu_deputy_general_secretary': ('Deputy General Secretary', 'other')}
# Every USSR role other than the two CPSU roles, with its holders, as on the integration base.
OTHER_HOLDERS_SHA256 = '118a1e53267de1003d78e061c922ac880412c9ea060afe9a5f0186901078099e'
# Separate events in date order: each earlier claim is dated strictly before the later one.
ORDERED = (
    ('su_pravda37_plenum_report_by_general_secretary_19900205', 'su_pravda192_gorbachev_addresses_as_general_secretary_19900710'),
    ('su_pravda193_deputy_general_secretary_vote_held_19900711', 'su_pravda194_ivashko_elected_deputy_general_secretary_19900712'),
    ('su_pravda194_ivashko_elected_deputy_general_secretary_19900712', 'su_pravda195_program_commission_lists_deputy_general_secretary_19900713'),
    ('su_pravda201_secretariat_statement_names_general_secretary_19910822', 'su_vs26_gorbachev_laid_down_general_secretary_duties_19910826'),
    ('su_ukaz_up2460_cpsu_property_19910824', 'su_vs_2371i_cpsu_activity_suspended_19910829'),
    ('su_vs_2371i_cpsu_activity_suspended_19910829', 'su_rsfsr_ukaz_169_cpsu_activity_terminated_19911106'),
)
GS = 'su_cpsu_general_secretary'
DGS = 'su_cpsu_deputy_general_secretary'
ORG = 'su_cpsu'
HOLDER_KINDS = {'in_office_attestation'}
ELECTION_KINDS = {'candidates_listed_for_ballot', 'secret_ballot_held', 'election_result_announced',
                  'election_reported_in_biography', 'vote_figures_reported'}
END_KINDS = {'resignation_stated_by_holder', 'resignation_referenced_by_another_deputy'}
ORG_KINDS = {'self_dissolution_proposed', 'party_property_decree', 'self_dissolution_not_adopted_statement', 'activity_suspended',
             'activity_terminated_in_republic', 'party_property_transferred_in_republic'}
VOCABULARY = HOLDER_KINDS | ELECTION_KINDS | END_KINDS | ORG_KINDS
HOLDER_CLAIM_IDS = {c for ids in HOLDER_CLAIMS.values() for group in ids for c in group}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_IDS)
# Days that must never become a holder or lifecycle boundary: every claim day, the election and resignation days and the
# party's end-of-period records.
NEVER_BOUNDARY = {v[0] for v in EVENTS.values()} | {'1990-07-11', '1990-07-12', '1991-08-24', '1991-08-26', '1991-08-29',
                                                    '1991-11-06'}
LEAD_URL_MARKERS = ('wikipedia', 'wikiwand', 'ruwiki', 'doc20vek', 'gorby.ru', 'soveticus5', 'narod.ru', 'bigenc', 'ria.ru',
                    'yeltsin.ru/archive/search', 'dokumen.pub', 'imwerden', 'e-history', 'rusconstitution', 'interfax')
REPORT = research.RESEARCH / 'ussr-cpsu-general-secretary-1990-1991-41.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-41.md'
DECISIONS = {'SU-CPSU-01': 'Accepted', 'SU-CPSU-02': 'Accepted', 'SU-CPSU-03': 'Accepted', 'SU-CPSU-04': 'Accepted in part',
             'SU-CPSU-05': 'Accepted in part', 'SU-CPSU-06': 'Accepted in part'}


def roles_of(packet):
    return {r['id']: (e, r) for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles']}


def holder_digest(packet, skip_roles, appended=frozenset()):
    rows = [(r['id'], [(h['name'], h.get('attested_on'), h['from'], h['until'], h['sources'], h['claim_ids'])
                       for h in r['holder_claims']
                       if isinstance(h, dict) and not (h['sources'] and set(h['sources']) <= appended)])
            for g in ('organizations', 'institutions') for e in packet[g] for r in e['roles'] if r['id'] not in skip_roles]
    return hashlib.sha256(json.dumps(rows, ensure_ascii=False).encode('utf-8')).hexdigest()


def c41_invariants(ussr, russia):
    """Packet-level rules this test owns; raises AssertionError, KeyError, IndexError or StopIteration on any violation."""
    claims = {c['id']: c for s in ussr['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in ussr['sources'] for c in s['claims']}
    orgs = {e['id']: e for e in ussr['organizations']}
    org = orgs[ORG]
    assert list(orgs)[0] == ORG
    for key, value in ORG_FIXED.items():
        assert org[key] == value, key
    roles = roles_of(ussr)
    assert [r['id'] for r in org['roles']] == [GS, DGS]
    for role_id, (title, kind) in ROLES.items():
        assert (roles[role_id][1]['title'], roles[role_id][1]['kind']) == (title, kind), role_id
    # The S10.h month observations stay first and byte-for-byte unchanged, including the 'Ivashkov' source spelling.
    for role_id, base in BASE_HOLDERS.items():
        assert roles[role_id][1]['holder_claims'][0] == base, role_id
    assert 'Ivashkov (source spelling; identity reconciliation pending)' in roles[DGS][1]['holder_claims'][0]['name']
    # Every other USSR role and holder is unchanged.
    assert holder_digest(ussr, {GS, DGS}, C01_49_SOURCES) == OTHER_HOLDERS_SHA256
    # Each new holder rests on one in-office claim of its own role, dated that day, with no from and no until (rules first,
    # so a mutation meets the rule it breaks; the exact lists are compared at the end).
    for role_id in HOLDERS:
        role = roles[role_id][1]
        new = role['holder_claims'][1:]
        days = [h['attested_on'] for h in new]
        assert all(days) and days == sorted(days), role_id
        for h in new:
            assert h['from'] is None and h['until'] is None, h['name']
            assert not {h['from'], h['until']} & NEVER_BOUNDARY, h['name']
            assert not re.search(r'(?i)acting|исполняющ|временно|и\. ?о\.|Ivashkov', h['name']), h['name']
            assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
            assert all(cid in role['claim_ids'] for cid in h['claim_ids']), h['name']
            assert all(claims[cid]['attested_on'] == h['attested_on'] for cid in h['claim_ids']), h['name']
            assert h['sources'] == list(dict.fromkeys(owner[cid] for cid in h['claim_ids'])), h['name']
            assert all(EVENTS[cid][1] in HOLDER_KINDS and EVENTS[cid][3] == role_id for cid in h['claim_ids']), h['name']
    # The deputy's holders never name the General Secretary and the reverse.
    names = {rid: {h['name'] for h in roles[rid][1]['holder_claims'][1:]} for rid in HOLDERS}
    assert not names[GS] & names[DGS]
    # The exact holders, existing first, then the new ones in chronological order.
    for role_id, expected in HOLDERS.items():
        holders = roles[role_id][1]['holder_claims']
        assert [(h['name'], h.get('attested_on'), h['from'], h['until']) for h in holders] == expected, role_id
        assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS[role_id], role_id
    # Each claim is cited by its own role (role claims) and by su_cpsu (every claim), after the existing citations, and by
    # nothing else in either packet.
    ours = set(EVENTS)
    assert org['claim_ids'] == BASE_ORG_CLAIMS + list(EVENTS)
    assert org['sources'] == BASE_ORG_SOURCES + NEW_SOURCES
    for role_id in (GS, DGS):
        ids = [cid for cid, v in EVENTS.items() if v[3] == role_id]
        role = roles[role_id][1]
        assert role['claim_ids'] == BASE_ROLE_CLAIMS[role_id] + ids, role_id
        assert role['sources'] == BASE_ROLE_SOURCES[role_id] + list(dict.fromkeys(owner[c] for c in ids)), role_id
    for packet in (russia, ussr):
        for group in ('organizations', 'institutions'):
            for entry in packet[group]:
                if entry['id'] != ORG:
                    assert not set(entry['claim_ids']) & ours and not set(entry['sources']) & set(NEW_SOURCES), entry['id']
                for r in entry['roles']:
                    cited = set(r['claim_ids']) | {c for h in r['holder_claims'] if isinstance(h, dict) for c in h['claim_ids']}
                    allowed = {cid for cid, v in EVENTS.items() if v[3] == r['id']}
                    assert not (cited & ours) - allowed, r['id']
                    if r['id'] not in (GS, DGS):
                        assert not set(r['sources']) & set(NEW_SOURCES), r['id']
    # Organization records (suspension, termination, property, self-dissolution) are never role claims.
    for cid, v in EVENTS.items():
        if v[1] in ORG_KINDS:
            assert v[3] is None, cid
    # No lifecycle bound comes from any claim.
    for entry in ussr['organizations'] + ussr['institutions']:
        if entry['id'] != 'su_presidency':
            assert (entry['lifecycle']['from'], entry['lifecycle']['until']) == (None, None), entry['id']
        assert entry['lifecycle']['until'] is None, entry['id']
    for earlier, later in ORDERED:
        assert claims[earlier]['attested_on'] < claims[later]['attested_on'], (earlier, later)
    for cid in ours:
        assert '1990-01-01' <= claims[cid]['attested_on'] <= '1991-12-25', cid


def c41_extract_check(packet, root):
    """Extracts equal the packet: snapshot bytes and hash, claim ids and texts, dates and locators."""
    sources = {s['id']: s for s in packet['sources']}
    for sid in NEW_SOURCES:
        source = sources[sid]
        data = (root / source['snapshot']['path']).read_bytes()
        assert (len(data), hashlib.sha256(data).hexdigest()) == (source['snapshot']['bytes'], source['snapshot']['sha256']), sid
        extract = json.loads(data)
        assert [r['claim_id'] for r in extract['rows']] == [c['id'] for c in source['claims']], sid
        for row, claim in zip(extract['rows'], source['claims']):
            assert row['text'] == claim['text'] and row['locator'] == claim['locator'], claim['id']
            assert row['attested_on'] == claim['attested_on'] == EVENTS[claim['id']][0], claim['id']
            assert (row['event_kind'], row['review_observation'], row['role_id']) == EVENTS[claim['id']][1:4], claim['id']
            assert row['holder_name'] == ROW_HOLDERS[claim['id']], claim['id']
        assert extract['source_url'] == source['url'], sid
        assert (extract['source_response_bytes'], extract['source_response_sha256']) == RESPONSES[sid], sid


class UssrCpsuGeneralSecretaryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8')
        cls.russia_raw = (research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8')
        cls.packet, cls.russia = json.loads(cls.raw), json.loads(cls.russia_raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.roles = roles_of(cls.packet)
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
        self.assertEqual((len(NEW_SOURCES), len(EVENTS)), COUNTS)
        self.assertEqual([s['id'] for s in self.packet['sources'][FIRST:FIRST + len(NEW_SOURCES)]], NEW_SOURCES)
        self.assertEqual((len(ids['sources']), len(ids['claims']), len(ids['entries']), len(ids['roles'])), TOTALS)
        self.assertEqual([c['id'] for sid in NEW_SOURCES for c in self.sources[sid]['claims']], list(EVENTS))
        dates = [self.sources[sid]['document_date'] for sid in NEW_SOURCES]
        self.assertEqual(dates, sorted(dates))
        self.assertEqual({v[1] for v in EVENTS.values()}, VOCABULARY)
        self.assertEqual({v[2] for v in EVENTS.values()}, {f'SU-CPSU-0{n}' for n in range(1, 7)})
        self.assertEqual(re.findall(r'^### (SU-CPSU-\d\d)\b', self.report, re.M), [f'SU-CPSU-0{n}' for n in range(1, 7)])
        for sid in NEW_SOURCES:
            self.assertTrue(sid.startswith('su_'))
            self.assertEqual(self.sources[sid]['accessed_date'], '2026-09-30')
            for c in self.sources[sid]['claims']:
                self.assertTrue(c['id'].startswith('su_'))
        # At most ten people are researched: the two holders.
        holders = {h['name'] for rid in (GS, DGS) for h in self.roles[rid][1]['holder_claims'][1:]}
        self.assertEqual(holders, {'Михаил Сергеевич Горбачев', 'Владимир Антонович Ивашко'})

    def test_holders_are_exactly_as_intended(self):
        c41_invariants(self.packet, self.russia)
        for role_id in (GS, DGS):
            role = self.roles[role_id][1]
            self.assertIn('CLAUDE-C01-41', role['scope_note'])
            self.assertIn('unchanged', role['scope_note'])

    def test_starts_and_ends_only_where_a_source_states_one(self):
        for role_id in (GS, DGS):
            for h in self.roles[role_id][1]['holder_claims']:
                self.assertIsNone(h['from'])
                self.assertIsNone(h['until'])
        org = self.roles[GS][0]
        self.assertEqual((org['lifecycle']['status'], org['lifecycle']['from'], org['lifecycle']['until']),
                         ('documented_at_specific_observations', None, None))
        # Elections, resignation statements and organization records are claims, never holders or bounds.
        for cid, (day, kind, _, role_id, _) in EVENTS.items():
            if kind in ELECTION_KINDS | END_KINDS | ORG_KINDS:
                self.assertIn(cid, NEVER_HOLDER)
        self.assertIn('states no day', self.claims['su_snd5_medvedev_refers_to_general_secretary_resignation_19910903']['uncertainty'])
        self.assertIn('no until is set', self.claims['su_vs26_gorbachev_laid_down_general_secretary_duties_19910826']['uncertainty'])
        self.assertIn('sets no from', self.claims['su_pravda192_gorbachev_elected_general_secretary_19900710']['uncertainty'])
        self.assertIn('sets no from', self.claims['su_pravda194_ivashko_elected_deputy_general_secretary_19900712']['uncertainty'])
        self.assertIn('not a dissolution', self.claims['su_vs_2371i_cpsu_activity_suspended_19910829']['uncertainty'])
        # The vote (11 July) and the announcement (12 July) of the deputy's election stay two dated claims.
        self.assertEqual(self.claims['su_pravda193_deputy_general_secretary_vote_held_19900711']['attested_on'], '1990-07-11')
        self.assertEqual(self.claims['su_pravda194_ivashko_elected_deputy_general_secretary_19900712']['attested_on'], '1990-07-12')
        self.assertEqual(self.claims['su_pravda194_biography_elected_deputy_general_secretary_19900711']['attested_on'], '1990-07-11')
        # No acting holder for the deputy after 24 August 1991.
        for h in self.roles[GS][1]['holder_claims']:
            self.assertNotIn('Ивашко', h['name'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        c41_extract_check(self.packet, research.ROOT)
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual(extract['source_id'], sid)
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertFalse(extract['source_response_checked_in'])
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('No portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PAGES[sid])
            self.assertEqual(source['snapshot']['kind'], 'derived_factual_extract')
            self.assertEqual(source['snapshot']['path'], 'docs/campaign-certification/C01/research/sources/' + FILES[sid])
            self.assertNotEqual(extract['source_response_sha256'], source['snapshot']['sha256'])
            for key in ('observation_id',):
                self.assertEqual({r[key] for r in extract['rows']}, {ORG})
            # Two downloads of every recorded and attached response, 30 minutes or more apart, identical.
            self.assertEqual(extract['downloads'][0]['response'], 'source_response')
            for d in extract['downloads']:
                self.assertEqual(d['result'], 'identical on both downloads')
                t1, t2 = (datetime.fromisoformat(t.replace('Z', '+00:00')) for t in d['downloaded_at'])
                self.assertGreaterEqual((t2 - t1).total_seconds(), 1800)
            self.assertEqual((extract['downloads'][0]['bytes'], extract['downloads'][0]['sha256']), RESPONSES[sid])
            self.assertEqual(extract['downloads'][0]['url'], source['url'])
            if sid in STORED:
                ident, name, sha1 = STORED[sid]
                stored = extract['stored_file']
                self.assertEqual((stored['identifier'], stored['file'], stored['archive_org_file_sha1']), (ident, name, sha1))
                self.assertEqual(stored['archive_org_file_size'], RESPONSES[sid][0])
                self.assertTrue(source['url'].startswith('https://ia') and '.us.archive.org/' in source['url'])
                self.assertIn(ident, source['url'])
                self.assertEqual(source['access_method'], 'internet_archive_stored_file')
            else:
                self.assertEqual(source['url'], f"https://web.archive.org/web/{ARCHIVED[sid]}id_/{source['original_url']}")
                self.assertLess(ARCHIVED[sid][:8], '20260907')
                self.assertEqual(source['access_method'], 'internet_archive_raw_capture')
            if sid in LIVE:
                self.assertEqual((extract['live_file_response']['bytes'], extract['live_file_response']['sha256']), RESPONSES[sid])
                self.assertEqual(extract['live_file_response']['url'], source['original_url'])
            if sid in GZIPPED:
                self.assertEqual(extract['source_response_content_encoding'], 'gzip')
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), GZIPPED[sid])
                self.assertEqual((extract['original_edition_response']['bytes'], extract['original_edition_response']['sha256']),
                                 ORIGINAL_EDITION)
            else:
                self.assertNotIn('source_response_content_encoding', extract)
        # Four sources repeat bytes already recorded by earlier packets under their own IDs; those records stay unchanged.
        for sid, earlier in SAME_BYTES.items():
            old = self.sources[earlier]
            old_extract = json.loads((research.ROOT / old['snapshot']['path']).read_text(encoding='utf-8'))
            self.assertEqual((old_extract['source_response_bytes'], old_extract['source_response_sha256']), RESPONSES[sid])
            self.assertEqual(old['snapshot']['sha256'], EARLIER_SNAPSHOTS[earlier])
            self.assertFalse({c['id'] for c in old['claims']} & set(EVENTS))
        # Event kinds and holder names per row.
        for cid, row in self.rows.items():
            self.assertEqual(row['event_kind'], EVENTS[cid][1])
            self.assertEqual(row['holder_name'], ROW_HOLDERS[cid])

    def test_secondary_leads_and_volatile_urls_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            urls = [source['url'], source.get('original_url') or '']
            for marker in LEAD_URL_MARKERS:
                self.assertFalse(any(marker in u for u in urls), (sid, marker))
            self.assertNotRegex(source['url'], r'[?&](cb|_cb|_chk|nocache|q)=|/web/\d{14}/|search')
            self.assertTrue(source['source_type'].startswith('primary_'), sid)
        # Pravda's reportage (vote figures, the press conference) never carries the General Secretary's holder.
        self.assertEqual(self.rows['su_pravda194_deputy_general_secretary_vote_figures_19900712']['event_kind'], 'vote_figures_reported')
        self.assertIn('Reportage', self.claims['su_pravda194_deputy_general_secretary_vote_figures_19900712']['uncertainty'])
        self.assertIn('su_pravda201_dzasokhov_refers_to_general_secretary_19910821', NEVER_HOLDER)
        leads = self.section('Leads not imported')
        for marker in ('doc20vek', 'soveticus5', 'Wikipedia', 'gorby.ru'):
            self.assertIn(marker, leads)

    def test_separation_from_state_offices_russia_and_the_simulation(self):
        c41_invariants(self.packet, self.russia)
        for oid in ('su_presidency', 'su_supreme_soviet', 'su_government', 'su_congress_peoples_deputies'):
            entry = next(e for e in self.packet['institutions'] if e['id'] == oid)
            self.assertFalse(set(entry['sources']) & set(NEW_SOURCES), oid)
        for cid in EVENTS:
            self.assertNotIn(cid, self.russia_raw)
        for sid in NEW_SOURCES:
            self.assertNotIn(sid, self.russia_raw)
        self.assertEqual(self.roles[GS][0]['represented_party_ids'], [])
        self.assertNotIn('reconciled_organization_id', self.roles[GS][0])

    def test_packet_formatting_is_preserved(self):
        self.assertEqual(self.raw, json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')
        for sid in NEW_SOURCES:
            text = (research.ROOT / self.sources[sid]['snapshot']['path']).read_text(encoding='utf-8')
            self.assertEqual(text, json.dumps(self.extracts[sid], indent=2, ensure_ascii=False) + '\n')
            self.assertNotIn('\r', text)

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def role(packet, role_id):
            return roles_of(packet)[role_id][1]

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def org(packet):
            return next(o for o in packet['organizations'] if o['id'] == ORG)

        def added(name, day, sid, cid, role_id):
            return lambda p: role(p, role_id)['holder_claims'].append(
                {'name': name, 'attested_on': day, 'from': None, 'until': None, 'sources': [sid], 'claim_ids': [cid]})

        changes = {
            'election day as from': lambda p: role(p, GS)['holder_claims'][2].update({'from': '1990-07-10'}),
            'resignation day as until': lambda p: role(p, GS)['holder_claims'][3].update({'until': '1991-08-24'}),
            'deputy election announcement as from': lambda p: role(p, DGS)['holder_claims'][1].update({'from': '1990-07-12'}),
            'holder day moved off its claim': lambda p: role(p, DGS)['holder_claims'][1].update({'attested_on': '1990-07-12'}),
            'Ivashkov reconciled': lambda p: role(p, DGS)['holder_claims'][0].update({'name': 'Владимир Антонович Ивашко'}),
            'existing holder removed': lambda p: role(p, GS)['holder_claims'].pop(0),
            'new holders reordered': lambda p: role(p, GS)['holder_claims'].insert(1, role(p, GS)['holder_claims'].pop(3)),
            'holder from the resignation words': added(
                'Михаил Сергеевич Горбачев', '1991-08-26', 'su_vs_bulletin1_cpsu_19910826',
                'su_vs26_gorbachev_laid_down_general_secretary_duties_19910826', GS),
            'deputy acting as General Secretary': added(
                'Владимир Антонович Ивашко (и. о. Генерального секретаря)', '1991-08-21', 'su_pravda_no201_19910822',
                'su_pravda201_ivashko_deputy_general_secretary_flew_to_crimea_19910821', GS),
            'holder from the election result': added(
                'Владимир Антонович Ивашко', '1990-07-12', 'su_pravda_no194_19900713',
                'su_pravda194_ivashko_elected_deputy_general_secretary_19900712', DGS),
            'party claim filed on the Presidency': lambda p: roles_of(p)['su_president'][1]['claim_ids'].append(
                'su_pravda192_gorbachev_addresses_as_general_secretary_19900710'),
            'suspension as lifecycle end': lambda p: org(p)['lifecycle'].update({'until': '1991-08-29'}),
            'RSFSR decree as lifecycle end': lambda p: org(p)['lifecycle'].update({'until': '1991-11-06'}),
            'role claim dropped': lambda p: role(p, DGS)['claim_ids'].remove(
                'su_pravda193_deputy_general_secretary_ballot_19900711'),
            'organization claim moved to a role': lambda p: role(p, GS)['claim_ids'].append(
                'su_vs_2371i_cpsu_activity_suspended_19910829'),
            'claim redated': lambda p: claim(p, 'su_pravda195_program_commission_lists_deputy_general_secretary_19900713').update(
                {'attested_on': '1990-07-14'}),
        }
        for label, change in changes.items():
            with self.subTest(label):
                with self.assertRaises((AssertionError, KeyError, IndexError, StopIteration)):
                    c41_invariants(mutated(change), self.russia)
        # A claim text that differs from its extract is rejected.
        packet = mutated(lambda p: claim(p, 'su_pravda37_plenum_report_by_general_secretary_19900205').update(
            {'text': 'Генеральный секретарь'}))
        with self.assertRaises(AssertionError):
            c41_extract_check(packet, research.ROOT)
        # A future date and a broken snapshot fail the shared validator.
        packet = mutated(lambda p: role(p, GS)['holder_claims'][3].update({'attested_on': '2026-09-08'}))
        with self.assertRaisesRegex(ValueError, 'exceeds cutoff'):
            self.validate(packet)
        packet = mutated(lambda p: next(s for s in p['sources'] if s['id'] == NEW_SOURCES[0])['snapshot'].update(
            {'sha256': '0' * 64}))
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            self.validate(packet)

    def test_report_and_handoff_are_ready_for_review_and_close_nothing(self):
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        self.assertIn('State: **ready_for_review**', handoff)
        self.assertIn('State: **ready_for_review**', self.report)
        self.assertNotIn('State: **complete**', handoff)
        outcome = self.section('Outcome')
        for obs, decision in DECISIONS.items():
            self.assertRegex(outcome, rf'\| {obs} \|[^\n]*\*\*{decision}[:*]')
        for heading in ('Sources added', 'Response identities and stability checks', 'Leads not imported', 'Sources attempted',
                        'Suggested next work orders', 'Integration notes (outside this packet\'s file boundary)', 'Checks'):
            self.section(heading)
        notes = self.section("Integration notes (outside this packet's file boundary)")
        self.assertIn('research-index.json', notes)
        self.assertIn('continue existing claims first', notes)
        for sid in NEW_SOURCES:
            self.assertIn(f'`{sid}`', self.report)
        self.assertIn('C01 and all parent gates stay open', handoff)


if __name__ == '__main__':
    unittest.main()
