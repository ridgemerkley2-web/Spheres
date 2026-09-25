"""CLAUDE-C01-16: the ANC's Presidents, 1990-2026, are a party office kept apart from the Presidency of the Republic.
Each National Conference's election, result publication, acceptance, handover and departure, deputy and chair offices
and continuation attestations stay separate claims, and every holder is a dated in-office observation with no start or
end, because no ANC record reviewed states one."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
from test_south_africa_deputy_presidents_c01_21 import RESPONSES as C01_21_RESPONSES

ANC_ID = 'za_iec_n2024_014'
ROLE = 'za_anc_president'
PRES = 'President of the African National Congress'
DEPUTY = 'Deputy President of the African National Congress'
CHAIR = 'National Chairman of the African National Congress'
# The packet's sources before this packet: the original S10h intake and CLAUDE-C01-09 (pinned in their own tests).
EARLIER_SOURCE_COUNT = 55

# Original response identity recorded in each extract, (bytes, sha256), fetched as the extract's fetch_recipe says:
# the raw id_ capture, no Accept-Encoding header, no decoding. Every source is a raw Internet Archive capture.
RESPONSES = {
    'za_anc_jan8_statement_19900108': (44048, '38277dede7cd881d97c6ea58dfab692d81c28314c257dcec7179473491a69fff'),
    'za_anc_nec_statement_19900302': (5363, '754b4849e9b08f556fc199625852e902887cb100534df5681c739051fc9ad7f2'),
    'za_anc_tambo_48th_opening_19910702': (23902, '845207bfd2b2231fdf7bc9a26fe75f305ee599acc07e1d6d556fe774aed1fcc8'),
    'za_anc_mandela_48th_opening_19910702': (38670, '47f53e018831eaa83bb2e81af9cd7ac75d8d448053eb937208389d4f27354b80'),
    'za_anc_48th_conference_index_1997': (3569, 'dbd246de0f5b32487fe449d1e0a89cf4e6efb2c382742f3ad379c72d7d660c01'),
    'za_anc_48th_conference_statement_19910713': (7606, '170a20ef90c6ef653a1133cfba6987b3f4d6f09aef65af948ec57870fe88c9e4'),
    'za_anc_mandela_48th_closing_199107': (24345, 'e9d364f2b8ab94bd0fadac78a378b2493292b3c94b7559f36e60672a7d02ec96'),
    'za_anc_nec_statement_19910718': (5344, '2ccf364c84d812d1d92859980bb2a61dfc46f3f1fc1ca7416e05070ed33a3bd0'),
    'za_anc_49th_conference_index_1997': (3473, 'c9929f597eb29eb2c640573c78cc33c7e5933d002521cef5a165ea586c0ade36'),
    'za_anc_mandela_49th_opening_19941217': (60145, '691a00d70d738ee3427ca2585012fc1e49647ba1cd36fa222b89f3366e568269'),
    'za_anc_49th_nec_as_elected_199412': (4004, '378bcd623b3faf679e9b4f0da277e4a6a9e8b6c2d909b2d94c3410c30319132a'),
    'za_anc_mandela_49th_closing_19941222': (21566, '967eb375580293bba746f6f06db82353fd604b3a12d3537f9774b382809935da'),
    'za_anc1912_mandela_48th_closing_address_page': (206816, 'da6f822adb6ee4a9411486952faa748e0c773383ef7c8f4ad29db3ca5fde06a0'),
    'za_anc1912_48th_conference_page': (156498, 'a958ee3c7d20c617e98bfdd7138b48d8f04c3792bb5c76ce2d07b9ea39c4a502'),
    'za_anc1912_49th_nec_as_elected_page': (186330, 'c6507c276fcc618f4240d7822a66dac08231ce407744b4c76249a7b136e740ac'),
    'za_anc_conf50_documents_index_19980121': (7826, 'c0015916eead2ac62378ed54276f6ea274796bc720b2736f7ef6da3b3a0cd0b3'),
    'za_anc_conf50_report_introduction': (4138, '59c8088a93d10e9e608adcc95e979d32c1dcd3ce735834f9019a2cfb8b85e535'),
    'za_anc_mandela_political_report_19971216': (155970, '1f47d2d6a32075c170c4b2138b6bdb7ac44f0d354aa74790a07d6a50023d7725'),
    'za_anc_mandela_closing_address_19971220': (11036, 'a55c4a73bceefcb1662596eff500d88cf18ed201d94aa2e122e6d36bb1fd7d57'),
    'za_anc_mbeki_closing_statement_19971220': (12372, '1c35392d95232c367eca2c5b937b2a415608cd56505e524c84f4ac4a76251f9f'),
    'za_anc_mbeki_stadium_address_19971220': (10645, '904bc31823844e3bd8c4ca5d6e54925e6973dfeb1f6e675d5bf5942d9ecd0ac8'),
    'za_anc_biography_mbeki_1998': (4499, 'd848299fbedf9b194c49c55c5d23324ccf6a40bf69e75663b2ac9fffb039d8c8'),
    'za_anc_conf51_programme': (8771, 'e49d09ca55231023c2964f0e44b90882824d18ff27d84ae9554e4501c61dd562'),
    'za_anc_conf51_briefing_200302': (122638, 'a8679d79b2537bef5e482985773a608b7eb3adf8393da96ac2b5b8191315db2b'),
    'za_anc_mbeki_closing_statement_20021220': (17777, '4aaac5efc9611db3a347ff091f0dba2a67b9a099f7a414536e0236ca18493d36'),
    'za_anc_conf52_officials_nominations': (26094, '28bade6095558b2110fed7b61659dd516609225ea89a9d23fa5ac11bda9fd771'),
    'za_anc_mbeki_political_report_20071216': (122472, '105cf529a2d08a1410839214e094ed2586b70dd5f21d4dd2accb39261681fe31'),
    'za_anc_electoral_commission_statement_20071218': (2567, '7e3ba46f979ac775d8af67e7f6ebbf05419d5e2de95af65608a06f19e18b8111'),
    'za_anc_conf52_press_statements_index': (1468, '6e558b3283cdb962845f1ad01e6b582f095a9171a31837abb2758b67f827c8b8'),
    'za_anc_officials_election_results_2007': (1051, '59f8d09a567509eb7cd3ac4cdc3bf76d0c7416b2e561896e048daf76b085520a'),
    'za_anc_zuma_closing_statement_20071220': (14941, 'aadbc7279ae7da4961838d14384729b010700e5fa016241dd51a94bc5264b972'),
    'za_anc_newly_elected_nec_2007': (3578, '62ae26bc9061ca7ba31e4a0e9b584b6aa4e5fef771e3b35877a6902ddf99a415'),
    'za_anc_today_v7n50_20071221': (39662, '22e7c961aded54084cc8681009e21ef090bd8d1963f26416fa01630b517dd1bd'),
    'za_anc_mayibuye_199803': (87296, '310f413591deaf503cdbb8d79ea82d95a82d0184b0df266033d0025818caaf51'),
    'za_anc_conf50_declaration': (7007, '31492be0086e2a0215b94284ca7f6dfb351071efc7390084bf9cde24df7383cb'),
    'za_anc_mbeki_opening_address_51st_20021216': (104348, 'cd93efdaccab7ef73e50b1d05d688276c54ada3d223e3f020fe2e829187772b3'),
    'za_anc_political_report_zuma_53rd_20121216': (66057, '43390c9fc49c12d2d1cee7465f86bbdbbdbc3fd9839b58e193d421a52dbabd01'),
    'za_anc_events_53rd_conference_20121222': (31288, '166898ce561672232f1028fc2a2056f959439ced4e25116f4102ba1813728189'),
    'za_anc_closing_remarks_zuma_53rd_20121220': (40166, '5cf876573fe22bd4bc41e422468259ed599abe472cb1d1763d7ff05e52e67af0'),
    'za_anc_nec_members_53rd_2012': (30832, '3b0dbc7782b23de98b39d1789fff00c6025f449555686729b92e4f8705f31825'),
    'za_anc_statement_vavi_response_20121218': (29125, 'd8aa0f947fd20e48741853f6b8833f46bb9e902840c8b9ce22803139c4b4b589'),
    'za_anc_statement_zille_remarks_20121219': (29344, 'cad89d4848971cb7413cd849b06ea1d66ba8ed506f6a2f97ee7baa19b8a02507'),
    'za_anc_zuma_centenary_concert_address_20130106': (38812, '970bf1038ba1919b85e8f78b842299f040bea1c9ab5b3c5d4733ce6270253748'),
    'za_anc_officials_ramaphosa_profile_20171222': (71494, 'beb7e76a7c457aa93ef0555e3a25ab9890151d3efcaca123eb16ff5f8de75607'),
    'za_anc_statement_zuma_congratulates_ramaphosa_20171220': (47085, 'ec7a959adfd91b8f36e8584f6facd720fca56a9bdbbfd139584de4a7a9f144ec'),
    'za_anc_closing_address_ramaphosa_54th_20171220': (34165, '2237863075ba872d827a6941b5be339b634d9459ebc8377f34bfb6a968b161a9'),
    'za_anc_54th_conference_report': (750729, '80feb09418ad9ba8b1b9efd792419d5d10cc0c4189376a7696473caa4e53b969'),
    'za_anc_54th_declaration_20171220': (25499, '2c8f2bb22e4209e0298a50912adde1fe32f696d738648f5c5f9bebdcc0b11f95'),
    'za_anc_political_report_ramaphosa_55th_20221216': (55991, 'c6432fa29f7ce29bb7dffa64ad309755598e5d0621a1e910c2c13532c33b94f6'),
    'za_anc_55th_conference_page_20221223': (29197, '2a5def8f7ff68af91ea868bd8df085e15348720fdb67f07ffcf4c985df07d3ca'),
    'za_anc_january_8_statement_2023': (257284, '4b8cc59d264521aa26f3ff9f6353c9c8b824ee375e3187c5a0af9771be8abe99'),
    'za_anc_officials_page_20230129': (215841, 'e909e45826647e70289e8026d10f5eead6a6d162e989db3aa2679b3366c30df1'),
    'za_anc1912_55th_declaration_20230105': (208104, '7bad5a2cbb57fac85bb8582e5a99def11701b09f78040e0961faba08696ca2fc'),
    'za_anc_sg_statement_special_nec_20260515': (329216, 'ca97396712d69acd466d750f6a2630ab392b31d5731db468b4491d0b28d304ec'),
}
# Capture timestamp of each raw id_ capture (all before the 7 September 2026 cutoff).
ARCHIVED = {
    'za_anc_jan8_statement_19900108': '20010505074110',
    'za_anc_nec_statement_19900302': '20010505080051',
    'za_anc_tambo_48th_opening_19910702': '19970709011829',
    'za_anc_mandela_48th_opening_19910702': '19970708204307',
    'za_anc_48th_conference_index_1997': '19970709013440',
    'za_anc_48th_conference_statement_19910713': '20001002044652',
    'za_anc_mandela_48th_closing_199107': '19970708204332',
    'za_anc_nec_statement_19910718': '20001002044708',
    'za_anc_49th_conference_index_1997': '19970709013452',
    'za_anc_mandela_49th_opening_19941217': '19971017205525',
    'za_anc_49th_nec_as_elected_199412': '20001002034916',
    'za_anc_mandela_49th_closing_19941222': '19971017205545',
    'za_anc1912_mandela_48th_closing_address_page': '20211028121642',
    'za_anc1912_48th_conference_page': '20210919035856',
    'za_anc1912_49th_nec_as_elected_page': '20211019220812',
    'za_anc_conf50_documents_index_19980121': '19980212214032',
    'za_anc_conf50_report_introduction': '19990224081712',
    'za_anc_mandela_political_report_19971216': '20020504205232',
    'za_anc_mandela_closing_address_19971220': '19980212211617',
    'za_anc_mbeki_closing_statement_19971220': '19980212211649',
    'za_anc_mbeki_stadium_address_19971220': '19980212211543',
    'za_anc_biography_mbeki_1998': '19980212215253',
    'za_anc_conf51_programme': '20021214121357',
    'za_anc_conf51_briefing_200302': '20030802024112',
    'za_anc_mbeki_closing_statement_20021220': '20030116145943',
    'za_anc_conf52_officials_nominations': '20081202224304',
    'za_anc_mbeki_political_report_20071216': '20071220195823',
    'za_anc_electoral_commission_statement_20071218': '20090107170908',
    'za_anc_conf52_press_statements_index': '20080425082951',
    'za_anc_officials_election_results_2007': '20071222060228',
    'za_anc_zuma_closing_statement_20071220': '20071224100917',
    'za_anc_newly_elected_nec_2007': '20071226030548',
    'za_anc_today_v7n50_20071221': '20071224095929',
    'za_anc_mayibuye_199803': '20040417182638',
    'za_anc_conf50_declaration': '19990224112822',
    'za_anc_mbeki_opening_address_51st_20021216': '20030115064311',
    'za_anc_political_report_zuma_53rd_20121216': '20130811122031',
    'za_anc_events_53rd_conference_20121222': '20121222151527',
    'za_anc_closing_remarks_zuma_53rd_20121220': '20130811125052',
    'za_anc_nec_members_53rd_2012': '20130811110029',
    'za_anc_statement_vavi_response_20121218': '20130818112629',
    'za_anc_statement_zille_remarks_20121219': '20130818101708',
    'za_anc_zuma_centenary_concert_address_20130106': '20130818001502',
    'za_anc_officials_ramaphosa_profile_20171222': '20171222003417',
    'za_anc_statement_zuma_congratulates_ramaphosa_20171220': '20171224043428',
    'za_anc_closing_address_ramaphosa_54th_20171220': '20171225105931',
    'za_anc_54th_conference_report': '20211109161824',
    'za_anc_54th_declaration_20171220': '20171226233021',
    'za_anc_political_report_ramaphosa_55th_20221216': '20230103184831',
    'za_anc_55th_conference_page_20221223': '20221223112927',
    'za_anc_january_8_statement_2023': '20230126132648',
    'za_anc_officials_page_20230129': '20230129165110',
    'za_anc1912_55th_declaration_20230105': '20230206054451',
    'za_anc_sg_statement_special_nec_20260515': '20260515091552',
}
# Captures the archive stores and serves gzip-encoded: decoded identity (bytes, sha256), recorded beside the served one.
GZIP = {
    'za_anc_political_report_ramaphosa_55th_20221216': (274319, '3c1b2958a88bc8ff5a3dabbdcf05ae4fa0668d4b055128d19869a2182af6a1d8'),
    'za_anc_55th_conference_page_20221223': (192283, '82523409b1c97e6c3a20c6397b4a7ba25238776ad5488f6bbf610d0bc40ed407'),
}
# PDF pages rendered and visually reviewed.
PDF_PAGES = {
    'za_anc_conf51_briefing_200302': [1, 16],
    'za_anc_conf52_officials_nominations': [1],
    'za_anc_54th_conference_report': [1, 11, 13, 82],
}
# Every new claim's (attested_on, event_kind), exactly: distinct dated events are never re-dated or relabelled.
EVENTS = {
    'za_anc_tambo_president_jan8_statement_19900108': ('1990-01-08', 'in_office_attestation'),
    'za_anc_nec_elects_mandela_deputy_president_19900302': (None, 'deputy_president_election_by_nec'),
    'za_anc_nec_greets_president_tambo_19900302': ('1990-03-02', 'in_office_continuation_attestation'),
    'za_anc_tambo_valedictory_presidency_19910702': ('1991-07-02', 'valedictory_statement'),
    'za_anc_tambo_salutes_deputy_president_mandela_19910702': ('1991-07-02', 'deputy_in_office_attestation'),
    'za_anc_mandela_greets_president_tambo_19910702': ('1991-07-02', 'in_office_continuation_attestation'),
    'za_anc_48th_conference_dates_19910702_19910706': (None, 'retrospective_conference_dates'),
    'za_anc_48th_conference_elects_mandela_president_199107': (None, 'national_conference_election'),
    'za_anc_48th_conference_elects_tambo_national_chairman_199107': (None, 'national_conference_election_other_office'),
    'za_anc_mandela_accepts_presidency_199107': (None, 'election_acceptance_statement'),
    'za_anc_mandela_recounts_tambo_release_199107': (None, 'predecessor_not_available_statement'),
    'za_anc_nec_meeting_president_mandela_19910718': ('1991-07-18', 'in_office_attestation'),
    'za_anc_49th_conference_dates_19941217_19941222': (None, 'retrospective_conference_dates'),
    'za_anc_mandela_presents_nec_political_report_19941217': ('1994-12-17', 'in_office_continuation_attestation'),
    'za_anc_49th_conference_elects_mandela_president_199412': (None, 'national_conference_election_result_list'),
    'za_anc_mandela_leads_incoming_nec_19941222': ('1994-12-22', 'in_office_attestation'),
    'za_anc1912_heading_mandela_48th_closing_19910706': (None, 'retrospective_web_edition_date'),
    'za_anc1912_48th_conference_mandela_elected_president': (None, 'election_reference_retrospective'),
    'za_anc1912_49th_nec_list_heading_19941220': (None, 'national_conference_election_result_list'),
    'za_anc_conf50_mbeki_elected_president_19971216_19971220': (None, 'national_conference_election'),
    'za_anc_conf50_report_mbeki_elected_19971216_19971220': (None, 'national_conference_election'),
    'za_anc_conf50_report_mandela_departure_19971216_19971220': (None, 'departure_from_office_stated'),
    'za_anc_mandela_political_report_19971216': ('1997-12-16', 'in_office_continuation_attestation'),
    'za_anc_mandela_handover_prospective_19971216': ('1997-12-16', 'handover_announced_prospective'),
    'za_anc_mandela_hands_over_baton_19971220': ('1997-12-20', 'handover_speech'),
    'za_anc_mandela_salutes_mbeki_my_president_19971220': ('1997-12-20', 'in_office_attestation'),
    'za_anc_mbeki_president_closing_statement_19971220': ('1997-12-20', 'in_office_attestation'),
    'za_anc_mbeki_president_stadium_address_19971220': ('1997-12-20', 'in_office_attestation'),
    'za_anc_bio_mbeki_became_president_199712': (None, 'retrospective_biography_statement'),
    'za_anc_conf51_programme_nec_announcement_scheduled': (None, 'national_conference_session_scheduled_prospective'),
    'za_anc_conf51_briefing_mbeki_president_as_elected_20021216_20021220': (None, 'national_conference_election_result_list'),
    'za_anc_mbeki_president_closing_statement_20021220': ('2002-12-20', 'in_office_attestation'),
    'za_anc_conf52_zuma_nomination_accepted_pre_conference': (None, 'nomination_acceptance'),
    'za_anc_conf52_mbeki_nomination_accepted_pre_conference': (None, 'nomination_acceptance'),
    'za_anc_mbeki_political_report_20071216': ('2007-12-16', 'in_office_continuation_attestation'),
    'za_anc_mbeki_recalls_leadership_elected_stellenbosch_2002': (None, 'election_reference_retrospective'),
    'za_anc_conf52_first_round_voting_20071218': ('2007-12-18', 'election_vote'),
    'za_anc_conf52_results_announcement_expected_20071218': ('2007-12-18', 'result_declaration_expected_prospective'),
    'za_anc_conf52_officials_results_statement_listed_20071219': ('2007-12-19', 'result_publication'),
    'za_anc_conf52_officials_results_zuma_president': (None, 'national_conference_election_result_list'),
    'za_anc_zuma_president_closing_statement_20071220': ('2007-12-20', 'in_office_attestation'),
    'za_anc_zuma_accepts_mandate_20071220': ('2007-12-20', 'election_acceptance_statement'),
    'za_anc_zuma_succeeding_mbeki_20071220': ('2007-12-20', 'predecessor_reference'),
    'za_anc_zuma_two_presidents_state_and_party_20071220': ('2007-12-20', 'context_office_separation'),
    'za_anc_conf52_newly_elected_nec_zuma_president': (None, 'national_conference_election_result_list'),
    'za_anc_today_zuma_newly_elected_president_20071221': ('2007-12-21', 'in_office_continuation_attestation'),
    'za_anc_today_mbeki_outgoing_president_20071221': ('2007-12-21', 'predecessor_reference'),
    'za_anc_mayibuye_office_bearers_elected_second_day_1997': (None, 'election_reference_retrospective'),
    'za_anc_conf50_declaration_salutes_outgoing_president_mandela': (None, 'departure_from_office_stated'),
    'za_anc_mbeki_opening_political_report_20021216': ('2002-12-16', 'in_office_continuation_attestation'),
    'za_anc_zuma_political_report_as_president_20121216': ('2012-12-16', 'in_office_continuation_attestation'),
    'za_anc_site_officials_panel_zuma_president_20121222': ('2012-12-22', 'in_office_continuation_attestation'),
    'za_anc_53rd_conference_document_index_20121221': ('2012-12-21', 'document_index_entry'),
    'za_anc_zuma_closing_remarks_incoming_leadership_20121220': ('2012-12-20', 'in_office_attestation'),
    'za_anc_nec_members_elected_53rd_officials_2012': (None, 'national_conference_election_result_list'),
    'za_anc_zuma_shortly_after_being_elected_20121218': ('2012-12-18', 'election_reference'),
    'za_anc_zuma_re_election_referenced_20121219': ('2012-12-19', 'election_reference'),
    'za_anc_zuma_address_as_anc_president_20130106': ('2013-01-06', 'in_office_continuation_attestation'),
    'za_anc_ramaphosa_elected_president_54th_20171218': ('2017-12-18', 'election_reference_retrospective'),
    'za_anc_ramaphosa_new_anc_president_statement_20171220': ('2017-12-20', 'in_office_attestation'),
    'za_anc_zuma_former_anc_president_20171220': ('2017-12-20', 'predecessor_reference'),
    'za_anc_ramaphosa_closing_address_as_president_20171220': ('2017-12-20', 'in_office_attestation'),
    'za_anc_zuma_outgoing_president_20171220': ('2017-12-20', 'predecessor_reference'),
    'za_anc_54th_report_officials_president_ramaphosa_201712': (None, 'national_conference_election_result_list'),
    'za_anc_54th_declaration_conference_dates_20171216_20171220': (None, 'conference_session_dates'),
    'za_anc_54th_declaration_web_conference_dates': (None, 'conference_session_dates'),
    'za_anc_ramaphosa_political_report_as_president_20221216': ('2022-12-16', 'in_office_continuation_attestation'),
    'za_anc_55th_conference_dates_20221216_20221220': (None, 'conference_session_dates'),
    'za_anc_ramaphosa_january8_address_as_president_20230108': ('2023-01-08', 'in_office_attestation'),
    'za_anc_officials_new_officials_ramaphosa_president_20230129': ('2023-01-29', 'in_office_continuation_attestation'),
    'za_anc_55th_declaration_nasrec_and_imvelo_sessions': (None, 'conference_session_dates'),
    'za_anc_nec_reaffirms_ramaphosa_anc_president_20260515': ('2026-05-15', 'in_office_attestation'),
    'za_anc_ramaphosa_elected_five_year_term_55th_202212': (None, 'election_reference_retrospective'),
}
NEW_SOURCES = list(RESPONSES)

# Exact holder observations of za_anc_president, (name, attested_on, from, until), one per reviewed observation.
HOLDERS = [
    ('Oliver Tambo', '1990-01-08', None, None),
    ('Nelson Mandela', '1991-07-18', None, None),
    ('Nelson Mandela', '1994-12-22', None, None),
    ('Thabo Mbeki', '1997-12-20', None, None),
    ('Thabo Mbeki', '2002-12-20', None, None),
    ('Jacob Zuma', '2007-12-20', None, None),
    ('Jacob Zuma', '2012-12-20', None, None),
    ('Cyril Ramaphosa', '2017-12-20', None, None),
    ('Cyril Ramaphosa', '2023-01-08', None, None),
    ('Cyril Ramaphosa', '2026-05-15', None, None),
]
HOLDER_CLAIMS = [
    ['za_anc_tambo_president_jan8_statement_19900108'],
    ['za_anc_nec_meeting_president_mandela_19910718'],
    ['za_anc_mandela_leads_incoming_nec_19941222'],
    ['za_anc_mandela_salutes_mbeki_my_president_19971220', 'za_anc_mbeki_president_closing_statement_19971220',
     'za_anc_mbeki_president_stadium_address_19971220'],
    ['za_anc_mbeki_president_closing_statement_20021220'],
    ['za_anc_zuma_president_closing_statement_20071220'],
    ['za_anc_zuma_closing_remarks_incoming_leadership_20121220'],
    ['za_anc_ramaphosa_new_anc_president_statement_20171220', 'za_anc_ramaphosa_closing_address_as_president_20171220'],
    ['za_anc_ramaphosa_january8_address_as_president_20230108'],
    ['za_anc_nec_reaffirms_ramaphosa_anc_president_20260515'],
]
# Claims that must never feed a holder observation.
ELECTIONS = (
    'za_anc_48th_conference_elects_mandela_president_199107', 'za_anc_mandela_accepts_presidency_199107',
    'za_anc1912_48th_conference_mandela_elected_president', 'za_anc_49th_conference_elects_mandela_president_199412',
    'za_anc1912_49th_nec_list_heading_19941220', 'za_anc_conf50_mbeki_elected_president_19971216_19971220',
    'za_anc_conf50_report_mbeki_elected_19971216_19971220', 'za_anc_mayibuye_office_bearers_elected_second_day_1997',
    'za_anc_bio_mbeki_became_president_199712', 'za_anc_conf51_briefing_mbeki_president_as_elected_20021216_20021220',
    'za_anc_mbeki_recalls_leadership_elected_stellenbosch_2002', 'za_anc_conf52_zuma_nomination_accepted_pre_conference',
    'za_anc_conf52_mbeki_nomination_accepted_pre_conference', 'za_anc_conf52_first_round_voting_20071218',
    'za_anc_conf52_results_announcement_expected_20071218', 'za_anc_conf52_officials_results_statement_listed_20071219',
    'za_anc_conf52_officials_results_zuma_president', 'za_anc_zuma_accepts_mandate_20071220',
    'za_anc_conf52_newly_elected_nec_zuma_president', 'za_anc_nec_members_elected_53rd_officials_2012',
    'za_anc_zuma_shortly_after_being_elected_20121218', 'za_anc_zuma_re_election_referenced_20121219',
    'za_anc_ramaphosa_elected_president_54th_20171218', 'za_anc_54th_report_officials_president_ramaphosa_201712',
    'za_anc_ramaphosa_elected_five_year_term_55th_202212')
HANDOVERS = (
    'za_anc_tambo_valedictory_presidency_19910702', 'za_anc_mandela_recounts_tambo_release_199107',
    'za_anc_conf50_report_mandela_departure_19971216_19971220', 'za_anc_mandela_handover_prospective_19971216',
    'za_anc_mandela_hands_over_baton_19971220', 'za_anc_conf50_declaration_salutes_outgoing_president_mandela',
    'za_anc_zuma_succeeding_mbeki_20071220', 'za_anc_today_mbeki_outgoing_president_20071221',
    'za_anc_zuma_former_anc_president_20171220', 'za_anc_zuma_outgoing_president_20171220')
DEPUTY_AND_CHAIR = ('za_anc_nec_elects_mandela_deputy_president_19900302',
                    'za_anc_tambo_salutes_deputy_president_mandela_19910702',
                    'za_anc_48th_conference_elects_tambo_national_chairman_199107')
CONTINUATION = (
    'za_anc_nec_greets_president_tambo_19900302', 'za_anc_mandela_greets_president_tambo_19910702',
    'za_anc_mandela_presents_nec_political_report_19941217', 'za_anc_mandela_political_report_19971216',
    'za_anc_mbeki_opening_political_report_20021216', 'za_anc_mbeki_political_report_20071216',
    'za_anc_today_zuma_newly_elected_president_20071221', 'za_anc_zuma_political_report_as_president_20121216',
    'za_anc_site_officials_panel_zuma_president_20121222', 'za_anc_zuma_address_as_anc_president_20130106',
    'za_anc_ramaphosa_political_report_as_president_20221216',
    'za_anc_officials_new_officials_ramaphosa_president_20230129')
SPANS_AND_CONTEXT = (
    'za_anc_48th_conference_dates_19910702_19910706', 'za_anc1912_heading_mandela_48th_closing_19910706',
    'za_anc_49th_conference_dates_19941217_19941222', 'za_anc_conf51_programme_nec_announcement_scheduled',
    'za_anc_zuma_two_presidents_state_and_party_20071220', 'za_anc_53rd_conference_document_index_20121221',
    'za_anc_54th_declaration_conference_dates_20171216_20171220', 'za_anc_54th_declaration_web_conference_dates',
    'za_anc_55th_conference_dates_20221216_20221220', 'za_anc_55th_declaration_nasrec_and_imvelo_sessions')
NEVER_HOLDER = ELECTIONS + HANDOVERS + DEPUTY_AND_CHAIR + CONTINUATION + SPANS_AND_CONTEXT
# Claims printed without a structured date: month-only, span-only, undated lists and web-edition headings.
UNDATED = (
    'za_anc_nec_elects_mandela_deputy_president_19900302', 'za_anc_48th_conference_dates_19910702_19910706',
    'za_anc_48th_conference_elects_mandela_president_199107', 'za_anc_48th_conference_elects_tambo_national_chairman_199107',
    'za_anc_mandela_accepts_presidency_199107', 'za_anc_mandela_recounts_tambo_release_199107',
    'za_anc_49th_conference_dates_19941217_19941222', 'za_anc_49th_conference_elects_mandela_president_199412',
    'za_anc1912_heading_mandela_48th_closing_19910706', 'za_anc1912_48th_conference_mandela_elected_president',
    'za_anc1912_49th_nec_list_heading_19941220', 'za_anc_conf50_mbeki_elected_president_19971216_19971220',
    'za_anc_conf50_report_mbeki_elected_19971216_19971220', 'za_anc_conf50_report_mandela_departure_19971216_19971220',
    'za_anc_bio_mbeki_became_president_199712', 'za_anc_conf51_programme_nec_announcement_scheduled',
    'za_anc_conf51_briefing_mbeki_president_as_elected_20021216_20021220',
    'za_anc_conf52_zuma_nomination_accepted_pre_conference', 'za_anc_conf52_mbeki_nomination_accepted_pre_conference',
    'za_anc_mbeki_recalls_leadership_elected_stellenbosch_2002', 'za_anc_conf52_officials_results_zuma_president',
    'za_anc_conf52_newly_elected_nec_zuma_president', 'za_anc_mayibuye_office_bearers_elected_second_day_1997',
    'za_anc_conf50_declaration_salutes_outgoing_president_mandela', 'za_anc_nec_members_elected_53rd_officials_2012',
    'za_anc_54th_report_officials_president_ramaphosa_201712', 'za_anc_54th_declaration_conference_dates_20171216_20171220',
    'za_anc_54th_declaration_web_conference_dates', 'za_anc_55th_conference_dates_20221216_20221220',
    'za_anc_55th_declaration_nasrec_and_imvelo_sessions', 'za_anc_ramaphosa_elected_five_year_term_55th_202212')
# The one event kind that may date a holder; every other kind never does.
HOLDER_KINDS = {'in_office_attestation'}
BOUNDARY_KINDS = {'assumption_of_office', 'end_of_term_statement', 'resignation_effective', 'oath_of_office'}
# Dates that are never any holder's attested_on, start or end: NEC and conference elections, the conference spans and
# their stated or derived days, result publications, election references, continuation attestations, handovers and
# declarations, the derived day two of the 1997 conference and the 2026 briefing day.
NEVER_HOLDER_DATE = {
    '1990-03-01', '1990-03-02', '1991-07-02', '1991-07-06', '1991-07-07', '1991-07-13', '1991-07-17', '1994-12-17',
    '1994-12-20', '1994-12-21', '1997-12-16', '1997-12-17', '1997-12-19', '2002-12-16', '2007-12-16', '2007-12-18',
    '2007-12-19', '2007-12-21', '2012-12-16', '2012-12-18', '2012-12-19', '2012-12-21', '2012-12-22', '2013-01-06',
    '2017-12-18', '2022-12-16', '2022-12-19', '2022-12-20', '2023-01-05', '2023-01-29', '2026-05-13', '2026-05-14',
    '2026-08-31'}
SURNAMES = {'Oliver Tambo': 'Tambo', 'Nelson Mandela': 'Mandela', 'Thabo Mbeki': 'Mbeki', 'Jacob Zuma': 'Zuma',
            'Cyril Ramaphosa': 'Ramaphosa'}
REVIEW = [f'ZA-ANC-{n:02d}' for n in range(1, 11)]
# Secondary leads and live pages that must never be a source: history sites, encyclopaedias, news, the live ANC site.
LEAD_URL_MARKERS = ('sahistory', 'wikipedia', 'britannica', 'news24', 'iol.co.za', 'timeslive', 'dailymaverick',
                    'mg.co.za', 'sabcnews', 'politicsweb', 'polity.org', 'reuters', 'bbc.', 'voanews')
# Page shapes a server can generate per request (search, API, e-mail tokens, cache-busting queries, live pages).
PER_REQUEST_URL = re.compile(r'wp-json|cdx/search|cdn-cgi|email-protection|nocache|cachebust|[?&]s=|[?&]_=|'
                             r'[?&]cb=|/search[/?]|page-range|download\.php', re.I)
ARCHIVED_URL = re.compile(r'^https://web\.archive\.org/web/(\d{14})id_/(https?://(?:www\.)?(?:anc\.org\.za|anc1912\.org\.za)'
                          r'(?::80)?/.*)$')
DROPPED = ('za_anc_officials_page_20260831', 'za_anc_officials_ramaphosa_president_20260831')
STALE_IDS = ('za_conf50_mbeki_elected_anc_president_19971216_19971220', 'za_zuma_elected_anc_president_results_20071219',
             'za_conf52_zuma_nomination_accepted_20071208', 'za_conf52_newly_elected_nec_zuma_president_20071221',
             'za_mandela_anc_president_political_report_19971216', 'za_mbeki_anc_president_closing_statement_19971220')
REPORT = research.RESEARCH / 'south-africa-anc-presidents-1990-2026-16.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-16.md'


def anc_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError or IndexError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    claim_source = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    anc, = [o for o in packet['organizations'] if o['id'] == ANC_ID]
    # The IEC reporting identity, unknown lifecycle and empty game mapping stay unchanged.
    assert (anc['name'], anc['kind']) == ('AFRICAN NATIONAL CONGRESS', 'national_ballot_party_reporting_identity')
    assert anc['source_identifier']['value'] == 'national-results-page-1-row-14'
    assert anc['represented_party_ids'] == [] and anc['reconciled_organization_id'] is None
    assert anc['lifecycle']['status'] == 'unknown' and anc['lifecycle']['from'] is None
    assert anc['lifecycle']['until'] is None
    assert [r['id'] for r in anc['roles']] == [ROLE], 'exactly one ANC role'
    role = anc['roles'][0]
    assert (role['title'], role['kind']) == (PRES, 'party_leader')
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    for h in holders:
        assert h['from'] is None and h['until'] is None, h['name']
        assert h['attested_on'] not in NEVER_HOLDER_DATE, h['name']
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        expected = []
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'] and cid.startswith('za_anc'), cid
            assert claims[cid]['attested_on'] == h['attested_on'], cid
            if claim_source[cid] not in expected:
                expected.append(claim_source[cid])
        assert h['sources'] == expected, h['name']
    # Party office and the Presidency of the Republic never feed each other.
    assert len(packet['institutions']) == 1
    presidency = packet['institutions'][0]
    assert presidency['id'] == 'za_presidency'
    # CLAUDE-C01-21 adds the Deputy President role and its twelve holder observations to the presidency.
    assert [r['id'] for r in presidency['roles']] == ['za_president_election', 'za_state_president',
                                                      'za_deputy_president']
    assert sum(len(r['holder_claims']) for r in presidency['roles']) == 23
    p_claims = set(presidency['claim_ids']) | {c for r in presidency['roles'] for c in r['claim_ids']} | {
        c for r in presidency['roles'] for h in r['holder_claims'] for c in h['claim_ids']}
    p_sources = set(presidency['sources']) | {s for r in presidency['roles'] for s in r['sources']} | {
        s for r in presidency['roles'] for h in r['holder_claims'] for s in h['sources']}
    anc_claims = set(role['claim_ids']) | {c for h in holders for c in h['claim_ids']}
    anc_sources = set(role['sources']) | {s for h in holders for s in h['sources']}
    assert not anc_claims & p_claims and not anc_sources & p_sources
    assert all(c.startswith('za_anc') for c in anc_claims) and not any(c.startswith('za_anc') for c in p_claims)
    assert all(s.startswith('za_anc') for s in anc_sources) and not any(s.startswith('za_anc') for s in p_sources)
    assert all(claim_source[c] in anc_sources for c in anc_claims)
    assert 'za_ramaphosa_president_elect_20240614' not in anc_claims
    for entry in packet['organizations'] + packet['institutions']:
        if entry is not anc:
            assert not set(entry['claim_ids']) & anc_claims and not set(entry['sources']) & anc_sources, entry['id']
            assert not any(r['id'] == ROLE for r in entry['roles']), entry['id']
    assert set(role['claim_ids']) <= set(anc['claim_ids']) and set(role['sources']) <= set(anc['sources'])
    # Undated claims carry no structured date at all.
    for cid in UNDATED:
        assert not {'attested_on', 'attested_period', 'period'} & set(claims[cid]), cid
    # Distinct dated events stay distinct.
    assert claims['za_anc_conf52_first_round_voting_20071218']['attested_on'] == '2007-12-18'
    assert claims['za_anc_conf52_officials_results_statement_listed_20071219']['attested_on'] == '2007-12-19'
    assert claims['za_anc_zuma_president_closing_statement_20071220']['attested_on'] == '2007-12-20'
    assert claims['za_anc_zuma_shortly_after_being_elected_20121218']['attested_on'] == '2012-12-18'
    assert claims['za_anc_zuma_closing_remarks_incoming_leadership_20121220']['attested_on'] == '2012-12-20'
    assert claims['za_anc_ramaphosa_elected_president_54th_20171218']['attested_on'] == '2017-12-18'
    assert claims['za_anc_ramaphosa_new_anc_president_statement_20171220']['attested_on'] == '2017-12-20'
    assert claims['za_anc_nec_meeting_president_mandela_19910718']['attested_on'] == '1991-07-18'


class SouthAfricaAncPresidentsTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'south-africa.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.anc = next(o for o in cls.packet['organizations'] if o['id'] == ANC_ID)
        cls.role = cls.anc['roles'][0]
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (54, 73))
        order = [s['id'] for s in self.packet['sources']]
        # CLAUDE-C01-21 appends its Deputy President sources after these, pinned in its own test.
        self.assertEqual(order[EARLIER_SOURCE_COUNT:EARLIER_SOURCE_COUNT + len(NEW_SOURCES)], NEW_SOURCES)
        self.assertEqual(order[EARLIER_SOURCE_COUNT + len(NEW_SOURCES):], list(C01_21_RESPONSES))
        self.assertFalse([sid for sid in order[:EARLIER_SOURCE_COUNT] if sid.startswith('za_anc')])
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (53, 7))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = [cid for ids_ in HOLDER_CLAIMS for cid in ids_]
        self.assertEqual(len(holder_claims), 13)
        self.assertEqual(len(NEVER_HOLDER), len(set(NEVER_HOLDER)))
        self.assertFalse(set(holder_claims) & set(NEVER_HOLDER))
        self.assertEqual(set(holder_claims) | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        self.assertEqual(self.anc['claim_ids'], ['za_n2024_ballot_014', 'za_n2024_seats_014'] + self.new_claims)
        self.assertEqual(self.anc['sources'], ['za_iec_national_results_20240621', 'za_iec_national_seats_20240606']
                         + NEW_SOURCES)
        # At most ten observations, ZA-ANC-01..10, each with one holder observation.
        observations = re.findall(r'^### (ZA-ANC-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, REVIEW)
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, set(REVIEW))
        self.assertEqual([self.rows[ids_[0]]['review_observation'] for ids_ in HOLDER_CLAIMS], REVIEW)
        # Renamed or dropped per the checks: no stale id, dropped source or synthetic period field remains.
        for stale in STALE_IDS + DROPPED + ('attested_period',):
            self.assertNotIn(f'"{stale}"', self.raw)

    def test_holders_are_exactly_as_intended(self):
        anc_invariants(self.packet)
        for index, holder in enumerate(self.role['holder_claims']):
            self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
            self.assertRegex(holder['uncertainty'], r'No start', holder['name'])
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertIn(SURNAMES[holder['name']], self.claims[cid]['text'], cid)
                self.assertEqual((row['holder_name'], row['role_id'], row['role_title']), (holder['name'], ROLE, PRES), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                self.assertEqual(row['review_observation'], REVIEW[index], cid)
                self.assertIn('Dates the holder observation', self.claims[cid]['uncertainty'], cid)
        for cid in NEVER_HOLDER:
            self.assertNotIn(self.rows[cid]['event_kind'], HOLDER_KINDS | BOUNDARY_KINDS, cid)
        for cid in DEPUTY_AND_CHAIR:
            self.assertIn(self.rows[cid]['role_title'], {DEPUTY, CHAIR}, cid)
            self.assertIn('never a holder', self.claims[cid]['uncertainty'].lower(), cid)
        for cid in CONTINUATION:
            self.assertEqual(self.rows[cid]['event_kind'], 'in_office_continuation_attestation', cid)
            self.assertIn('never a holder', self.claims[cid]['uncertainty'], cid)
        # Holder names are normalised; the printed forms stay in the claim text.
        self.assertTrue({r['holder_name'] for r in self.rows.values()} <= set(SURNAMES) | {None})
        self.assertIn("'OR Tambo'", self.claims['za_anc_mandela_greets_president_tambo_19910702']['uncertainty'])
        self.assertIn("'Zuma, Jacob'", self.claims['za_anc_nec_members_elected_53rd_officials_2012']['uncertainty'])
        self.assertIn('Cyril Matamela Ramaphosa',
                      self.claims['za_anc_54th_report_officials_president_ramaphosa_201712']['text'])
        # Every row but the deputy and chair rows is titled as the President's office.
        for cid, row in self.rows.items():
            if cid not in DEPUTY_AND_CHAIR:
                self.assertEqual(row['role_title'], PRES, cid)

    def test_no_start_or_end_is_stated_or_inferred(self):
        claims = self.claims
        for cid in UNDATED:
            self.assertIn('no structured date', claims[cid]['uncertainty'].lower(), cid)
        # Evidence that is recorded but never used as a boundary, each saying why.
        self.assertIn("'were held on the second day of conference'",
                      claims['za_anc_mayibuye_office_bearers_elected_second_day_1997']['text'])
        self.assertIn('is not stored', claims['za_anc_mayibuye_office_bearers_elected_second_day_1997']['uncertainty'])
        self.assertIn('take leave of the Presidency',
                      claims['za_anc_conf50_declaration_salutes_outgoing_president_mandela']['text'])
        self.assertIn('never an until', claims['za_anc_conf50_declaration_salutes_outgoing_president_mandela']['uncertainty'])
        self.assertIn("'We then released him'", claims['za_anc_mandela_recounts_tambo_release_199107']['text'])
        self.assertIn("'as I told you the other day'", claims['za_anc_mandela_recounts_tambo_release_199107']['text'])
        self.assertIn("'The leadership we have elected here today'", claims['za_anc_mandela_accepts_presidency_199107']['text'])
        self.assertIn('no end is inferred', claims['za_anc_zuma_succeeding_mbeki_20071220']['uncertainty'])
        self.assertIn('Cabinet', claims['za_anc_mandela_leads_incoming_nec_19941222']['uncertainty'])
        self.assertIn('on or before the evening of 14 May 2026',
                      claims['za_anc_nec_reaffirms_ramaphosa_anc_president_20260515']['uncertainty'])
        self.assertIn('concluded on 5 January 2023', claims['za_anc_55th_declaration_nasrec_and_imvelo_sessions']['text'])
        # One rule for unqualified 'President' headings: a political report or closing address to Conference is the party
        # President's act, and the state office is never read from it.
        for cid in ('za_anc_mandela_presents_nec_political_report_19941217', 'za_anc_mandela_political_report_19971216',
                    'za_anc_mbeki_opening_political_report_20021216', 'za_anc_zuma_political_report_as_president_20121216',
                    'za_anc_zuma_closing_remarks_incoming_leadership_20121220'):
            self.assertIn('the rule applied to every such report in this packet', claims[cid]['uncertainty'], cid)
        scope = self.role['scope_note']
        for phrase in ('no za_presidency claim or source feeds this role', 'none of its claims feeds za_presidency',
                       'Do not fill the interval', "infer an outgoing holder's last day from a successor's election",
                       'never holders', 'procedure only, never a date'):
            self.assertIn(phrase, scope)
        self.assertTrue(self.anc['coverage']['unresolved'][-1].startswith('ANC Presidents 1990-2026 (CLAUDE-C01-16)'))
        self.assertEqual(self.anc['coverage']['unresolved'][:-1], [
            'Reconcile legal registration, precise organization identity, renames, alliances, mergers and splits against original records.',
            'Research all independently dated party, parliamentary and executive leadership roles before character or succession eligibility.',
            'No game-party mapping is asserted. An electoral entry is not automatically a parliamentary caucus, coalition or umbrella affiliation.'])
        unresolved = self.packet['coverage']['unresolved']
        self.assertEqual(sum('CLAUDE-C01-16' in u for u in unresolved), 1)
        self.assertTrue(unresolved[-1].startswith('ANC Presidents 1990-2026 (CLAUDE-C01-16'))
        self.assertEqual(sum('CLAUDE-C01-09' in u for u in unresolved), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url'], extract['original_url']),
                             (sid, source['url'], source['original_url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual((extract['accessed_date'], source['accessed_date']), ('2026-09-24', '2026-09-24'))
            self.assertLessEqual(source['published_date'] or '', research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertRegex(extract['source_response_sha1_base32'], r'^[A-Z2-7]{32}$')
            self.assertIn('no Accept-Encoding request header and no automatic decoding', extract['fetch_recipe'])
            if sid in GZIP:
                self.assertEqual(extract['source_response_content_encoding'], 'gzip')
                self.assertEqual((extract['decoded_response_bytes'], extract['decoded_response_sha256']), GZIP[sid])
                self.assertIn('gzip-encoded', source['scope_note'])
            else:
                self.assertEqual(extract['source_response_content_encoding'], 'identity')
                self.assertNotIn('decoded_response_sha256', extract)
            self.assertRegex(extract['stability_check'], r'again|twice')
            self.assertIn('not checked into this repository', extract['provenance_note'])
            self.assertIn('derived factual extract', extract['provenance_note'])
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'] and source['scope_note'] and source['publisher'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/south-africa-anc'))
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
                self.assertEqual((row['observation_id'], row['role_id']), (ANC_ID, ROLE))
                self.assertNotIn('name', row)
                self.assertIn(row['review_observation'], REVIEW)
                self.assertEqual('printed_range' in row, row['attested_on'] is None and 'printed_range' in row)
            # Every source is a raw Internet Archive capture made before the cutoff.
            match = ARCHIVED_URL.match(source['url'])
            self.assertTrue(match, sid)
            stamp = match.group(1)
            self.assertEqual(stamp, ARCHIVED[sid])
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertEqual(source['original_url'], match.group(2).replace('anc.org.za:80/', 'anc.org.za/'))
            self.assertNotIn(':80', source['original_url'])
        self.assertEqual({cid: (row['attested_on'], row['event_kind']) for cid, row in self.rows.items()}, EVENTS)
        self.assertEqual({cid for cid, row in self.rows.items() if row['attested_on'] is None}, set(UNDATED))
        # Rows whose source prints a range or day keep it as text only.
        self.assertEqual(self.rows['za_anc1912_heading_mandela_48th_closing_19910706']['printed_range'], '6 July 1991')
        self.assertEqual(self.rows['za_anc1912_49th_nec_list_heading_19941220']['printed_range'], '20 December 1994')
        self.assertEqual(self.rows['za_anc_mayibuye_office_bearers_elected_second_day_1997']['printed_range'],
                         'the second day of conference')

    def test_secondary_leads_and_per_request_pages_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            self.assertEqual(urlsplit(url).hostname, 'web.archive.org', sid)
            self.assertIsNone(PER_REQUEST_URL.search(url), sid)
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, url, sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'sahistory', 'britannica', 'news24', 'wikipedia-outlinks'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('sahistory.org.za', 'anc1912.org.za/49th-national-conference-1994', '19981205082127',
                       'officials/current', 'id=10006', '20260825152506', '20071217172822', 'programme.pdf',
                       '20260831192551'):
            self.assertIn(marker, leads, marker)
        for marker in ('sahistory.org.za', 'anc1912.org.za/49th-national-conference-1994', 'officials/current',
                       'id=10006', '20260825152506', '20071217172822', '20260831192551'):
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

        def anc(packet):
            return next(o for o in packet['organizations'] if o['id'] == ANC_ID)

        def role(packet):
            return anc(packet)['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def president_role(packet):
            return packet['institutions'][0]['roles'][0]

        validator_cases = [
            (lambda p: source(p, 'za_anc_54th_conference_report')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'za_anc_jan8_statement_19900108')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: holder(p, 9).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'za_anc_nec_reaffirms_ramaphosa_anc_president_20260515').update(attested_on='2026-09-08'),
             'exceeds cutoff'),
            (lambda p: holder(p, 3)['claim_ids'].append('za_anc_mayibuye_office_bearers_elected_second_day_1997'),
             'cited source'),
            (lambda p: role(p)['claim_ids'].append('za_anc_does_not_exist'), 'Unknown'),
            (lambda p: anc(p).update(represented_party_ids=['SouthAfrica/guessed_anc']), 'foreign represented party'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        invariant_cases = [
            ('successor start used as an end (Tambo)', lambda p: holder(p, 0).update(until='1991-07-18')),
            ('successor start used as an end (Mbeki 2002)', lambda p: holder(p, 4).update(until='2007-12-20')),
            ('successor start used as an end (Zuma 2012)', lambda p: holder(p, 6).update(until='2017-12-20')),
            ('re-election used as an end (Mandela 1991)', lambda p: holder(p, 1).update(until='1994-12-22')),
            ('election date used as start (Ramaphosa 2017)', lambda p: holder(p, 7).update({'from': '2017-12-18'})),
            ('election reference used as start (Zuma 2012)', lambda p: holder(p, 6).update({'from': '2012-12-18'})),
            ('derived conference day used as start (Mbeki 1997)', lambda p: holder(p, 3).update({'from': '1997-12-17'})),
            ('declaration date used as start (Ramaphosa 2023)', lambda p: holder(p, 8).update({'from': '2023-01-05'})),
            ('election date used as observation (Zuma 2012)', lambda p: holder(p, 6).update(attested_on='2012-12-18')),
            ('result publication used as observation (Zuma 2007)', lambda p: holder(p, 5).update(attested_on='2007-12-19')),
            ('in-office observation used as start (Mandela 1994)', lambda p: holder(p, 2).update({'from': '1994-12-22'})),
            ('deputy service added as a holder', lambda p: role(p)['holder_claims'].insert(1, {
                'name': 'Nelson Mandela', 'attested_on': '1991-07-02', 'from': None, 'until': None,
                'sources': ['za_anc_tambo_48th_opening_19910702'],
                'claim_ids': ['za_anc_tambo_salutes_deputy_president_mandela_19910702']})),
            ('continuation attestation added as a holder', lambda p: role(p)['holder_claims'].insert(5, {
                'name': 'Thabo Mbeki', 'attested_on': '2007-12-16', 'from': None, 'until': None,
                'sources': ['za_anc_mbeki_political_report_20071216'], 'claim_ids': ['za_anc_mbeki_political_report_20071216']})),
            ('continuation claim cited by a holder', lambda p: (
                holder(p, 4)['claim_ids'].append('za_anc_mbeki_political_report_20071216'),
                holder(p, 4)['sources'].append('za_anc_mbeki_political_report_20071216'))),
            ('election claim cited by a holder', lambda p: (
                holder(p, 7)['claim_ids'].append('za_anc_ramaphosa_elected_president_54th_20171218'),
                holder(p, 7)['sources'].append('za_anc_officials_ramaphosa_profile_20171222'))),
            ('presidency holder added to the ANC role', lambda p: role(p)['holder_claims'].append(
                copy.deepcopy(president_role(p)['holder_claims'][8]))),
            ('ANC holder added to the Presidency', lambda p: president_role(p)['holder_claims'].append(
                copy.deepcopy(holder(p, 9)))),
            ('presidency claim moved onto the ANC role', lambda p: (
                role(p)['claim_ids'].append('za_ramaphosa_president_elect_20240614'),
                role(p)['sources'].append('za_parliament_president_elect_20240614'))),
            ('ANC claim moved onto the Presidency', lambda p: (
                president_role(p)['claim_ids'].append('za_anc_nec_reaffirms_ramaphosa_anc_president_20260515'),
                president_role(p)['sources'].append('za_anc_sg_statement_special_nec_20260515'))),
            ('ANC role copied to another party', lambda p: p['organizations'][0]['roles'].append(copy.deepcopy(role(p)))),
            ('second ANC role', lambda p: anc(p)['roles'].append(dict(copy.deepcopy(role(p)), id='za_anc_deputy_president'))),
            ('ANC role removed', lambda p: anc(p)['roles'].clear()),
            ('ANC lifecycle given a start', lambda p: anc(p)['lifecycle'].update({'from': '1912-01-08'})),
            ('month-only list given a date', lambda p: claim(p, 'za_anc_49th_conference_elects_mandela_president_199412').update(
                attested_on='1994-12-20')),
            ('holder order changed', lambda p: role(p)['holder_claims'].reverse()),
        ]
        anc_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                anc_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = dict.fromkeys([f'{n:02d}' for n in range(1, 10)], 'Accepted in part')
        decisions['10'] = 'Accepted'
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| ZA-ANC-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 12)] + [f'B{n}' for n in range(1, 17)] + [f'C{n}' for n in range(1, 19)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Applied in part|Resolved by removal|Declined)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('6e9a489b', '82a23f9d', 'ffe54b02', 'claude/c01-za-09', 'research-index.json',
                     'test_south_africa_research_s10h.py', 'test_south_africa_heads_of_state_c01_09.py',
                     'test_campaign_census'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'south-africa-anc-presidents-1990-2026-16.md', 'claude/c01-za-16', '6e9a489b',
                     'claude/c01-za-09', 'test_south_africa_anc_presidents_c01_16.py'):
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
