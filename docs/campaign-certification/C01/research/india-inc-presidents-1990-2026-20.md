# Indian National Congress presidents 20: the party office, 1990-2026

Packet: **CLAUDE-C01-20**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-20`, **stacked on CLAUDE-C01-15** (`claude/c01-in-15` at
`dd58a610`, ready for review and not yet integrated), which is itself stacked on CLAUDE-C01-11 (`claude/c01-in-11` at
`538920f1`) and so based on `codex/campaign-certification` at `ffe54b02`; claim commit `d19f756b`. Research access:
25 September 2026 (UTC). The historical cutoff stays **7 September 2026**.

This packet adds one party role to [india.json](india.json): `in_inc_president` ("President of the Indian National
Congress", kind `party_leader`) on the existing recognition observation `in_eci_20240323_np_05` (Indian National Congress).
The observation's identity, recognition row, lifecycle (`unresearched`), coverage status (`reporting_identity_only`) and
empty game mapping are unchanged; only its sources, claim ids, role list and one coverage note grow. The packet adds 73
sources and 113 claims, appended after the CLAUDE-C01-15 sources, ten holder observations with no start or end, a role
scope note and one bounded note in the packet coverage. It adds no organization, institution, game mapping, lifespan,
portrait or avatar, and it changes nothing that CLAUDE-C01-11 or CLAUDE-C01-15 added: the thirteen prime-minister and eight
president holders are unchanged, and no existing extract is edited. The party office and the `in_prime_minister` and
`in_presidency` institutions stay separate both ways. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1990-1997; B: 1998-2017; C: 2019-2026 and the stated ends), and each part was
checked independently before this packet was written. Every checker defect is applied or its outcome explained (see
[Checker defects](#checker-defects)), and every missing primary record the checks confirmed is imported or declined with a
reason.

## Outcome

| ID | Question | Decision |
|---|---|---|
| INC-PRES-01 | The Congress President when the period opens (Rajiv Gandhi), and the vacancy after his death only as source-stated facts | **Accepted in part:** observed on 29 Aug 1990 (Rajya Sabha, in English, official debates store); the Secretariat's item heading of 4 Mar 1991 and a member's words of 5 Mar 1991 attest him again; his death on 21 May 1991 is stated by the Rajya Sabha Chairman's resolution and the party's own page; posthumous references of 3 and 4 Jun 1991 and the Prime Minister's reference of 4 Jun 1991 to an unnamed Congress President; no record attests 1 Jan 1990 itself or states a vacancy, an acting President or the day the office ended |
| INC-PRES-02 | 1991-1992: P. V. Narasimha Rao's selection and any AICC confirmation | **Accepted in part:** no record of the 1991 selection or of an AICC confirmation at Tirupati; a member's translated recollection of 31 Jul 1991; the Lok Sabha Secretariat's retrospective span '29 May'91 - Sept.1996'; the party's retrospective session list (79th Session, Tirupati, 14-16 Apr 1992) and biography; observed on 15 Jul 1996 by the Leader of the House |
| INC-PRES-03 | 1996: Sitaram Kesri's selection | **Accepted in part:** Rao still 'our Congress President' on 10 Sep 1996; the party's time line and history give the resignation and the selection the year 1996 only; Kesri observed on 11 Apr 1997 in the Lok Sabha, where the Prime Minister recalls, without a date, the day he became President; the day of selection and the selecting body are not found |
| INC-PRES-04 | 1998: Sonia Gandhi's selection by the Working Committee and its ratification | **Accepted in part:** no INC record of the Working Committee's March 1998 selection; unnamed remarks at the AICC session of 6 Apr 1998 ('You have elected me President'); the INC's 2015 history calls that day her ratification (a recollection); later INC texts say March or April 1998; observed on 15 May 1999 by the INC website's heading of her letter |
| INC-PRES-05 | Sonia Gandhi's organisational elections and extensions from 2000 | **Accepted in part:** the Central Election Authority's rescheduling of 5 Oct 2000; her election and its declaration on 15 Nov 2000; observed on 26 Nov 2000 (the Working Committee made a Steering Committee); the elections of 28 May 2005 and 3 Sep 2010 (with a signed certificate) are claims only; the 2015 and 2016 extensions are not found; observed on 29 Apr 2017 |
| INC-PRES-06 | 2017: Rahul Gandhi's election and assumption of the office | **Accepted in part:** Working Committee schedule approvals of 6 Jun and 20 Nov 2017; declaration of 11 Dec; President-elect on 13 Dec; certificate ceremony and acceptance on 16 Dec, when the INC styles him the new President (observed); the INC's statement that he assumed the office that day is a recollection captured in 2026; no contemporaneous stated assumption and no certificate found |
| INC-PRES-07 | 2019: Rahul Gandhi's resignation and Sonia Gandhi's appointment as interim President | **Accepted in part:** observed on 25 May 2019, the day the Working Committee rejected his offer to resign; his statement that he had resigned (published 3 Jul); the resolutions and briefings of 10 Aug (continue, declined, Interim President requested and accepted); interim service attested on 22 Aug 2019, continued on 24 Aug 2020 and called interim by her on 16 Oct 2021, all claims only; no day on which his office ended and no acceptance of the resignation found |
| INC-PRES-08 | 2022: the election result and Mallikarjun Kharge's assumption of the office | **Accepted in part:** Working Committee schedules of 16 Oct 2021 and 28 Aug 2022; notification 22 Sep; scrutiny 1 Oct; final list 8 Oct; poll 17 Oct; count and declaration 19 Oct; President-elect 19 Oct; observed on 26 Oct 2022 by three same-day stylings; the day's handover statements are claims and no record says he took charge that day; no AICC ratification found |
| INC-PRES-09 | Any stated end of a presidency, where a source states it | **Unresolved:** no primary source reviewed states the day any President's office ended; resignations (1996 retrospective, 1999 withdrawn, 2019 stated without a day), a farewell (16 Dec 2017), predecessor references, the handover that ended interim service (26 Oct 2022) and a retrospective plenary resolution (25 Feb 2023) are claims only |
| INC-PRES-10 | An attestation by the party of its President before the cutoff | **Accepted:** the INC styles Mallikarjun Kharge 'Congress President' on 3 Sep 2026; an unnamed 'Hon'ble Congress President' acts on 7 Sep 2026 (a claim only) |

The resulting holder observations of `in_inc_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Rajiv Gandhi | 1990-08-29 | null | null | Rajya Sabha, 29 Aug 1990: Shrimati Jayanthi Natarajan, 'the former Prime Minister and the Congress President, Shri Rajiv Gandhi' (in English, official debates store) |
| P. V. Narasimha Rao | 1996-07-15 | null | null | Rajya Sabha, 15 Jul 1996: the Leader of the House, 'the President of the Congress Party, Shri Narasimha Rao' |
| Sitaram Kesri | 1997-04-11 | null | null | Lok Sabha, 11 Apr 1997: Shri N.K. Premchandran, 'the Congress Working Committee President, Shri Sitaram Kesri' |
| Sonia Gandhi | 1999-05-15 | null | null | the INC website's heading 'the resignation letter written by the Congress President Smt. Sonia Gandhi', dated by the site's index |
| Sonia Gandhi | 2000-11-26 | null | null | Congress Sandesh, December 2000: 'The Congress president, Mrs. Sonia Gandhi' made the Working Committee a Steering Committee (stated 26 November) |
| Sonia Gandhi | 2017-04-29 | null | null | AICC press release: 'The Congress President Smt. Sonia Gandhi has approved the composition of the Central Organisational Election Authority' |
| Rahul Gandhi | 2017-12-16 | null | null | the Hindi title of the INC's release of 16 Dec 2017: 'नये अध्यक्ष श्री राहुल गांधी' (the new President Shri Rahul Gandhi) |
| Rahul Gandhi | 2019-05-25 | null | null | the Working Committee resolution of 25 May 2019: 'Congress President, Shri Rahul Gandhi' |
| Mallikarjun Kharge | 2022-10-26 | null | null | the INC's heading of his address, the General Secretary's salutation and the Steering Committee list of 26 Oct 2022 |
| Mallikarjun Kharge | 2026-09-03 | null | null | the INC's highlights of 3 Sep 2026: 'Shri Mallikarjun Kharge, Congress President' |

### How a start and an end are decided

One rule covers all three parts, taken from the stacked India packets and, for a party office, the ANC presidents packet
(CLAUDE-C01-16). A holder has `from` only where a source states the day the office was assumed or took effect, and
`until` only where a source states the day it ended; otherwise the holder is dated by `attested_on`, from a same-day
in-office attestation that names the holder and styles the office. No record reviewed states either day, so every holder
is an observation with no start or end, and each cites only in-office attestations of its own day. The observations are
separate (the ANC model), not folded into one tenure (the model of the presidency, which has stated starts), because no
holder has a start (check C5).

Kept as claims that never feed a holder: Working Committee decisions and schedule approvals, AICC sessions and the
recollection of a ratification, the Central Election Authority's notifications, scrutiny, final lists, polls, counts and
declarations, certificates and their presentation, President-elect stylings, acceptances, handover statements,
resignations, their rejection, maintenance, withdrawal or recording, farewells, predecessor references, deaths and
posthumous references, the 2019-2022 interim arrangement, later attestations of a holder already observed
(`in_office_continuation_attestation`), retrospective spans, lists, biographies and histories, and records that name
nobody. Parliament records are used only where they record the party office; statements in the Houses by members and by
the Leader of the House date three holders because no party record of 1990-1997 that names the President was found.

Rulings on the questions the parts and checks left open:

- **A death is not an end.** Rajiv Gandhi's death on 21 May 1991 is stated by the Rajya Sabha Chairman's resolution and
  the party's page, but neither states the day the party office ended or a vacancy; the death is its own claim.
- **Kharge's start is declined.** On 26 October 2022 the resolution of thanks read by Shri Ajay Maken says the post is
  handed over 'today', and Sonia Gandhi and Kharge speak of the handover; these are handover statements, which the ANC
  packet keeps as claims, and no record says he took charge (karyabhar sambhala) that day. He is observed that day with no
  start (check C4).
- **Rahul Gandhi's 2017 start is declined.** The INC's statement that he 'assumed the Presidency ... on December 16, 2017'
  is on a page captured in 2026; CLAUDE-C01-15 check C1 ruled that a later recollection of an assumption never dates a
  holder. He is observed on 16 December 2017 by the INC's same-day Hindi title (check B1).
- **Sonia Gandhi's 2017 end is declined.** Her farewell of 16 December 2017 ('आज इस ज़िम्मेदारी को छोड़ते हुए', today,
  relinquishing this responsibility) is her own speech, not a record fixing the transfer; farewells are never an until in
  the stacked packets (check B2).
- **Sonia Gandhi's 1998 observation is dated 15 May 1999.** Every INC record of 1998 found styles 'the Congress
  President' without a name, so the rows name nobody; the earliest INC record that both names her and styles her Congress
  President is the website's heading of her letter of 15 May 1999 (check B4).
- **No exception for 2005 and 2010.** The AICC pages and the certificate record elections and declarations, which never
  date a holder, and no same-day in-office styling was found; her 2017 observation is the next (check B3).
- **Interim service is claims only.** The Working Committee asked her on 10 August 2019 to act as Interim President
  pending an AICC election, she accepted, and on 16 October 2021 she called herself 'interim Congress President'; the
  service from 2019 to 26 October 2022 is recorded as claims and never as a holder (check C12).

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. "Observed" marks a
holder's `attested_on` claim; every other claim never feeds a holder.

| Date | Events | Claims |
|---|---|---|
| 29 Aug 1990 | Rajiv Gandhi: in office | `in_rs_natarajan_rajiv_gandhi_congress_president_19900829` (observed) |
| 4 Mar 1991 | Rajiv Gandhi: in office again | `in_rs_heading_surveillance_on_congress_president_rajiv_gandhi_19910304` (claim) |
| 5 Mar 1991 | Rajiv Gandhi: in office again | `in_rs_goswami_rajiv_gandhi_congress_president_19910305` (claim) |
| 21 May 1991 | Rajiv Gandhi: death stated; Rajiv Gandhi: death stated | `in_inc_inspiration_rajiv_gandhi_death_19910521` (claim), `in_rs_chairman_resolution_demise_rajiv_gandhi_19910521` (claim) |
| 3 Jun 1991 | Rajiv Gandhi: posthumous reference; Rajiv Gandhi: posthumous reference | `in_rs_leader_of_house_rajiv_gandhi_president_congress_party_19910603` (claim), `in_rs_arora_rajiv_gandhi_president_inc_i_19910603` (claim) |
| 4 Jun 1991 | Rajiv Gandhi: posthumous reference; unnamed: unnamed Congress President referred to | `in_rs_chavan_rajiv_gandhi_was_congress_president_19910604` (claim), `in_rs_pm_letter_from_unnamed_congress_president_19910604` (claim) |
| 15 Jul 1996 | Rao: in office | `in_rs_gujral_rao_president_of_congress_party_19960715` (observed) |
| 10 Sep 1996 | Rao: in office again | `in_rs_narayanasamy_rao_our_congress_president_19960910` (claim) |
| 11 Apr 1997 | Kesri: in office | `in_ls_premchandran_kesri_cwc_president_19970411` (observed) |
| 6 Apr 1998 | unnamed: AICC session: "You have elected me"; unnamed: acceptance; unnamed: index entry; Sonia Gandhi: ratification recalled | `in_aicc_session_remarks_you_have_elected_me_president_19980406` (claim), `in_aicc_session_remarks_accepts_office_19980406` (claim), `in_inc_speech_index_aicc_opening_remarks_19980406` (claim), `in_inc_history_sonia_ratified_at_aicc_session_19980406` (claim) |
| 15 May 1999 | Sonia Gandhi: resignation tendered; Sonia Gandhi: in office; Sonia Gandhi: resignation tendered | `in_inc_speech_index_resignation_letter_19990515` (claim), `in_inc_site_heading_congress_president_sonia_gandhi_19990515` (observed), `in_sonia_resignation_letter_to_cwc_19990515` (claim) |
| 25 May 1999 | unnamed: resignation withdrawn; unnamed: resignation withdrawn | `in_inc_speech_index_withdrawal_of_resignation_19990525` (claim), `in_sonia_withdrawal_of_resignation_speech_19990525` (claim) |
| 5 Oct 2000 | unnamed: election schedule | `in_cea_mirdha_reschedules_president_election_20001005` (claim) |
| 15 Nov 2000 | Sonia Gandhi: election result; Sonia Gandhi: declaration | `in_sonia_elected_congress_president_20001115` (claim), `in_cea_mirdha_declares_sonia_president_certificate_20001115` (claim) |
| 26 Nov 2000 | Sonia Gandhi: in office | `in_sonia_converts_cwc_into_steering_committee_20001126` (observed) |
| 28 Dec 2000 | Sonia Gandhi: in office again | `in_foundation_day_resolution_sonia_re_elected_20001228` (claim) |
| 28 May 2005 | Sonia Gandhi: election result; Sonia Gandhi: declaration | `in_sonia_elected_congress_president_20050528` (claim), `in_cea_fernandes_declares_sonia_duly_elected_20050528` (claim) |
| 3 Sep 2010 | Sonia Gandhi: election result; Sonia Gandhi: declaration; Sonia Gandhi: certificate of election | `in_sonia_unanimously_elected_congress_president_20100903` (claim), `in_cea_fernandes_declares_and_hands_certificate_20100903` (claim), `in_certificate_sonia_duly_elected_unopposed_20100903` (claim) |
| 29 Apr 2017 | Sonia Gandhi: in office | `in_congress_president_sonia_approves_cea_composition_20170429` (observed) |
| 6 Jun 2017 | unnamed: schedule put to the Working Committee; unnamed: Working Committee schedule approval; unnamed: election schedule | `in_congress_president_places_schedule_before_cwc_20170606` (claim), `in_cwc_approves_org_election_schedule_20170606` (claim), `in_cea_programme_congress_president_election_window_20170606` (claim) |
| 20 Nov 2017 | unnamed: Working Committee schedule approval | `in_cwc_approves_congress_president_election_schedule_20171120` (claim) |
| 11 Dec 2017 | Rahul Gandhi: declaration; Rahul Gandhi: declaration reported; unnamed: certificate hand-over scheduled; Sonia Gandhi: in office again | `in_cea_declares_rahul_gandhi_elected_president_20171211` (claim), `in_cea_briefing_rahul_declared_elected_20171211` (claim), `in_cea_briefing_certificate_handover_scheduled_20171211` (claim), `in_cea_briefing_sonia_styled_congress_president_20171211` (claim) |
| 13 Dec 2017 | Rahul Gandhi: President-elect | `in_rahul_gandhi_styled_president_elect_20171213` (claim) |
| 16 Dec 2017 | Rahul Gandhi: certificate presented; Rahul Gandhi: acceptance; Rahul Gandhi: in office; Sonia Gandhi: farewell; Rahul Gandhi: assumption recalled | `in_certificate_presented_to_newly_elected_president_20171216` (claim), `in_rahul_gandhi_accepts_position_20171216` (claim), `in_rahul_gandhi_styled_new_president_20171216` (observed), `in_sonia_last_address_as_congress_president_20171216` (claim), `in_inc_rahul_gandhi_assumption_recalled_20171216` (claim) |
| 22 Dec 2017 | Sonia Gandhi: predecessor reference; Rahul Gandhi: in office again; Sonia Gandhi: predecessor reference | `in_cwc_resolution_thanks_outgoing_president_sonia_20171222` (claim), `in_rahul_gandhi_chairs_cwc_as_congress_president_20171222` (claim), `in_rahul_gandhi_thanks_former_congress_president_20171222` (claim) |
| 25 May 2019 | Rahul Gandhi: resignation offered; Rahul Gandhi: Working Committee rejects the resignation; Rahul Gandhi: in office | `in_rahul_gandhi_offers_resignation_cwc_20190525` (claim), `in_cwc_rejects_resignation_offer_20190525` (claim), `in_rahul_gandhi_attested_congress_president_20190525` (observed) |
| 3 Jul 2019 | Rahul Gandhi: resignation statement published | `in_inc_lists_rahul_gandhi_resignation_statement_20190703` (claim) |
| 10 Aug 2019 | Rahul Gandhi: members ask him to continue; Rahul Gandhi: resignation maintained; Rahul Gandhi: resignation pending; Rahul Gandhi: in office again; Rahul Gandhi: Working Committee resolves he should continue; Rahul Gandhi: resignation maintained; Sonia Gandhi: Interim President requested; Rahul Gandhi: stepping down recorded; Sonia Gandhi: interim request accepted | `in_cwc_members_ask_rahul_gandhi_to_continue_20190810` (claim), `in_rahul_gandhi_says_resignation_decision_final_20190810` (claim), `in_resignation_still_under_cwc_consideration_20190810` (claim), `in_rahul_gandhi_styled_congress_president_20190810` (claim), `in_cwc_resolves_rahul_gandhi_continue_20190810` (claim), `in_rahul_gandhi_declines_to_withdraw_resignation_20190810` (claim), `in_cwc_requests_sonia_gandhi_interim_president_20190810` (claim), `in_cwc_first_resolution_records_stepping_down_20190810` (claim), `in_sonia_gandhi_accepts_interim_request_20190810` (claim) |
| 22 Aug 2019 | Sonia Gandhi: interim service | `in_sonia_gandhi_styled_president_inc_20190822` (claim) |
| 24 Aug 2020 | Sonia Gandhi: interim service continued; Sonia Gandhi: interim service continued | `in_cwc_requests_sonia_gandhi_continue_20200824` (claim), `in_cwc_resolution_read_out_continue_20200824` (claim) |
| 16 Oct 2021 | Sonia Gandhi: interim service; unnamed: Working Committee schedule approval | `in_sonia_gandhi_interim_congress_president_self_description_20211016` (claim), `in_cwc_approves_schedule_aicc_president_election_20211016` (claim) |
| 28 Aug 2022 | unnamed: Working Committee schedule approval | `in_cwc_approves_final_schedule_20220828` (claim) |
| 22 Sep 2022 | unnamed: election notification | `in_cea_notification_congress_president_election_20220922` (claim) |
| 1 Oct 2022 | unnamed: scrutiny | `in_cea_scrutiny_two_candidates_20221001` (claim) |
| 8 Oct 2022 | unnamed: final list of candidates | `in_cea_final_list_two_candidates_20221008` (claim) |
| 17 Oct 2022 | unnamed: poll; unnamed: poll recited | `in_cea_poll_held_20221017` (claim), `in_cea_declaration_recites_poll_20221017` (claim) |
| 19 Oct 2022 | Kharge: count; Kharge: declaration; Kharge: declaration reported; Kharge: certificate announced; Kharge: President-elect; Kharge: charge announced for the 26th | `in_inc_president_count_20221019` (claim), `in_cea_declares_kharge_elected_president_20221019` (claim), `in_mistry_announces_kharge_elected_briefing_20221019` (claim), `in_certificate_to_be_given_20221019` (claim), `in_kharge_styled_president_elect_20221019` (claim), `in_kharge_charge_on_26th_announced_20221019` (claim) |
| 26 Oct 2022 | Sonia Gandhi: handover statement; Kharge: handover statement; Sonia Gandhi: interim service ends (stated); Kharge: handover statement; Kharge: in office; Kharge: handover statement; Kharge: in office; Sonia Gandhi: predecessor reference; unnamed: act of an unnamed Congress President; Kharge: in office | `in_sonia_gandhi_handing_over_post_20221026` (claim), `in_kharge_receives_post_of_president_20221026` (claim), `in_sonia_gandhi_relieved_of_responsibility_today_20221026` (claim), `in_sonia_gandhi_says_responsibility_now_on_kharge_20221026` (claim), `in_kharge_styled_president_inc_20221026` (observed), `in_kharge_says_reached_this_post_today_20221026` (claim), `in_venugopal_greets_congress_president_kharge_20221026` (observed), `in_venugopal_outgoing_president_sonia_gandhi_20221026` (claim), `in_congress_president_constitutes_steering_committee_20221026` (claim), `in_steering_committee_list_kharge_congress_president_20221026` (observed) |
| 25 Feb 2023 | Sonia Gandhi: plenary resolution (retrospective) | `in_plenary_resolution_former_president_sonia_gandhi_20230225` (claim) |
| 3 Sep 2026 | Kharge: in office | `in_kharge_congress_president_attested_20260903` (observed) |
| 7 Sep 2026 | unnamed: act of an unnamed Congress President | `in_congress_president_approves_dcc_odisha_20260907` (claim) |
| undated | Rao: selection recalled in debate; Rao: retrospective span; Rao: retrospective biography; Rao: session list (retrospective); Rao: session list (retrospective); Rao: session list (retrospective); Kesri: session list (retrospective); Rao: resignation recalled; Kesri: selection recalled; Rao: resignation recalled; Kesri: selection recalled; Kesri: retrospective span; Kesri: selection recalled in debate; Kesri: retrospective biography; Sonia Gandhi: retrospective span; Sonia Gandhi: selection recalled; Sonia Gandhi: resignation recalled; Sonia Gandhi: Working Committee decision recalled; Sonia Gandhi: selection recalled; Rahul Gandhi: resignation recalled; Rahul Gandhi: resignation stated (undated) | `in_rs_bhattacharjee_recalls_rao_became_congress_president_1991`, `in_ls_bio_span_rao_inc_president_19910529_199609`, `in_inc_bio_rao_presided_tirupati_session_1992`, `in_inc_sessions_79th_tirupati_rao_1992`, `in_inc_sessions_special_surajkund_rao_1993`, `in_inc_sessions_special_new_delhi_rao_1994`, `in_inc_sessions_80th_calcutta_kesri_1997`, `in_aicc_timeline_rao_resigns_presidentship_1996`, `in_aicc_timeline_kesri_chosen_president_1996`, `in_inc_history_rao_resigned_presidentship_1996`, `in_inc_history_kesri_elected_president_1996`, `in_rs_sketch_span_kesri_inc_president_1996_98`, `in_ls_deve_gowda_recalls_kesri_became_president_1997`, `in_inc_bio_kesri_presided_calcutta_session_1997`, `in_inc_profile_span_sonia_president_from_march_1998`, `in_sandesh_directory_sonia_elected_president_april_1998`, `in_inc_history_sonia_resigned_after_letter_1999`, `in_inc_history_cwc_reiterated_faith_1999`, `in_inc_past_president_page_sonia_became_president_april_1998`, `in_inc_rahul_gandhi_stepped_down_recalled_201905`, `in_rahul_gandhi_states_he_has_resigned_2019` (no structured date) |

Date conventions follow the stacked India packets: `attested_on` is the day of the observed event as the source dates it, a
page's issue date is `published_date`, and a retrospective statement that recalls a day is dated by that day; spans, year- or
month-only statements, undated lists and undated recollections carry no structured date.

## Observations

### INC-PRES-01 — The holder when the period opens

Evidence: in the Rajya Sabha on 29 August 1990 Shrimati Jayanthi Natarajan speaks, in English, of the Accord 'that the
former Prime Minister and the Congress President, Shri Rajiv Gandhi, had entered into'
(`in_rs_natarajan_rajiv_gandhi_congress_president_19900829`). The Secretariat heads an item of 4 March 1991 'Alleged
Surveillance on Congress President Shri Rajiv Gandhi' (`in_rs_heading_surveillance_on_congress_president_rajiv_gandhi_19910304`),
and Shri Dinesh Goswami speaks of 'the powerful Congress President' on 5 March 1991
(`in_rs_goswami_rajiv_gandhi_congress_president_19910305`). His death on 21 May 1991 is stated by the Rajya Sabha
Chairman's resolution of 3 June 1991 (`in_rs_chairman_resolution_demise_rajiv_gandhi_19910521`) and by the party's page
(`in_inc_inspiration_rajiv_gandhi_death_19910521`); the Leader of the House and two members call him President of the
Congress after his death (3 and 4 June 1991), and on 4 June 1991 the Prime Minister refers to a letter from 'the Congress
President', unnamed (`in_rs_pm_letter_from_unnamed_congress_president_19910604`).

Decision: accepted in part. Rajiv Gandhi is observed on 29 August 1990, the earliest record adopted.

Limits: no record attests the holder on 1 January 1990 itself. A Lok Sabha passage of 22 May 1990 is earlier but is the
Secretariat's translation of a Hindi speech, held only as an Internet Archive copy and ambiguously worded; it is a lead
(check A2). No record states the day his presidency ended, a vacancy, or an acting or interim President between 21 May
and Rao's selection. The party's past-president biography (JSON capture of 26 October 2022), which says he was party
President when assassinated, could not be re-downloaded on 25 September 2026 (the archive answered 404 for the capture it
lists) and is a lead.

### INC-PRES-02 — 1991-1992: P. V. Narasimha Rao

Evidence: in the Rajya Sabha on 31 July 1991 a member recalls, in the Secretariat's translation of a Bengali speech, that
Rao became 'the President of the Congress Party after tha[t] tragic death of Shri Rajiv Gandhi'
(`in_rs_bhattacharjee_recalls_rao_became_congress_president_1991`). The Lok Sabha Secretariat's sketch lists '29 May'91 -
Sept.1996 President, Indian National Congress (I)' (`in_ls_bio_span_rao_inc_president_19910529_199609`). The party's
session list places him at the 79th Session at Tirupati, 'Apr. 14-16, 1992', and at special sessions in 1993 and 1994,
and its biography says he presided at Tirupati in 1992. On 15 July 1996 the Leader of the House, Shri Inder Kumar
Gujral, speaks of 'the President of the Congress Party, Shri Narasimha Rao'
(`in_rs_gujral_rao_president_of_congress_party_19960715`).

Decision: accepted in part. Rao is observed on 15 July 1996; the span's 29 May 1991 is a candidate start that is not
applied.

Limits: no Working Committee decision, no AICC confirmation at Tirupati and no Central Election Authority notice was
found, and no record of 1991 or 1992 that names him as Congress President; a Lok Sabha remark of 10 April 1992 about 'the
Hon. Prime Minister, who is also the Congress(I) President' names nobody, may be a translation and is a lead.

### INC-PRES-03 — 1996: Sitaram Kesri

Evidence: Rao is still 'our Congress President' in the Rajya Sabha on 10 September 1996
(`in_rs_narayanasamy_rao_our_congress_president_19960910`). The AICC's time line lists '1996 P.V. Narasimha Rao resigns
from Presidentship of Party' and '1996 Sitaram Kesri chosen as President'; the INC's brief history says Rao resigned and
Kesri 'was elected as the new Congress president'; the Rajya Sabha Secretariat's sketch gives 'President, Indian National
Congress, 1996-98'. In the Lok Sabha on 11 April 1997 Shri N.K. Premchandran speaks of 'the Congress Working Committee
President, Shri Sitaram Kesri' (`in_ls_premchandran_kesri_cwc_president_19970411`), and the Prime Minister recalls 'the day
when Shri Sitaram Kesari became the President of the Congress Party' (`in_ls_deve_gowda_recalls_kesri_became_president_1997`).
The party lists him presiding at the 80th Session at Calcutta in August 1997.

Decision: accepted in part. Kesri is observed on 11 April 1997.

Limits: no record gives the day of his selection (expected in September 1996), the selecting body, any ratification, or
the day of Rao's resignation; the Lok Sabha sketch's 'Sept.1996' is month-only. 'Congress Working Committee President' is
the member's wording.

### INC-PRES-04 — 1998: Sonia Gandhi's selection and its ratification

Evidence: the INC website's text of the opening remarks 'by the Congress President' at the AICC meeting at Sirifort
Auditorium, 6 April 1998, says 'You have elected me President of this great organization' and 'I accept this privilege'
(`in_aicc_session_remarks_you_have_elected_me_president_19980406`, `in_aicc_session_remarks_accepts_office_19980406`); the
site's speech index dates them. The INC's 2015 history says she was 'ratified as the Congress President at the AICC
session' on 6 April 1998 (`in_inc_history_sonia_ratified_at_aicc_session_19980406`); the website's profile says 'March
1998 onwards', and the party journal and the current past-president page say April 1998. The website heads her letter of
15 May 1999 as 'written by the Congress President Smt. Sonia Gandhi' (`in_inc_site_heading_congress_president_sonia_gandhi_19990515`).

Decision: accepted in part. Sonia Gandhi is observed on 15 May 1999; the 1998 records name no one.

Limits: no INC record of the Working Committee's March 1998 decision was found: the party website was last updated on 28
February 1998 and its archived news page of December 1998 is empty. No AICC resolution of ratification was found.

### INC-PRES-05 — Sonia Gandhi's elections and extensions from 2000

Evidence: the party journal reports the Central Election Authority's rescheduling of 5 October 2000, her election on 15
November 2000 over Jitendra Prasada (7,448 to 94), Ram Niwas Mirdha's declaration and certificate, and that 'The Congress
president, Mrs. Sonia Gandhi' made the Working Committee a Steering Committee, as stated on 26 November
(`in_sonia_converts_cwc_into_steering_committee_20001126`); the Foundation Day resolution of 28 December 2000 notes her
re-election. The AICC website records her election on 28 May 2005 and, with a signed Certificate of Election, on 3
September 2010. An AICC press release of 29 April 2017 says 'The Congress President Smt. Sonia Gandhi has approved the
composition of the Central Organisational Election Authority' (`in_congress_president_sonia_approves_cea_composition_20170429`).

Decision: accepted in part. Sonia Gandhi is observed on 26 November 2000 and 29 April 2017; the 2005 and 2010 elections
are claims only.

Limits: the day of polling in 2000, a 2005 certificate and the 2015 and 2016 extensions were not found: inc.in's 2015-2016
release files were never archived, aicc.org.in returns 404 in every capture from 2014, and the Working Committee
resolutions of 8 September 2015 and 7 November 2016 are silent on the term.

### INC-PRES-06 — 2017: Rahul Gandhi

Evidence: the Congress President tells the Working Committee on 6 June 2017 that the election schedule 'will come up
before the CWC for approval today', and the briefing records its approval; the Central Election Authority's programme of
the same day and the Working Committee's schedule of 20 November 2017 follow. On 11 December 2017 the Returning Officer
declares Shri Rahul Gandhi elected (89 valid nominations, one candidate) and a certificate hand-over is set for 16
December; on 13 December he is 'President-elect'. On 16 December the certificate is presented, he accepts 'this
position', and the Hindi title of the INC's release of Sonia Gandhi's farewell calls him 'नये अध्यक्ष श्री राहुल गांधी'
(`in_rahul_gandhi_styled_new_president_20171216`). On 22 December he chairs the Working Committee as 'Congress President'.

Decision: accepted in part. Rahul Gandhi is observed on 16 December 2017 with no start.

Limits: the certificate itself was not found, and the only statement of the day of assumption is the INC's page captured
in July 2026 (`in_inc_rahul_gandhi_assumption_recalled_20171216`), a recollection.

### INC-PRES-07 — 2019: the resignation and the interim President

Evidence: the Working Committee resolution of 25 May 2019 records that 'Congress President, Shri Rahul Gandhi' offered his
resignation and that the Committee 'unanimously and with one voice rejected the same'
(`in_rahul_gandhi_attested_congress_president_20190525`). His statement 'I have resigned as Congress President' was
published by the INC on 3 July 2019. On 10 August 2019 the resignation was still before the Working Committee before its 8
pm sitting; the Committee then resolved that he should continue, recorded that he 'declined to withdraw his resignation'
and asked Smt. Sonia Gandhi 'to take over as Interim President pending the election of a regular President by the AICC',
which she accepted. The INC styles her 'President, Indian National Congress' on 22 August 2019; on 24 August 2020 the
Working Committee asks her to continue until an AICC session can be convened; on 16 October 2021 she says 'I have been
interim Congress President ever since the CWC, asked me to return in this capacity in 2019'.

Decision: accepted in part. Rahul Gandhi is observed on 25 May 2019; Sonia Gandhi's interim service is claims only.

Limits: no record states the day his office ended or an acceptance of the resignation. The five 2019 files are the INC's
August 2020 re-uploads; the originals on cdn.inc.in are gone.

### INC-PRES-08 — 2022: Mallikarjun Kharge

Evidence: the Working Committee approved an election schedule on 16 October 2021 and the final schedule on 28 August
2022; the Central Election Authority's notification of 22 September, scrutiny of 1 October (two candidates), final list of 8
October and poll of 17 October (about 9,500 of about 9,900 delegates, provisional) followed. The declaration of 19 October
records the count (Kharge 7,897, Tharoor 1,072, invalid 416) and declares him elected; that day he is 'President-Elect' and
the party says he will take charge on the 26th. On 26 October 2022 the INC heads his address 'Shri Mallikarjun Kharge,
President Indian National Congress', the General Secretary greets 'Respected Congress President, Shri Mallikarjun Kharge
Ji', and the Steering Committee list is headed 'Shri Mallikarjun Kharge, Congress President'.

Decision: accepted in part. Kharge is observed on 26 October 2022 with no start.

Limits: the handover statements of 26 October are claims; the certificate presentation is recorded only by a live inc.in
page-data file regenerated after the cutoff, which is a lead (check C3); the 85th Plenary of February 2023 records no
ratification of the result.

### INC-PRES-09 — Stated ends

Evidence: the AICC time line and the INC's brief history say Rao resigned in 1996; Sonia Gandhi tendered her resignation
on 15 May 1999 and withdrew it on 25 May 1999; she gave her last address as Congress President on 16 December 2017 and was
thanked as 'outgoing' by the Working Committee on 22 December 2017; Rahul Gandhi's resignation is recorded without a day,
and the INC later says he 'stepped down ... in May 2019'; on 26 October 2022 the resolution of thanks says Sonia Gandhi
hands over the post 'today' and she says she will be freed of the responsibility that day; the 85th Plenary calls her
'former President' on 25 February 2023.

Decision: unresolved. No primary source reviewed states the day any President's office ended, so no holder has `until`,
and no end is inferred from a successor's election or observation.

Limits: the 26 October 2022 statements end an interim arrangement, which is claims only, and are handover and farewell
statements in any case.

### INC-PRES-10 — In office before the cutoff

Evidence: the INC's highlights of 3 September 2026 style 'Shri Mallikarjun Kharge, Congress President & Leader of
Opposition (LoP), Rajya Sabha' (`in_kharge_congress_president_attested_20260903`); a release of 7 September 2026 says the
'Hon'ble Congress President has approved' two District Congress Committee appointments
(`in_congress_president_approves_dcc_odisha_20260907`).

Decision: accepted. Kharge is observed on 3 September 2026; the unnamed act of 7 September is a claim only.

Limits: the Leader of the Opposition office is a separate parliamentary role and is not used.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `in_rs_debate_19900829_sri_lanka` | Rajya Sabha Debates, 29 August 1990 (Session 155): short duration discussion on the situation in Sri Lanka, cols 315-378 (store file ID_155_29081990_01_p315-378_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/250799); PDF pages 1, 17 read |
| `in_rs_debate_19910304_surveillance` | Rajya Sabha Debates, 4 March 1991 (Session 157): alleged surveillance on Congress President Shri Rajiv Gandhi, cols 325-335 (store file ID_157_04031991_01_p325-335_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/242181); PDF pages 1 read |
| `in_rs_debate_19910305_surveillance_rajiv_gandhi` | Rajya Sabha Debates, 5 March 1991 (Session 157): statement by the Prime Minister on surveillance at the residence of Shri Rajiv Gandhi by the Haryana Police, cols 223-262 (store file ID_157_05031991_01_p223-262_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/242217); PDF pages 12, 13 read |
| `in_inc_our_inspiration_rajiv_gandhi_20260805` | Indian National Congress, Our Inspiration: Shri Rajiv Gandhi (inc.in/our-inspiration/shri-rajiv-gandhi, capture of 5 August 2026) | capture 2026-08-05 of inc.in, served gzip-encoded |
| `in_rs_debate_19910603_resolution_demise_rajiv_gandhi` | Rajya Sabha Debates, 3 June 1991 (Session 158): resolution on the demise of Shri Rajiv Gandhi, former Prime Minister, cols 2-51 (store file ID_158_03061991_01_p2-51_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/237736); PDF pages 1, 5, 19 read |
| `in_rs_debate_19910604_security_discussion` | Rajya Sabha Debates, 4 June 1991 (Session 158): short duration discussion on the failure of Government to provide adequate security to Shri Rajiv Gandhi, cols 31-164 (store file ID_158_04061991_01_p31-164_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/237760); PDF pages 4, 18, 28, 54, 55, 60 read |
| `in_rs_debate_19910731_budget` | Rajya Sabha Debates, 31 July 1991 (Session 159): the Budget (General), 1991-92, cols 245-342 (store file ID_159_31071991_01_p245-342_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/242104); PDF pages 22 read |
| `in_ls_bio_pv_narasimha_rao_xi_lok_sabha_20141024` | Lok Sabha Secretariat, biographical sketch of a Member of the XI Lok Sabha: Rao, Shri P.V. Narasimha (Congress (I), Berhampur) | capture 2014-10-24 of 164.100.47.132 |
| `in_inc_past_president_pv_narasimha_rao_20221026` | Indian National Congress, Past Party Presidents: P.V. Narasimha Rao (page data JSON of inc.in/leadership/past-party-presidents/p-v-narasimha-rao, capture of 26 October 2022) | capture 2022-10-26 of www.inc.in |
| `in_inc_sessions_20260827` | Indian National Congress, INC Sessions (page data JSON of inc.in/inc-sessions, capture of 27 August 2026) | capture 2026-08-27 of inc.in, served gzip-encoded |
| `in_rs_debate_19960715_remarks_against_congress_president` | Rajya Sabha Debates, 15 July 1996 (Session 178): re reported remarks made by Home Minister against President of Congress Party, cols 180-202 (store file ID_178_15071996_01_p180-202_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/156812); PDF pages 1, 2, 12 read |
| `in_rs_debate_19960910_ministry_of_power` | Rajya Sabha Debates, 10 September 1996 (Session 178): discussion on the working of the Ministry of Power, cols 266-357 (store file ID_178_10091996_01_p266-357_1.pdf) | official file, Rajya Sabha debates store (handle 123456789/156664); PDF pages 14 read |
| `in_aicc_timeline_20040606` | All India Congress Committee, The Indian National Congress: A Time Line of its History (aicc.org.in/timeline.htm, capture of 6 June 2004) | capture 2004-06-06 of www.aicc.org.in |
| `in_inc_brief_history_1995_2005_20260731` | Indian National Congress, Brief History of Congress 1995-2005 (inc.in/brief-history-of-congress/1995-2005, capture of 31 July 2026) | capture 2026-07-31 of inc.in, served zstd-encoded |
| `in_rs_member_sketches_k_20101005` | Rajya Sabha Secretariat, biographical sketches of members 1952-2003, letter K (rajyasabha.nic.in/rsnew/pre_member/1952_2003/k.pdf) | capture 2010-10-05 of rajyasabha.nic.in; PDF pages 8 read |
| `in_ls_debates_19970411_premchandran` | Lok Sabha Debates, 11 April 1997: motion of confidence in the Council of Ministers, continuation page 0311049722 (Shri N.K. Premchandran) | capture 2009-04-11 of parliamentofindia.nic.in |
| `in_ls_debates_19970411_deve_gowda_reply` | Lok Sabha Debates, 11 April 1997: motion of confidence in the Council of Ministers, continuation page 0311049727 (the Prime Minister's reply) | capture 2009-04-11 of parliamentofindia.nic.in |
| `in_inc_past_president_sitaram_kesri_20221026` | Indian National Congress, Past Party Presidents: Sitaram Kesri (page data JSON of inc.in/leadership/past-party-presidents/sitaram-kesri, capture of 26 October 2022) | capture 2022-10-26 of www.inc.in |
| `in_inc_site_sonia_aicc_opening_remarks_19980406` | Opening remarks by the Congress President, meeting of the All India Congress Committee, Sirifort Auditorium, New Delhi, 6 April 1998 (INC website text, capture of 19 September 2000) | capture 2000-09-19 of www.indiancongress.org |
| `in_inc_site_congress_president_speech_index_2000` | English speeches of the Congress President (INC website index, capture of 23 August 2000) | capture 2000-08-23 of www.indiancongress.org |
| `in_inc_site_sonia_resignation_letter_19990515` | Text of the resignation letter written by the Congress President Smt. Sonia Gandhi to the Congress Working Committee (INC website, capture of 19 September 2000) | capture 2000-09-19 of www.indiancongress.org |
| `in_inc_site_sonia_withdrawal_speech_19990525` | Translated speech in English at Talkatora Stadium: withdrawal of resignation (INC website, capture of 19 September 2000) | capture 2000-09-19 of www.indiancongress.org |
| `in_inc_site_president_profile_2002` | Indian National Congress, President: profile (biographical sketch of Smt. Sonia Gandhi, INC website, capture of 17 December 2002) | capture 2002-12-17 of www.indiancongress.org |
| `in_congress_sandesh_directory_congress_president_2001` | Congress Sandesh web directory: 'Congress President' page on Smt. Sonia Gandhi (capture of 1 March 2002) | capture 2002-03-01 of www.congresssandesh.com |
| `in_inc_history_years_in_opposition_2015` | 'Years in Opposition', a chapter of the INC history 'Congress and the Making of the Indian Nation' on inc.in (capture of 9 January 2015) | capture 2015-01-09 of www.inc.in |
| `in_inc_past_president_page_sonia_gandhi_20251230` | inc.in 'Past Party Presidents' page: Smt. Sonia Gandhi (capture of 30 December 2025) | capture 2025-12-30 of inc.in, served gzip-encoded |
| `in_congress_sandesh_nov2000_news_diary` | Congress Sandesh, November 2000 issue: dated news diary for October 2000 | capture 2004-01-21 of congresssandesh.com |
| `in_congress_sandesh_nov2000_sonia_elected` | Congress Sandesh, November 2000 issue: 'Sonia Gandhi Elected President' | capture 2003-09-08 of congresssandesh.com |
| `in_congress_sandesh_dec2000_reports` | Congress Sandesh, December 2000 issue: reports, including 'CWC Converted into Steering Committee' | capture 2003-05-05 of congresssandesh.com |
| `in_congress_sandesh_jan2001_foundation_day_resolution` | Congress Sandesh, January 2001 issue: resolution on the 116th Foundation Day of the Indian National Congress, 28 December 2000 | capture 2003-07-08 of www.congresssandesh.com |
| `in_aicc_site_sonia_elected_president_20050528` | aicc.org.in 'elected-president' page: Smt. Sonia Gandhi elected President on 28 May 2005 (capture of 27 October 2005) | capture 2005-10-27 of www.aicc.org.in |
| `in_aicc_site_sonia_unanimously_elected_20100903` | aicc.org.in page 'On the 3rd of September 2010 Smt. Sonia Gandhi was unanimously elected President of the Indian National Congress' (capture of 6 December 2010) | capture 2010-12-06 of www.aicc.org.in |
| `in_aicc_certificate_of_election_congress_president_20100903` | Certificate of Election of Congress President, Indian National Congress, dated 03/09/2010 (image linked from the aicc.org.in page, capture of 7 December 2010) | capture 2010-12-07 of www.aicc.org.in |
| `in_aicc_press_release_cea_composition_20170429` | AICC press release of 29 April 2017, signed by General Secretary Janardan Dwivedi: composition of the Central Organisational Election Authority | stored file, INC media store (res.cloudinary.com); PDF pages 1 read |
| `in_inc_cp_remarks_cwc_20170606` | Hon. Congress President's remarks at the CWC meeting, 6 June 2017 (INC release) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_aicc_cwc_press_briefing_highlights_20170606` | Highlights of the press briefing on the CWC meeting of 6 June 2017 (AICC) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_cea_organisational_elections_programme_20170606` | Organisational Elections Programme 2017, Phases I-V, signed by Madhusudan Mistry (Central Election Authority), 06-06-2017 | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_cea_schedule_congress_president_election_20171120` | Schedule for the election of Congress President as approved by the Congress Working Committee on 20 November 2017 (signed Mullappally Ramachandran, Chairman, Central Election Authority) | stored file, INC media store (res.cloudinary.com); PDF pages 1 read |
| `in_cea_declaration_of_result_rahul_gandhi_20171211` | Election of the President, Indian National Congress - 2017: Declaration of Result, signed Mullappally Ramachandran, Returning Officer, 11 December 2017 | stored file, INC media store (res.cloudinary.com) |
| `in_aicc_cea_press_briefing_highlights_20171211` | AICC Communication Department: highlights of the press briefing of 11 December 2017 (Mullapally Ramachandran, Chairman, Central Election Authority) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3, 4 read |
| `in_aicc_highlights_president_elect_address_20171213` | AICC Communication Department: highlights of the address at the Constitution Club, 13 December 2017 (Rahul Gandhi, President-elect) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_aicc_transcript_rahul_gandhi_speech_20171216` | Transcript: speech by Shri Rahul Gandhi, programme on the occasion of presentation of certificate to the newly elected Congress President, AICC, 24 Akbar Road, 16 December 2017 | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3, 4 read |
| `in_aicc_sonia_gandhi_outgoing_president_speech_20171216` | Speech of the outgoing Congress President Smt. Sonia Gandhi on the occasion of the new President Shri Rahul Gandhi accepting the election certificate, 24 Akbar Road, New Delhi, 16 December 2017 (Hindi original with English translation) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 3, 4, 8 read |
| `in_aicc_cwc_resolution_outgoing_president_20171222` | AICC press statement of 22 December 2017 with the CWC resolution (Hindi and English) thanking the outgoing Congress President | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_cp_opening_remarks_cwc_20171222` | Opening remarks by Congress President Shri Rahul Gandhi, Congress Working Committee meeting at AICC HQ, 24 Akbar Road, New Delhi, 22 December 2017 | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_past_president_page_rahul_gandhi_20260727` | inc.in 'Past Party Presidents' page: Shri Rahul Gandhi (capture of 27 July 2026) | capture 2026-07-27 of inc.in, served gzip-encoded |
| `in_inc_cwc_resolution_20190525` | CWC Resolution, 25 May 2019 (English page 1, Hindi pages 2-3); INC release of the Congress Working Committee resolution in Hindi and English | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_press_release_listing_20190704` | INC press releases listing (first page) as archived on 4 July 2019 | capture 2019-07-04 of www.inc.in |
| `in_inc_rahul_gandhi_statement_page_20190704` | 'Statement issued by Congress President Shri Rahul Gandhi' (INC press release page, capture of 4 July 2019) | capture 2019-07-04 of www.inc.in |
| `in_inc_cwc_media_byte_20190810` | Highlights of the media byte of 10 August 2019: Shri Randeep Singh Surjewala, In-charge, Communications Department, AICC (CWC meeting at AICC HQ) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3, 4 read |
| `in_inc_cwc_three_resolutions_20190810` | Three resolutions passed by the Congress Working Committee meeting at AICC HQ, 10 August 2019 | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_cwc_evening_briefing_20190810` | Highlights of the CWC press briefing of 10 August 2019: Shri K.C. Venugopal, General Secretary, AICC, and Shri Randeep Singh Surjewala | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3, 4 read |
| `in_inc_cp_speech_rajiv75_20190822` | Highlights of CP speech, 22 August 2019: Smt. Sonia Gandhi, President, Indian National Congress, at Rajiv@75 | stored file, INC media store (res.cloudinary.com); PDF pages 1 read |
| `in_inc_cwc_resolution_20200824` | CWC Resolution, 24 August 2020 (English pages 1-2, Hindi pages 3-4) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 4 read |
| `in_inc_cwc_briefing_20200824` | Highlights of the press briefing of 24 August 2020: Shri K.C. Venugopal, General Secretary, AICC, and Shri Randeep Singh Surjewala | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_cp_opening_remarks_cwc_20211016` | Opening remarks of Congress President Smt. Sonia Gandhi at the CWC meeting, 16 October 2021 | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_cwc_briefing_20211016` | Highlights of the press briefing of 16 October 2021: Shri K.C. Venugopal, General Secretary (Organisation), and Shri Randeep Singh Surjewala | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_cwc_briefing_20220828` | Highlights of the press briefing of 28 August 2022: Shri Madhusudan Mistry, Chairman, Central Election Authority, Shri K.C. Venugopal and Shri Jairam Ramesh | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_cea_notification_20220922` | Election of Congress President 2022: Notification, 22 September 2022, signed Madhusudan Mistry, Returning Officer | stored file, INC media store (res.cloudinary.com) |
| `in_inc_cea_media_bite_20221001` | Highlights of the media bite of 1 October 2022: Shri Madhusudan Mistry, Chairman, Central Election Authority, AICC | stored file, INC media store (res.cloudinary.com); PDF pages 1 read |
| `in_inc_cea_briefing_20221008` | Highlights of the press briefing of 8 October 2022: Shri Madhusudan Mistry, Chairman, Central Election Authority, AICC | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_cea_briefing_20221017` | Highlights of the press briefing of 17 October 2022: Shri Madhusudan Mistry, Chairman, Central Election Authority, AICC | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_cea_declaration_of_result_20221019` | Election of the President of Indian National Congress 2022: Declaration of Result (AICC, Central Election Authority), dated 19.10.2022 | stored file, INC media store (res.cloudinary.com) |
| `in_inc_cea_chairman_briefing_20221019` | Highlights of the press briefing of 19 October 2022: Shri Madhusudan Mistry, Chairman, Central Election Authority, AICC | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_president_elect_briefing_20221019` | Highlights of 19 October 2022: Shri Mallikarjun Kharge, President-Elect, Indian National Congress, addressed the media | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_maken_resolution_of_thanks_20221026` | Highlights of speech, 26 October 2022: Shri Ajay Maken, General Secretary, AICC (resolution of thanks to Smt. Sonia Gandhi) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_sonia_gandhi_speech_20221026` | Highlights of speech, 26 October 2022: Smt. Sonia Gandhi, Chairperson of the Congress Parliamentary Party | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_kharge_address_20221026` | Highlights of speech, 26 October 2022: Shri Mallikarjun Kharge, President, Indian National Congress | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3, 4, 5 read |
| `in_inc_venugopal_address_20221026` | Highlights of speech, 26 October 2022: Shri K.C. Venugopal, General Secretary, AICC | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_steering_committee_release_20221026` | Press release of 26 October 2022 (K.C. Venugopal, General Secretary): Steering Committee constituted by the Congress President | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2, 3 read |
| `in_inc_plenary_appreciation_resolution_20230225` | Resolution of Appreciation adopted at the 85th Plenary Session on the contribution of Smt. Sonia Gandhi as the President of the Indian National Congress, 25 February 2023 (English translation and Hindi original) | stored file, INC media store (res.cloudinary.com); PDF pages 1, 2 read |
| `in_inc_cp_media_bite_20260903` | Highlights of CP media bite, 3 September 2026: Shri Mallikarjun Kharge, Congress President and Leader of Opposition (LoP), Rajya Sabha | stored file, INC media store (res.cloudinary.com); PDF pages 1 read |
| `in_inc_dcc_odisha_release_20260907` | Press release of 7 September 2026 (K.C. Venugopal, General Secretary): appointment of District Congress Committee Presidents, Odisha | stored file, INC media store (res.cloudinary.com); PDF pages 1 read |

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least three times with identical bytes, the first and last at least 30 minutes apart, and no identity is a
page generated per request, a growing listing or a search. How each was established:

- **Raw Internet Archive captures (28).** Fetched in the `id_` form with `Accept-Encoding: identity` and without
  `--compressed`. Five captures are stored and served compressed whatever the request asks for (four gzip, one zstd): their
  recorded identities are the encoded bodies as transferred, and each extract gives `source_response_content_encoding` and
  the decoded size and SHA-256 (the CLAUDE-C01-11 precedent). All capture timestamps are before the cutoff.
- **The Rajya Sabha Secretariat's debates store (8).** The official host CLAUDE-C01-11 already records; each file's ETag
  equals the MD5 of its body and its Last-Modified is of July or August 2026. The store's query sets only the served content
  type and file name.
- **The Indian National Congress's media store (37).** Versioned Cloudinary uploads linked from inc.in's press releases;
  each ETag equals the MD5 of the body and Last-Modified is the upload time. Cloudinary's edge answers query-string variants
  from its cache, so a cache-busting download is not an origin test (check C8); stability rests on the ETag match and on
  downloads at least 30 minutes apart. The live inc.in release pages that link these files are rebuilt per request
  (Cloudflare e-mail tokens change the bytes) and are never recorded.
- **Times.** The researchers downloaded each dossier response twice, 31 to 77 minutes apart, between 12:02Z and 13:26Z on
  25 September 2026; the checks downloaded them again between 13:02Z and 13:42Z (Part C also after 13:33Z); this packet
  downloaded every recorded response again between 14:57Z and 15:00Z. Records added by the checks were downloaded twice by
  their check (13:11Z-14:36Z) and twice by this packet (14:47Z-14:52Z and 14:57Z-15:00Z). The large copies were deleted from
  scratch after hashing.

| Source | Bytes | SHA-256 | Downloads with identical bytes |
|---|---:|---|---|
| `in_rs_debate_19900829_sri_lanka` | 293,232 | `abaf31c2b040f0787605fa89f95e1ed4ee04a2c35d9011538d420597976f154d` | check 14:16Z and 14:35Z; packet 14:52Z, 14:59Z |
| `in_rs_debate_19910304_surveillance` | 398,978 | `208694149484595dd6462bf594700bd28dba7ee655f1a92cd61778bf15d2c0fb` | check 14:07Z and 14:34Z; packet 14:52Z, 14:59Z |
| `in_rs_debate_19910305_surveillance_rajiv_gandhi` | 1,005,620 | `1f49c918c2aa76d53c74b23059ebaceac2fae27b371bb83061e7f3fffa9c5a84` | check 13:56Z, 14:34Z; packet 14:51Z, 14:57Z |
| `in_inc_our_inspiration_rajiv_gandhi_20260805` | 14,404 | `e4a91eef61a6de615233f1e6032ac8246eeeb85e48cc68da73b626b7016fc17b` | research 12:39:24Z, 13:22:08Z; check between 13:33Z and 13:42Z; packet 14:57Z |
| `in_rs_debate_19910603_resolution_demise_rajiv_gandhi` | 2,225,446 | `dcead9b072e2b12c44fd177df0be5a2aea4770d42b7ab82b57581c2c03147baf` | check 13:56Z, 14:34Z; packet 14:52Z, 14:57Z |
| `in_rs_debate_19910604_security_discussion` | 3,611,707 | `fdc07bbb07a5a012af474dfa66f889221e3af0a611b3bb382c8be261c030b968` | check 13:56Z, 14:34Z; packet 14:51Z, 14:58Z |
| `in_rs_debate_19910731_budget` | 5,172,112 | `1461215dbfad67d3f857f75dd6e5a73263030715e193aec44441bc41551ccff3` | check 14:16Z and 14:36Z; packet 14:52Z, 14:59Z |
| `in_ls_bio_pv_narasimha_rao_xi_lok_sabha_20141024` | 4,656 | `f57efb1c73aee41e683ff11c9fca52394ba51fde5de46cf0f818b2930888f25d` | research 12:29:37Z, 13:24:28Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_inc_past_president_pv_narasimha_rao_20221026` | 1,833 | `297ee3cf049ae4914061883fdc2ae6880f7b903be583caf5a0be8bc21eca7bc3` | research 12:07:42Z, 13:21:04Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_inc_sessions_20260827` | 3,402 | `cf9de49af546df2e32f786a61642b76628731697a164d4d97e27f39a1b89aadc` | research 12:45:16Z, 13:23:51Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_rs_debate_19960715_remarks_against_congress_president` | 120,669 | `d9811540716d1ab6deb6f94b2a9aedcdbeef0e45e2be6756eac6cbb881469cd9` | check 14:07Z and 14:34Z; packet 14:52Z, 14:59Z |
| `in_rs_debate_19960910_ministry_of_power` | 623,098 | `6908fa9a830e2d2239af3e923ade37819833d1274ab5789a2ed9b0da94a7fcdd` | check 14:16Z and 14:36Z; packet 14:52Z, 14:59Z |
| `in_aicc_timeline_20040606` | 166,957 | `2b9f6e3b9110ae945a78d9fd1b0b01f140c4c54421d82274c1b8b5f093197f7a` | research 12:11:49Z, 13:24:22Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_inc_brief_history_1995_2005_20260731` | 18,108 | `e27b1786a0c26d8712c8293b97addb905e646c346eb825011eef0f3789844c6b` | research 12:48:02Z, 13:23:57Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_rs_member_sketches_k_20101005` | 133,305 | `553d0fb78bbdcec1a726bd63a056063a65c8b810bf630d1d39307e37345bd6c0` | research 12:30:21Z, 13:25:29Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_ls_debates_19970411_premchandran` | 20,221 | `33460a2efbe4972c878f24e56c8063a78608364ad88c7760b27119395eca6074` | check 14:29Z, 14:32Z and 14:36Z; packet 14:52Z, 14:59Z |
| `in_ls_debates_19970411_deve_gowda_reply` | 20,136 | `75c34c45abbfec683275a548ac6466d044aa250a426e0165b5a478c376f63140` | check 14:29Z, 14:32Z and 14:36Z; packet 14:52Z, 14:59Z |
| `in_inc_past_president_sitaram_kesri_20221026` | 849 | `a66372ee49ab30d9e577c2cea6fe17ffbb6b1820bd48b94fb54f983b4fa64eeb` | research 12:07:34Z, 13:21:13Z; check between 13:33Z and 13:42Z; packet 14:58Z |
| `in_inc_site_sonia_aicc_opening_remarks_19980406` | 22,666 | `4c8dd4d2beafcbceb69488f4fa2d19308a8047129d0d4685f23f1bdaf06777d4` | research 12:02Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_inc_site_congress_president_speech_index_2000` | 6,440 | `60ca897738afcbe2c0b5b1bedd614c0e7079ea897638adb41816720206a66a48` | research 12:03Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_inc_site_sonia_resignation_letter_19990515` | 2,082 | `82e395f2eabb63814864b5135f573d23eb51921757756f758e9fe5f3cbc5d64e` | research 12:03Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_inc_site_sonia_withdrawal_speech_19990525` | 10,341 | `debdf6e5a6e0235365b522dd2e9d810f08219c66bd1b825df86b26340bf0665a` | research 12:03Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_inc_site_president_profile_2002` | 22,366 | `6a5dcaa1c5bd9458839025465558cfed6f5a66828edd181862bada10b907c04d` | research 12:02Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_congress_sandesh_directory_congress_president_2001` | 13,191 | `2852d9ccfec222f22e7d31787fe78c23dbf683074b1c82f189415cff86239d76` | research 12:26Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_inc_history_years_in_opposition_2015` | 75,804 | `b36e6bd67188882c982b22f3f50b427ce035027cc23d96330159a93b3a1ba958` | research 12:34Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_inc_past_president_page_sonia_gandhi_20251230` | 12,523 | `d104e80a0785b5fd5d098ffb033b9ac1f53c9c576c2daf57a764d562f94e604b` | research 12:37Z, 13:10Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_congress_sandesh_nov2000_news_diary` | 23,965 | `22a2938180a8de41d4bb87dc5569999ceacbaf00c99b7ab1d73827a1354943d1` | research 12:23Z, 13:17Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_congress_sandesh_nov2000_sonia_elected` | 5,352 | `7036af0956c2ecac42570f37a6cabf8242cb27752430b718c511f461a2d102e4` | research 12:22Z, 13:18Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_congress_sandesh_dec2000_reports` | 24,398 | `914f28a0604de001c14c85749d98659289a2090b397400aff627a651d7655102` | research 12:21Z, 13:13Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_congress_sandesh_jan2001_foundation_day_resolution` | 8,968 | `52a6a7c4d0ae776bb88786fa20278eaf2347591b6ab7a4393676cdab79560b06` | research 12:20Z, 13:13Z; check between 13:28Z and 13:40Z; packet 14:58Z |
| `in_aicc_site_sonia_elected_president_20050528` | 15,410 | `58f3e954518f202ee009f3d1afd799e8a1fca4ed05cca267c686b928b61972bd` | research 12:03Z, 13:20Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_site_sonia_unanimously_elected_20100903` | 10,930 | `1fc311ed716897652484711111c0d7ab45c567e3a8da729eb38942e07a6b848c` | research 12:04Z, 13:15Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_certificate_of_election_congress_president_20100903` | 324,847 | `b350d70ca8a99c754a37417c88c4598815e75fba6b854035b5198f0718cb8f66` | research 12:07Z, 13:15Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_press_release_cea_composition_20170429` | 67,047 | `e6066095de4333887c24562f5b8e332cc7c4fea5158b428de023aade71ca5497` | research 12:14Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_inc_cp_remarks_cwc_20170606` | 117,199 | `3d185619a2e6923c0cb9db2b270532c0fd255692e79552c12f23be92377467c3` | check 13:43Z and 13:44Z; packet 14:47Z, 14:59Z |
| `in_aicc_cwc_press_briefing_highlights_20170606` | 175,499 | `4d828befaa9158518e330df029fd04b41f7676f5fe51629139ec507f2bad02cb` | research 12:14Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_cea_organisational_elections_programme_20170606` | 172,872 | `7c7380da01f4784e2534fe762a55d8eb476ba8eb4fed9d19b4bd8ee655d6adb8` | research 12:14Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_cea_schedule_congress_president_election_20171120` | 389,967 | `a132096435c0774be23decf1297cede662f9a012e4e2e50b430efebc171ec6ab` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_cea_declaration_of_result_rahul_gandhi_20171211` | 503,514 | `24a35772e0392ce0e2d60ca09e4bba6ddb974311f69ec80973d15ea7e6bb794a` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_cea_press_briefing_highlights_20171211` | 76,410 | `5f1e0a24ca72e60c4a5a4abb00891ac3fd16e6339ba0dfeb943452a47372c783` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_highlights_president_elect_address_20171213` | 177,313 | `a3ea8d42d1580c23ffa05043470c1c8053ed550ca3c68801528edaaa0bd438fa` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_transcript_rahul_gandhi_speech_20171216` | 91,142 | `644636ca14afcc3ca36cf23587642f3ad07a52f41a2833b15143271d5c2aa675` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_sonia_gandhi_outgoing_president_speech_20171216` | 404,244 | `71f0307bbda5d9d7174bf5d5feb505f8de4b9272cb14b680a1d561eb9a80d7de` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_aicc_cwc_resolution_outgoing_president_20171222` | 593,188 | `2f146cb795bd48a79c5cc79d939a09edd88b93c6cc0607d5a5fd3506d531bc99` | research 12:12Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_inc_cp_opening_remarks_cwc_20171222` | 60,430 | `8f05c0f26fd9e9255e34a89140586415c73a720024b5e0fdc72faf0326eb2771` | check 13:40Z and 13:42Z; packet 14:47Z, 14:59Z |
| `in_inc_past_president_page_rahul_gandhi_20260727` | 14,648 | `f5a65218daa8bebaaf3e6930973717317a0c9cdd3c4de4c4954756dd7a3d404e` | research 12:16Z, 13:16Z; check between 13:28Z and 13:40Z; packet 14:59Z |
| `in_inc_cwc_resolution_20190525` | 86,805 | `7752337145be583e62f8b1c1b8421ae50e55d5de5c389a0ec6c3d7080f9260a9` | research 12:11:50Z, 12:56:12Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_press_release_listing_20190704` | 119,961 | `400dd780d4ee1f34d5380f159f598e478d4ca783f6669cfef54c8c345487f3f4` | research 12:17:32Z, 12:56:44Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_rahul_gandhi_statement_page_20190704` | 40,720 | `bc3e0706fcb4837968993929eed5bc83e9a6a1432a795772cdc49dde33d756c4` | research 12:03:16Z, 12:56:19Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_cwc_media_byte_20190810` | 179,349 | `19b69b4f24f150e420427eb12f30b5a5364bb480191de958aa93b4663787a9fc` | research 12:05:55Z, 12:56:12Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_cwc_three_resolutions_20190810` | 122,977 | `8038e45d57bfbf8bb5a6c49e0f414171c2922cde58f889a48e69db99a435053c` | research 12:05:55Z, 12:56:13Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_cwc_evening_briefing_20190810` | 197,813 | `eec3eb165f5d996474a22db1f4f1f8db1d0416f0a22a87be6c48f1cfb27ff36f` | research 12:11:52Z, 12:56:13Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_cp_speech_rajiv75_20190822` | 164,283 | `83030903d3539cd2db32272623b31db291fac71fc48e1816a198564cd7325c04` | research 12:19:39Z, 12:56:13Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_cwc_resolution_20200824` | 191,780 | `f1da0b6d2e0d73eca1b11e15a0d955f6ddf8adc4be751debe45bd7bb706b891d` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cwc_briefing_20200824` | 216,700 | `45c5945dc3f58578f32352ae46f07d0a594e23723ca1b06b7d690969374c4e43` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cp_opening_remarks_cwc_20211016` | 67,874 | `78db3be74602d99fb33715f97f2b2c4499c9b18928df3d8e81b8d352ffdfce55` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cwc_briefing_20211016` | 232,932 | `de388317ca148b30854f6b9609eb5de2ff93058cd95556bb7d1b8071a683b935` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cwc_briefing_20220828` | 540,492 | `b49484dd8ecd782d42e49a4f26aa11af205d3afa9f48df844cb2fffb193d7727` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cea_notification_20220922` | 99,159 | `bbda539eb2a4aaf762bc53cc26f2b71285bf312393d63c8abd13c48e04d8b803` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cea_media_bite_20221001` | 426,255 | `30c62a88f620f46144cce63ce540e1de389fab632bd7c30c48d6dfa9a9324be7` | check 13:12Z and 13:33Z; packet 14:47Z, 14:59Z |
| `in_inc_cea_briefing_20221008` | 543,641 | `4450f67fa438eb30ddb769c339393a06ddf7e7e554fdf7ea3d63ce1cc74f76dd` | check 13:12Z and 13:33Z; packet 14:47Z, 15:00Z |
| `in_inc_cea_briefing_20221017` | 589,150 | `39db4153ccef9d96a03d5d1656dc8c7cdba22c41767a9ca7d9e977e9feaec53b` | check 13:12Z and 13:33Z; packet 14:47Z, 15:00Z |
| `in_inc_cea_declaration_of_result_20221019` | 116,341 | `8dd6c3fa5d32d12a76b96942bba38a07ab29d7d7e7d871cdaacb30d70890599e` | research 12:08:34Z, 12:56:13Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_cea_chairman_briefing_20221019` | 164,451 | `dbc069acf2aa41c7b6a990b8c8cdc28b75b8871177809f7ae2f86b6196b14b8b` | research 12:08:35Z, 12:56:14Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_president_elect_briefing_20221019` | 159,916 | `c4af13ab79e5a34ac983a1e9dc0e49e26dac17aeacd67e99acebd9a2f923ec9c` | research 12:08:35Z, 12:56:14Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_maken_resolution_of_thanks_20221026` | 416,720 | `d75822ea2815b48b9dd170ca1aa761370177129cd5069aec0c791b77c2d7f048` | research 12:09:18Z, 12:56:14Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_sonia_gandhi_speech_20221026` | 397,733 | `7010df9ad90986bedcd14b97f767f6b6c23b0509069ceac19717b457ab637514` | research 12:09:16Z, 12:56:15Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_kharge_address_20221026` | 483,792 | `42c3b7196a5f9b041fbc145a8bf49174a6a6fa7d789bb570bd41e2e3a920a10b` | research 12:09:17Z, 12:56:15Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_venugopal_address_20221026` | 306,332 | `7bbd827a9a747544104ff70b5d85b594fcabe7a01f52457a9662b3d371400d41` | research 12:09:17Z, 12:56:15Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_steering_committee_release_20221026` | 2,204,253 | `05ddf3adf84f7ab709045b9f7bf2d9c029e8321ca8e8ee94e777216036f7b278` | research 12:08:36Z, 12:56:16Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_plenary_appreciation_resolution_20230225` | 136,762 | `15b3c1e89c2ad3f55221ed81cba7c125e7ae79483bd9215a48cb8bc6241fbecf` | check 13:12Z and 13:33Z; packet 14:47Z, 15:00Z |
| `in_inc_cp_media_bite_20260903` | 452,587 | `372f5daa0ee8465a705c278658932a6712feb31537fd94f225dd6167807062c2` | research 12:13:10Z, 12:56:16Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |
| `in_inc_dcc_odisha_release_20260907` | 195,798 | `b82a80812d47a8317db5c154b2c7ef660c34a33cc810c9561098eb1731cfce6b` | research 12:13:09Z, 12:56:17Z; check at 13:02-13:03Z and again after 13:33Z; packet 14:59Z |

Compressed captures (the recorded identity is the encoded body; decoded identity for reference):

- `in_inc_our_inspiration_rajiv_gandhi_20260805`: gzip, decoded 80,039 bytes, SHA-256 `b98fa15f76fe5b58ee09be613901d16c156a0d700e7bf4cf73b41fbfa0a3369b`.
- `in_inc_sessions_20260827`: gzip, decoded 22,387 bytes, SHA-256 `91e175cabce0eaeec8adecae1e80015db52852ad186bed86274f81671dce3cc5`.
- `in_inc_brief_history_1995_2005_20260731`: zstd, decoded 98,190 bytes, SHA-256 `cbcbd20d41eddcf26197d31a013f28c0f458324672aa10a8ec24118b549ecc53`.
- `in_inc_past_president_page_sonia_gandhi_20251230`: gzip, decoded 67,209 bytes, SHA-256 `feca879777e88427e3182c7ba656c4d3f4affd00e33db0a6d2bb30863f473863`.
- `in_inc_past_president_page_rahul_gandhi_20260727`: gzip, decoded 77,102 bytes, SHA-256 `c084479fb5b823c9925d6aa066a8d6320f14db965b42bb35a581a2d15062770f`.

## Leads not imported

- Lok Sabha Debates, 22 May 1990 (https://archive.org/download/eparlib.nic.in.510/lsd_09_02_22-05-1990.pdf, 12,414,984
  bytes, SHA-256 `93b5c96b837c7292...`): the earliest passage found ('Shri Gulam Nabi Azad and Congress President. Shri Rajiv
  Gandhi'), but the Secretariat's translation of a Hindi speech, held only as an Internet Archive copy, with punctuation
  that leaves its reading uncertain (check A2).
- Lok Sabha Debates, 5 September 1990 (https://archive.org/download/eparlib.nic.in.3144/lsd_09_03_05-09-1990.pdf,
  23,393,907 bytes, `527b76fdad463f2e...`) and 22 February 1991
  (https://archive.org/download/eparlib.nic.in.3262/lsd_09_07_22-02-1991.pdf, 21,731,598 bytes, `9c9cb5f63bc8abed...`):
  English passages naming 'the Congress President Shri Rajiv Gandhi', held only as Internet Archive copies; they add only
  continuation attestations after the 29 August 1990 observation.
- Lok Sabha Debates, 10 April 1992 (https://archive.org/download/eparlib.nic.in.10512/10_III_10041992_p208_p212_t192.pdf,
  3,356,479 bytes, `2102604ec58ace87...`): 'the Hon. Prime Minister, who is also the Congress(I) President' names nobody,
  follows a '[Translation]' marker with no '[English]' marker before it, and is held only as an Internet Archive copy.
- Lok Sabha Debates, 12 June 1996 (https://archive.org/download/eparlib.nic.in.3517/lsd_11_1_12_06_1996.pdf, 4,833,239
  bytes, `6e6e598a8519047c...`) and 25 July 1997 (https://archive.org/download/eparlib.nic.in.10908/11_V_25071997_p131_p131_t262.pdf,
  985,245 bytes, `33ad35259278e869...`): translations of Hindi speeches on Internet Archive copies; the English records of
  15 July 1996 and 11 April 1997 date Rao and Kesri instead.
- Rajya Sabha, 15 July 1996, cols 239-240 (store file ID_178_15071996_01_p239-240_1.pdf, 36,230 bytes,
  `d1306f051554502f...`): the Home Minister on Rao 'remaining or not remaining as the President or Leader of his party',
  which does not distinguish the party presidency from the parliamentary leadership.
- Lok Sabha, 11 July 1991, resolution on Rajiv Gandhi's death (https://archive.org/details/eparlib.nic.in.7696): read from
  OCR only; it duplicates the Rajya Sabha records of 3 and 4 June 1991.
- The party's past-president biography of Rajiv Gandhi (the page-data capture
  https://web.archive.org/web/20221026055706id_/https://www.inc.in/_next/data/jaqjxgp0Go5FMUctobSal/leadership/past-party-presidents/rajiv-gandhi.json,
  6,002 bytes, `3ce2080e7faed681...`): downloaded by the researcher and the Part A check, but the archive answered 404 for
  this listed capture on six attempts between 14:57Z and 15:18Z on 25 September 2026, so its identity cannot be reproduced;
  its retrospective claim (party President when assassinated) is not imported.
- The INC's page data for the 2022 certificate function
  (https://inc.in/_next/data/5MH7AH6akrt52ENFFCW-S/media/press-releases/speech-by-congress-president-shri-mallikarjun-kharge-at-aicc-headquarters-new-delhi-2.json,
  728 bytes, `7826c680ad6c7e3f...`): regenerated on 25 September 2026, after the cutoff, served from a cache and tied to a
  build id that changes on redeploy (check C3).
- Undated INC re-uploads of Rahul Gandhi's 2019 statement: pdfjoiner_a4790fa70e.pdf (1,967,017 bytes, `69095c03aa6d0ac3...`,
  the signed letter) and SHRI_RAHUL_GANDHI_879ceb7fc1.pdf (110,408 bytes, `cd1a5c52...`); the dated 2019 captures are used.
- Duplicates and redundant records: the 25 May 2019 briefing (CWC_briefing_25_05_2019_8d53390e54.pdf), a second scan of
  the 2017 declaration (PRESS_RELEASE_11_12_2017_0ab871ff83.jpg, 479,437 bytes, `32b5ebf85122e3ec...`), the Eid message of
  2 December 2017 (HON_fcc708f21e.pdf), the Congress President's highlights of 2 September 2026
  (Shri_Mallikarjun_Kharge_02_09_2026_4daed81ad5.pdf), later AICC biographies and election pages
  (aicc-president-bio-display.php, aicc-president-bio.php, elected-president1.php), the AICC's 2004 past-president pages
  (rajiv_gandhi_president.htm) and the INC history's life sketches (37-Brief-Life-Sketches).
- Records that name nobody or another office: the 2019 Eid and Independence Day messages (undated bodies), the letter of 24
  August 2019 on the Congress Parliamentary Party's letterhead, the Jharkhand PCC release of 26 August 2019 and the release
  of 5 September 2026 (PR_AICC_GS_and_Incharges), both acts of an unnamed President; the 7 September 2026 release is the
  later unnamed act and is recorded.
- Silent on the President's term: the Working Committee's political resolution of 8 September 2015
  (Congress-Working-Committee-Meeting-September-8-2015-Political-Resolution) and the Congress Sandesh copies of it and of
  the resolutions of 7 November 2016 (CongressSandesh/205, CongressSandesh/796 and CongressSandesh/797).
- Context only: the Congress Sandesh diary of December 2000 (dec_2k/report4), a reader's letter in the December 2000
  reports, the AICC resolution on Rajiv Gandhi's 60th birth anniversary (rajiv-60th-birthday-resolution.htm) and the INC
  brief history of 1985-1995 (brief-history-of-congress/1985-1995, Rao's premiership, a government office).
- The Lok Sabha bioprofile of Rajiv Gandhi (sansad.in, biodata 2731) and the Lok Sabha obituary reference to Sitaram Kesri
  (eparlib.nic.in.716818): no party office.
- The wikipedia articles on the Congress presidents, and news reports of the elections: leads only, not consulted as
  evidence.

## Sources attempted

- The Parliament Digital Library (eparlib.sansad.in): connection timeouts; loksabha.nic.in and loksabhadocs.nic.in:
  connection failures; rajyasabha.nic.in: timeouts; pibarchive.nic.in: connection failures. The old Rajya Sabha host
  rsdebate.nic.in redirects every document to a sansad.in page; the Rajya Sabha data service
  (integration.rajyasabha.digital) and its store (bucketapi.rajyasabha.digital) were reachable and are used. The data
  service's 1998 item titles were searched for 'Congress President' or 'Sonia' without a match.
- Guessed sansad.in member-profile paths: 404 or 500. The Journal of Parliamentary Information has no 1991 issue on the
  Internet Archive.
- inc.in: the web archive holds no capture of the past-president HTML pages of Rajiv Gandhi, Rao or Kesri (the Rao and
  Kesri page-data captures are used; Rajiv Gandhi's answered 404 on 25 September 2026 and is a lead), none of the 2015-2016 release PDFs under inc.in/images/Pages/, and none of the 26 October 2022 and
  2 and 3 September 2026 release pages before the cutoff. The live release pages are not byte-stable (Cloudflare e-mail
  tokens); one old release slug answered 403. The live release listing, read on 25 September 2026 under build id
  5MH7AH6akrt52ENFFCW-S, holds nothing between its 2013 and 2015 items and the releases of 10 November 2016, so it has no
  record of the 2015 or 2016 extension; it is a growing listing and is not recorded.
- indiancongress.org, 1998-1999: the home page says 'Last update: February 28, 1998' and the news page captured on 6
  December 1998 is empty; congresssandesh.com online issues begin in 2000. aicc.org.in answers 404 in every capture from
  January 2014.
- cdn.inc.in (the original 2019 attachments): the domain no longer resolves and the files were never archived; the INC's
  Cloudinary re-uploads are used. The web archive's capture of 22 October 2022 of the 2022 declaration image answered 404;
  the stored file is used. An archived 2022 inc.in release page served gzip was not needed.
- web.archive.org refused connections or went offline at times on 25 September 2026 (including about 12:10Z, 13:05-13:35Z
  and 15:14Z); requests were retried, and no recorded identity is an error page.

No site terms, licences or cookie banners were accepted, no CAPTCHA was met, and no login was used.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | The 5 Mar and 4 Jun 1991 Rajya Sabha PDFs were labelled mirror-only | **Applied**: all three Rajya Sabha PDFs of the dossier are recorded from the Secretariat's debates store (same bytes as the researcher's copies); the mirror-only wording is gone |
| A2 | Rajiv Gandhi's earliest attestation was not 5 Mar 1991 | **Applied in part**: the Rajya Sabha records of 29 Aug 1990 and 4 Mar 1991 are imported and the holder is observed on 29 Aug 1990; the Lok Sabha copies of 22 May 1990, 5 Sep 1990 and 22 Feb 1991 are declined (see A-R6 to A-R8) |
| A3 | Holders cited claims not dated on their own day | **Applied**: every holder cites only in-office attestations of its own `attested_on` day, and the test pins it |
| A4 | Rao was dated by a later session list | **Applied in part**: Rao is observed on 15 Jul 1996 (Leader of the House, English, official store); the recollection of 31 Jul 1991 is imported; the Lok Sabha records of 12 Jun 1996 and 10 Apr 1992 are declined (A-R10, A-R11) |
| A5 | Kesri was dated by a later session list, and a lead misread 11 Apr 1997 | **Applied**: Kesri is observed on 11 Apr 1997 (page 22); the Prime Minister's undated recollection on page 27 is a claim; the lead is corrected; the translated record of 25 Jul 1997 is declined (A-R17) |
| A6 | The Lok Sabha sketch's span was split into two claims | **Applied**: one `retrospective_term_span` claim, `in_ls_bio_span_rao_inc_president_19910529_199609`, with no structured date |
| A7 | `attested_period` on thirteen claims | **Applied**: removed; year, month and range claims carry no structured date and keep the printed words in the text |
| A8 | The 1996 transition claims were too strong | **Applied**: Rao's attestation of 10 Sep 1996 is imported as a continuation claim; the resignation and selection claims and the observation text are reworded; the 25 Jul 1997 remark is a lead |
| A9 | Several new records are translations | **Applied**: the one translation imported (31 Jul 1991) says the Bengali original was not read; the translated Lok Sabha records are declined in favour of English originals |
| A10 | The 4 Jun 1991 locator omitted page 60 | **Applied**: PDF pages 55 and 60 (cols 139-140 and 149-150) |
| A11 | Kesri's party page gives a lifespan that conflicts with the Rajya Sabha sketch | **Applied**: noted in the source's scope and the claim's uncertainty |
| B1 | Rahul Gandhi's `from` rested on a 2026 recollection | **Applied**: `from` null; observed on 16 Dec 2017 by the INC's same-day Hindi title; the recollection is `assumption_recalled_retrospective`; INC-PRES-06 is accepted in part |
| B2 | Sonia Gandhi's 2017 holder and its candidate end | **Resolved by removal**: the dossier's holder dated 16 Dec 2017 (the farewell day, with that day as a candidate end) is dropped; the farewell is `farewell_address`, never an until; the 29 Apr 2017 observation comes from B3 and has no end |
| B3 | Sonia Gandhi's 2000, 2005 and 2010 holders rested on elections | **Applied in part**: 2000 is observed on 26 Nov 2000 (an act in office); no exception is made for 2005 and 2010, whose records stay claims; the 29 Apr 2017 release dates a second observation |
| B4 | Sonia Gandhi's 1998 holder named her by inference | **Applied**: the 6 Apr 1998 rows are nameless; the observation is dated 15 May 1999 by the INC's heading that names and styles her, as a claim separate from the resignation |
| B5 | Row shape | **Applied**: `observation_id` `in_eci_20240323_np_05`, `review_observation`, `role_id` and `role_title` in every row |
| B6 | `holder_name` carried printed forms | **Applied**: normalised names; printed forms, the Hindi included, stay in the texts |
| B7 | Synthetic month periods | **Applied**: removed; no structured date is stored |
| B8 | The Hindi close on page 8 was not read | **Applied**: read and quoted; 'laying down this office' is attributed to the INC's translation |
| B9 | The withdrawal speech's paraphrase | **Applied**: reworded to the speech's words; only the INC's title says 'withdrawal' |
| B10 | The 15 Nov 2000 paraphrase merged two moments | **Applied**: the brief speech and the later remark to journalists are separated |
| B11 | The rescheduling's list of bodies | **Applied**: quoted as printed |
| B12 | The resignation letter's wording | **Applied**: 'certain of my colleagues' quoted |
| B13 | The past-president page's tense | **Applied**: the uncertainty names the past-tense opening and the present-tense sentences |
| B14 | Event kinds differed from the stacked vocabulary | **Applied**: `resignation_tendered`, `in_office_continuation_attestation`, `farewell_address`, `assumption_recalled_retrospective` and the rest of one vocabulary, pinned |
| B15 | Two dated sentences of the INC history were not imported | **Applied**: the 1999 resignation and the Working Committee's reaffirmed faith are claims |
| B16 | The gzip identities | **Applied**: encoding fields and decoded identities in the extracts; the provenance says the identity is the encoded body (curl without --compressed) |
| B17 | The 6 Jun 2017 briefing is a 2020 re-keying | **Applied**: stated in its scope; the President's own remarks of that day are imported |
| B18 | The live release listing was described by page number | **Applied**: Sources attempted describes it by date range and build id |
| B19 | The 2015 and 2016 extensions | **Applied**: no new evidence; the three silent resolutions are listed as leads |
| C1 | Observation ids collided with the presidency's IN-PRES-07..10 | **Applied**: INC-PRES-01..10 throughout; the test forbids IN-PRES and IN-PM numbers on the party rows |
| C2 | `attested_period` on the 2019 statement | **Applied**: renamed `in_rahul_gandhi_states_he_has_resigned_2019`, no structured date; the listing keeps 3 Jul 2019 |
| C3 | The certificate-function page data was regenerated after the cutoff | **Resolved by removal**: a lead; INC-PRES-08 says the certificate is not recorded |
| C4 | Kharge's proposed `from` | **Applied**: observed on 26 Oct 2022 with no start; the ruling and the ANC contrast are recorded |
| C5 | The holder shape mixed two models | **Applied**: separate dated observations (the ANC model), each citing only its own day's claims |
| C6 | The declaration row copied a misprint | **Applied**: `holder_name` 'Mallikarjun Kharge'; the misprint stays in the text |
| C7 | A locator pointed to page 3 only | **Applied**: pages 2 and 3 |
| C8 | Stability wording relied on cache-busting | **Applied**: ETag equals MD5; the edge cache is disclosed |
| C9 | The 2019 files are August 2020 re-uploads | **Applied**: stated in each scope note |
| C10 | House fields were missing | **Applied**: `source_type`, `rights_note`, `scope_note`, `pdf_page` locators and structured visual review |
| C11 | 'Midday' is in no source | **Applied**: 'before the Working Committee's 8 pm sitting' |
| C12 | Sonia Gandhi's interim status | **Applied**: the 24 Aug 2020 resolution and briefing and the 16 Oct 2021 remarks are imported; interim service is claims only |
| C13 | The 2022 election steps were missing | **Applied**: the 16 Oct 2021 and 28 Aug 2022 schedules, the notification, scrutiny, final list and same-day poll are imported; the 19 Oct sentence is a recited corroboration |
| C14 | The AICC ratification was 'not sought' | **Applied**: the 85th Plenary's resolution is imported, and the text says no ratification was found in the records checked |

Missing primary records the checks confirmed:

| # | Record | Outcome |
|---|---|---|
| A-R1 | Rajya Sabha store copy, 5 Mar 1991 | **Imported**: the URL of `in_rs_debate_19910305_surveillance_rajiv_gandhi` |
| A-R2 | Rajya Sabha store copy, 4 Jun 1991 | **Imported**: the URL of `in_rs_debate_19910604_security_discussion` |
| A-R3 | Rajya Sabha store copy, 3 Jun 1991 | **Imported**: the URL of `in_rs_debate_19910603_resolution_demise_rajiv_gandhi` |
| A-R4 | Rajya Sabha, 4 Mar 1991 | **Imported**: `in_rs_debate_19910304_surveillance` |
| A-R5 | Rajya Sabha, 29 Aug 1990 | **Imported**: `in_rs_debate_19900829_sri_lanka` (Rajiv Gandhi's observation) |
| A-R6 | Lok Sabha, 22 May 1990 | **Declined**: a translation on an Internet Archive copy with an uncertain reading; a lead |
| A-R7 | Lok Sabha, 5 Sep 1990 | **Declined**: an Internet Archive copy adding only a continuation after 29 Aug 1990; a lead |
| A-R8 | Lok Sabha, 22 Feb 1991 | **Declined**: as A-R7 |
| A-R9 | Rajya Sabha, 31 Jul 1991 | **Imported**: `in_rs_debate_19910731_budget` |
| A-R10 | Lok Sabha, 10 Apr 1992 | **Declined**: names nobody, may be a translation, Internet Archive copy only; a lead |
| A-R11 | Lok Sabha, 12 Jun 1996 | **Declined**: a translation; the English Rajya Sabha record of 15 Jul 1996 dates Rao; a lead |
| A-R12 | Rajya Sabha, 15 Jul 1996, cols 180-202 | **Imported**: `in_rs_debate_19960715_remarks_against_congress_president` (Rao's observation) |
| A-R13 | Rajya Sabha, 15 Jul 1996, cols 239-240 | **Declined**: 'President or Leader of his party' does not distinguish the offices; a lead |
| A-R14 | Rajya Sabha, 10 Sep 1996 | **Imported**: `in_rs_debate_19960910_ministry_of_power` |
| A-R15 | Lok Sabha, 11 Apr 1997, page 22 | **Imported**: `in_ls_debates_19970411_premchandran` (Kesri's observation) |
| A-R16 | Lok Sabha, 11 Apr 1997, page 27 | **Imported**: `in_ls_debates_19970411_deve_gowda_reply` |
| A-R17 | Lok Sabha, 25 Jul 1997 | **Declined**: a translation on an Internet Archive copy, a continuation after 11 Apr 1997; a lead |
| B-R1 | Rahul Gandhi's opening remarks at the Working Committee, 22 Dec 2017 | **Imported**: `in_inc_cp_opening_remarks_cwc_20171222` |
| B-R2 | The Congress President's remarks at the Working Committee, 6 Jun 2017 | **Imported**: `in_inc_cp_remarks_cwc_20170606` |
| B-R3 | The Hindi 'new President' title in the release of 16 Dec 2017 | **Imported**: `in_rahul_gandhi_styled_new_president_20171216`, from the already-hashed file |
| C-R1 | Working Committee resolution, 24 Aug 2020 | **Imported**: `in_inc_cwc_resolution_20200824` |
| C-R2 | Working Committee briefing, 24 Aug 2020 | **Imported**: `in_inc_cwc_briefing_20200824` |
| C-R3 | Sonia Gandhi's opening remarks, 16 Oct 2021 | **Imported**: `in_inc_cp_opening_remarks_cwc_20211016` |
| C-R4 | Working Committee briefing, 16 Oct 2021 | **Imported**: `in_inc_cwc_briefing_20211016` |
| C-R5 | Working Committee schedule approval, 28 Aug 2022 | **Imported**: `in_inc_cwc_briefing_20220828` |
| C-R6 | Election notification, 22 Sep 2022 | **Imported**: `in_inc_cea_notification_20220922` |
| C-R7 | Nomination scrutiny, 1 Oct 2022 | **Imported**: `in_inc_cea_media_bite_20221001` |
| C-R8 | Final list of candidates, 8 Oct 2022 | **Imported**: `in_inc_cea_briefing_20221008` |
| C-R9 | Same-day poll record, 17 Oct 2022 | **Imported**: `in_inc_cea_briefing_20221017` |
| C-R10 | 85th Plenary resolution of appreciation, 25 Feb 2023 | **Imported**: `in_inc_plenary_appreciation_resolution_20230225` |

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-India-INC-002`: the Working Committee's decisions selecting Rao (1991), Kesri (1996) and Sonia Gandhi (March 1998)
  and any AICC ratification, from the party's print archive or from a network that can reach the Parliament Digital
  Library.
- `C01-India-INC-003`: the 2015 and 2016 extensions of Sonia Gandhi's term and the 2005 certificate.
- `C01-India-INC-004`: the certificates of 2017 and 2022 and any record fixing the hour of each handover.
- `C01-India-PARTY-001`: the leaders of other national parties in the ECI table (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** this branch is stacked on `claude/c01-in-15` at `dd58a610` (CLAUDE-C01-15, ready for review),
  which is stacked on `claude/c01-in-11` at `538920f1` (CLAUDE-C01-11) and so based on `ffe54b02`; the claim commit
  `d19f756b` holds only the handoff. Merge CLAUDE-C01-11 and then CLAUDE-C01-15 first: all three packets edit
  `india.json`, and this packet only appends to it (73 sources after the C01-15 sources; on `in_eci_20240323_np_05` one
  role, its sources and claim ids and one coverage note; one packet coverage note after the C01-15 note).
  `origin/claude/c01-in-15` had no commits beyond `dd58a610` when this packet was written, so no merge was needed.
- No existing extract is edited: the 73 extracts under `research/sources/` are new files, and no CLAUDE-C01-11 or
  CLAUDE-C01-15 source, claim, holder or extract changes. The verifier fixes edited one claim text in two of these
  extracts, `sources/india-inc-cea-briefing-20221019-facts.json` (`in_certificate_to_be_given_20221019`) and
  `sources/india-inc-cwc-opening-remarks-20211016-facts.json`
  (`in_sonia_gandhi_interim_congress_president_self_description_20211016`), and the same claims and the two snapshots
  in `india.json`.
- `research-index.json` is the only file this packet shares with other pending packets. It is regenerated in a
  **separate commit**. New totals against `dd58a610`: 360 sources and 2,071 claims (previously 287 and 1,958); organization
  and institution observations are unchanged (841 and 29). India now has three role observations (previously two), 391
  claims (previously 278), 84 entries pending mapping and the same nine discovery batches. The index must be regenerated
  after any other pending packet merges.
- Pinned tests updated, none loosened and no assertion removed:
  - `test_india_research_s10e.py`: counts (entries, sources, claims, roles) (84, 201, 391, 3), from (84, 128, 278, 2); the
    one organization role is pinned to `in_eci_20240323_np_05`, and the test that recognition grants no role is re-expressed
    exactly for that one observation (every other observation still has none, and all keep their empty mapping, lifecycle
    and status); the 73 sources are pinned to three hosts (the web archive, the Rajya Sabha store and res.cloudinary.com)
    and the 2026-09-25 access date, and their rows to the party role; the source list is the original, C01-11, C01-15 and
    then C01-20 sources.
  - `test_india_prime_ministers_c01_11.py` and `test_india_presidents_c01_15.py`: the source list now ends with the party
    role's sources (73 sources and 113 claims pinned); roles 3 (from 2); the rows beyond C01-11's are the presidency's or
    the party role's, the latter numbered INC-PRES only; the packet coverage has ten notes with C01-11's at index 7,
    C01-15's at index 8 and C01-20's last; the index has three role observations.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and the
  sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows "Observed on" for a holder with `attested_on`; every party
  holder is such an observation. No UI code changed.
- The party role's event-kind vocabulary is pinned in `test_india_inc_presidents_c01_20.py`.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_india*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
