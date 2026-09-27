# Indian prime ministers 11: holders and transitions, 1990-2026

Packet: **CLAUDE-C01-11**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-11`, based on `codex/campaign-certification`
at `ffe54b02`; claim commit `38f36fa2`. Research access: 23 and 24 September 2026 (UTC). The historical cutoff stays
**7 September 2026**.

This packet adds the prime-ministership to [india.json](india.json), which until now held 82 party-recognition
observations from two Election Commission sources and no institution. It reviews ten observations from 1 January 1990
to the cutoff: the Prime Minister when the period opens and the 1990 confidence vote, the November 1990 change and the
1991 resignation and continuation, the June 1991 appointment and the 1996 resignation, the three 1996 transitions, the
1997-1998 votes, resignations and continuations, the 1998-1999 appointments, vote, continuation and reappointment, and the
transitions of 2004, 2009, 2014, 2019 and 2024 with official attestations of the holder in office in 2026. It adds 70
sources and 112 claims, one institution (`in_prime_minister`, an `executive_institution` with lifecycle `unknown`) with
one role (`in_pm`, "Prime Minister of India", kind `head_of_government`), thirteen holder observations, a role scope
note, institution coverage and a bounded note in the packet coverage. It adds no organization, game mapping, lifespan,
portrait or avatar, and it changes none of the 82 organization observations or the two original sources. The parent
scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1990-1996; B: 1996-1999; C: 2004-2026), and each part was checked
independently before this packet was written. Every checker defect is applied (see [Checker defects](#checker-defects)),
and every missing primary record the checks confirmed is imported or explained.

## Outcome

| ID | Question | Decision |
|---|---|---|
| IN-PM-01 | The holder when the period opens (V. P. Singh), the 1990 confidence vote and his resignation | **Accepted in part:** observed on 12 Mar 1990; confidence motion negatived 142-346 on 7 Nov 1990; no primary record of his resignation, its acceptance or his end |
| IN-PM-02 | Chandra Shekhar: the November 1990 swearing-in, the 1991 resignation and the request to continue | **Accepted in part:** observed on 16 Nov 1990 (confidence vote won 269-204); resignation announced and tendered, accepted and continuation requested on 6 Mar 1991; in office on 4 Jun 1991; resignation accepted with effect from 21 Jun 1991 (his end); no record of his appointment day |
| IN-PM-03 | The June 1991 swearing-in and the 1996 resignation | **Accepted in part:** Rao appointed with effect from 12.50 P.M. on 21 Jun 1991 (his start) and his resignation accepted with effect from 16 May 1996 (his end); the day he tendered it and any request to continue are not found |
| IN-PM-04 | 1996: the May swearing-in, the resignation that followed and the June swearing-in | **Accepted in part:** Vajpayee appointed with effect from noon on 16 May 1996 (his start); resignation announced on 28 May, motion not put to the vote; Deve Gowda observed on 11 Jun 1996; no 1996 resignation instrument, no Vajpayee end and no Deve Gowda appointment record |
| IN-PM-05 | 1997-1998: the April 1997 vote and resignation, the April 1997 swearing-in, the November 1997 resignation and continuation | **Accepted in part:** vote lost 158-292 on 11 Apr 1997; Deve Gowda's resignation accepted with effect from 21 Apr 1997 (his end) and Gujral appointed with effect from 10.00 hours that day (his start); Gujral's resignation and continuation of 28 Nov 1997; his end is not stated (a corrigendum withdrew the only printed date) |
| IN-PM-06 | 1998-1999: the March 1998 swearing-in, the April 1999 vote, the continuation and the October 1999 swearing-in | **Accepted:** appointed with effect from 0930 hours on 19 Mar 1998; vote lost 269-270 on 17 Apr 1999; resignation accepted with a request to continue; resignation accepted with effect from 13 Oct 1999 and reappointed with effect from 1030 hours that day |
| IN-PM-07 | 2004: the resignation and continuation, and the May swearing-in | **Accepted in part:** Vajpayee's resignation, acceptance and request to continue on 13 May 2004 (19:30 IST); Manmohan Singh observed on 22 May 2004; the 19 May 2004 appointment is not byte-stable and no end of the continuation is stated |
| IN-PM-08 | 2009: the resignation, continuation and the May swearing-in | **Accepted in part:** resignation, acceptance and request to continue on 18 May 2009; appointment on 20 May; observed on 22 May 2009; no day of effect and no end of the continuation |
| IN-PM-09 | 2014 and 2019: the resignations, continuations and the two swearings-in | **Accepted in part:** Modi observed on 26 May 2014 (sworn in at 1800 hours) and 30 May 2019 (1900 hours) after appointments of 20 May 2014 and 25 May 2019; resignations with continuations of 17 May 2014 and 24 May 2019 whose ends are not stated |
| IN-PM-10 | 2024: the June swearing-in, and an official attestation before the cutoff without extending any term | **Accepted in part:** resignation and continuation of 5 Jun 2024; appointment of 7 Jun; observed on 9 Jun 2024 (1915 hours); in office on 23 Jun, 25 Jul and 30 Aug 2026; the end of the 2024 continuation is not stated |

The resulting holder observations of `in_pm`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Vishwanath Pratap Singh | 1990-03-12 | null | null | introduces a Minister as Prime Minister, Lok Sabha, 12 Mar 1990 (raw capture of the eparlib PDF) |
| Chandra Shekhar | 1990-11-16 | null | 1991-06-21 | moves the confidence motion as Prime Minister (Lok Sabha, mirror copy); MHA notification No. 10/4/91-M&G(i): resignation accepted with effect from 21 Jun 1991 |
| P. V. Narasimha Rao | null | 1991-06-21 | 1996-05-16 | MHA No. 10/4/91-M&G(ii): appointed with effect from 12.50 P.M. of 21 Jun 1991; MHA No. 10/5/96-M&G(i): resignation accepted with effect from 16 May 1996 |
| Atal Bihari Vajpayee | null | 1996-05-16 | null | MHA No. 10/5/96-M&G: appointed with effect from 12 noon of 16 May 1996 |
| H. D. Deve Gowda | 1996-06-11 | null | 1997-04-21 | introduced as Prime Minister by the Speaker, 11 Jun 1996; MHA No. 10/1/97-M&G(i): resignation accepted with effect from 21 Apr 1997 |
| I. K. Gujral | null | 1997-04-21 | null | MHA No. 10/1/97-M&G(ii): appointed with effect from 10.00 hours of 21 Apr 1997 |
| Atal Bihari Vajpayee | null | 1998-03-19 | 1999-10-13 | MHA No. 10/1/98-M&G(i): appointed with effect from 0930 hours of 19 Mar 1998; MHA No. 10/1/99-M&G(ii): resignation accepted with effect from 13 Oct 1999 |
| Atal Bihari Vajpayee | null | 1999-10-13 | null | MHA No. 10/1/99-M&G(i): appointed with effect from 1030 hours of 13 Oct 1999; PIB: "has assumed the august office" that day |
| Manmohan Singh | 2004-05-22 | null | null | President's Secretariat communique of 22 May 2004 (PIB, archived) |
| Manmohan Singh | 2009-05-22 | null | null | President's communique of 22 May 2009 (archived) |
| Narendra Modi | 2014-05-26 | null | null | Cabinet Secretariat O.M.: sworn in as Prime Minister at 1800 hours; President's communique of the same day |
| Narendra Modi | 2019-05-30 | null | null | President's communique headed as the oath to the Prime Minister; Cabinet Secretariat O.M.: 1900 hours |
| Narendra Modi | 2024-06-09 | null | null | Cabinet Secretariat O.M.: sworn in at 1915 hours; President's communique; attested in office on 23 Jun, 25 Jul and 30 Aug 2026 |

### How a start and an end are decided

One rule covers all three parts. A holder has `from` only where a source states the day the appointment took effect or
office was assumed, and `until` only where a source states the day the office ended; otherwise the holder is dated by
`attested_on`. From 1991 to 1999 the Ministry of Home Affairs (M&G section, not the Cabinet Secretariat) published each
appointment and each accepted resignation in the Gazette of India Extraordinary, Part I Section 2, "with effect from" a
stated day and often an hour; those notifications give all five starts and all four ends. For 2004-2024 no such
notification could be searched (the eGazette search was unavailable), so those holders are dated by a same-day record:
the President's communique that he "has appointed" the Prime Minister, issued with the oath, and from 2014 the Cabinet
Secretariat memorandum that the Prime Minister "has been sworn in" at a stated hour. An oath or swearing-in report dates
an observation and never makes a start unless it states that office was assumed (PIB, 13 October 1999, which agrees with
the Gazette). No clock time is stored.

Moving a motion of confidence as "THE PRIME MINISTER" is an act in office, like introducing ministers, and it dates
Chandra Shekhar's observation (16 November 1990), as both checks verified. The division that follows is a separate claim
and never a holder date, and the motions of V. P. Singh (7 November 1990), Vajpayee (27 May 1996) and Deve Gowda
(11 April 1997) are kept off their holders as context, so that a last act before an unstated end cannot be read as that
end.

Proposed ends that are refused:

- **A resignation, its acceptance by letter or communique, or a request to continue.** Every acceptance by the
  President found, from 1991 to 2024, asked the Prime Minister and colleagues to continue till a new Government was
  formed or other arrangements were made. The acceptance letters are kept as their own claims, separate from the Gazette's "accepted with
  effect from" claims, and only the latter give an `until` (check B9).
- **Service during that continuation.** It is claims only (Chandra Shekhar in the Rajya Sabha on 4 June 1991; Gujral in
  PIB's releases of 12 March 1998; Vajpayee at the Red Fort on 15 August 1999) and never a separate holder.
- **A successor's appointment or swearing-in, or the same person's reappointment.** None is used as an end. Where the
  Gazette gives an end on the day a successor was appointed or the same person reappointed (21 June 1991, 21 April 1997,
  13 October 1999), the end rests on the acceptance "with effect from" that day, published as a separate notification.
- **Retrospective PMO and PIB spans and members' recollections.** The PMO's former-prime-minister pages and PIB's
  profiles are claims with no structured date; Somnath Chatterjee's statements that the governments were sworn in on
  16 May and 1 June 1996 and the Speaker's 1996 statement about 21 June 1991 are claims dated by the day they recall,
  never holder dates.
- **A superseded Gazette text.** The notification of 6 April 1998 first accepted Gujral's resignation "with effect from
  19th March, 1998"; the corrigendum of 14 August 1998 replaced those lines, so that only the listed ministers
  relinquished charge that day and Gujral and his colleagues were advised to continue. Gujral has no `until` (check B2).

Where the Gazette's acceptance "with effect from" a day follows a request to continue (1991, 1997 and 1999), this packet
reads the stated effect as also ending the continuation. That reading is not in the notifications' words, and each
acceptance claim's uncertainty and holder note says so (check A1).

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| 12 Mar 1990 | V. P. Singh introduces a Minister as Prime Minister | `in_vp_singh_pm_introduces_minister_19900312`; V. P. Singh `attested_on` |
| 7 Nov 1990 | V. P. Singh moves the confidence motion; negatived 142-346 | `in_vp_singh_moves_confidence_motion_19901107`, `in_ls_confidence_motion_negatived_19901107` (claims) |
| 16 Nov 1990 | Chandra Shekhar moves the confidence motion; adopted 269-204 | `in_chandra_shekhar_moves_confidence_motion_19901116` (Chandra Shekhar `attested_on`); `in_ls_confidence_motion_adopted_19901116` (claim) |
| 27 Dec 1990 | Chandra Shekhar introduces Ministers in the Rajya Sabha | `in_chandra_shekhar_introduces_ministers_rs_19901227` (support) |
| 6 Mar 1991 | Resignation announced in both Houses; letter tendering it; the President's letter accepting it and asking him to continue | `in_chandra_shekhar_announces_resignation_19910306`, `in_rs_leader_announces_pm_resignation_19910306`, `in_rs_lays_chandra_shekhar_resignation_letter_19910306`, `in_chandra_shekhar_resignation_letter_19910306`, `in_rs_lays_president_acceptance_19910306`, `in_president_accepts_chandra_shekhar_resignation_19910306`, `in_rs_lays_continuation_request_19910306`, `in_president_requests_chandra_shekhar_continue_19910306` (claims; laid on 7 and 11 March) |
| 4 Jun 1991 | Chandra Shekhar speaks as Prime Minister during the continuation | `in_chandra_shekhar_pm_in_rajya_sabha_19910604` (claim) |
| 21 Jun 1991 | Chandra Shekhar's resignation accepted with effect from that day; Rao appointed with effect from 12.50 P.M. (both notified on 5 Jul 1991); the Speaker's 1996 recollection of the swearing-in | `in_president_accepts_chandra_shekhar_resignation_wef_19910621` (Chandra Shekhar `until`); `in_rao_appointed_pm_wef_19910621` (Rao `from`); `in_rao_swearing_in_recalled_by_speaker_19910621` (retrospective claim) |
| 16 May 1996 | Rao's resignation accepted with effect from that day; Vajpayee appointed with effect from 12 noon (both notified on 24 May 1996); a member's recollection | `in_president_accepts_rao_resignation_wef_19960516` (Rao `until`); `in_vajpayee_appointed_pm_wef_19960516` (Vajpayee 1996 `from`); `in_vajpayee_swearing_in_recalled_in_house_19960516` (claim) |
| 23 May 1996 | Vajpayee introduces his Council as Prime Minister | `in_vajpayee_introduces_council_as_pm_19960523` (support) |
| 27 May 1996 | The confidence motion debated | `in_vajpayee_moves_confidence_motion_19960527` (claim) |
| 28 May 1996 | Resignation announced on the floor; motion not put to the vote | `in_vajpayee_resignation_announced_in_house_19960528`, `in_vajpayee_confidence_motion_not_put_to_vote_19960528` (claims) |
| 1 Jun 1996 | Deve Gowda's swearing-in, recalled by a member on 11 June | `in_deve_gowda_swearing_in_recalled_in_house_19960601` (claim) |
| 11 Jun 1996 | The Speaker introduces Deve Gowda as Prime Minister | `in_deve_gowda_introduced_as_pm_19960611`; Deve Gowda `attested_on` |
| 11 Apr 1997 | Confidence motion negatived 158-292; resignation letter | `in_deve_gowda_moves_confidence_motion_19970411`, `in_deve_gowda_confidence_motion_negatived_19970411`, `in_deve_gowda_resignation_tendered_19970411` (claims) |
| 12 Apr 1997 | The President's letter accepting the resignation and asking him to continue | `in_president_accepts_deve_gowda_resignation_19970412`, `in_deve_gowda_requested_to_continue_19970412` (claims) |
| 21 Apr 1997 | Deve Gowda's resignation accepted with effect from that day; Gujral appointed with effect from 10.00 hours (both notified on 29 Apr 1997); swearing-in concluded by 11:04; Gujral introduced at 11:31; PMO recollection | `in_president_accepts_deve_gowda_resignation_wef_19970421` (Deve Gowda `until`); `in_gujral_appointed_pm_wef_19970421` (Gujral `from`); `in_gujral_introduced_as_pm_19970421` (support); `in_gujral_swearing_in_concluded_19970421`, `in_pmo_gujral_swearing_in_recalled_19970421` (claims) |
| 28 Nov 1997 | Gujral's resignation letter; the President's acceptance and request to continue | `in_gujral_resignation_tendered_19971128`, `in_president_accepts_gujral_resignation_19971128`, `in_gujral_requested_to_continue_19971128` (claims) |
| 12 Mar 1998 | PIB styles Gujral Prime Minister | `in_gujral_styled_pm_holi_greetings_19980312` (claim) |
| 16 Mar 1998 | PIB reports the President's invitation to Vajpayee | `in_president_invites_vajpayee_to_form_government_19980316` (claim) |
| 19 Mar 1998 | Vajpayee appointed with effect from 0930 hours (notified on 6 Apr 1998); sworn in; oath caption; Council list | `in_vajpayee_appointed_pm_wef_19980319` (Vajpayee 1998 `from`); `in_vajpayee_sworn_in_19980319`, `in_vajpayee_oath_of_office_19980319`, `in_vajpayee_heads_council_list_19980319` (support) |
| 14 Aug 1998 | Corrigendum replacing the first-printed acceptance of Gujral's resignation "with effect from 19th March, 1998" | `in_gazette_corrigendum_gujral_acceptance_19980814` (claim); `in_gujral_resignation_accepted_wef_superseded_19980406` (no structured date) |
| 17 Apr 1999 | Confidence motion negatived 269-270; resignation letter; the President's acceptance and request to continue | `in_vajpayee_confidence_motion_negatived_19990417`, `in_vajpayee_resignation_tendered_19990417`, `in_president_accepts_vajpayee_resignation_19990417`, `in_vajpayee_requested_to_continue_19990417` (claims) |
| 15 Aug 1999 | Vajpayee at the Red Fort as Prime Minister | `in_vajpayee_styled_pm_independence_day_19990815` (claim) |
| 13 Oct 1999 | Resignation accepted with effect from that day; reappointed with effect from 1030 hours (both notified on 26 Oct 1999); sworn in and assumed office; oath and Council captions; PMO recollection | `in_president_accepts_vajpayee_resignation_wef_19991013` (Vajpayee 1998 `until`); `in_vajpayee_appointed_pm_wef_19991013`, `in_vajpayee_assumes_office_statement_19991013` (Vajpayee 1999 `from`); `in_vajpayee_sworn_in_19991013`, `in_vajpayee_oath_of_office_19991013`, `in_vajpayee_with_new_council_19991013` (support); `in_pmo_vajpayee_took_charge_recalled_19991013` (claim) |
| 13 May 2004 | Resignation tendered, accepted and continuation requested (communique at 19:30 IST); Vajpayee's address | `in_vajpayee_tenders_resignation_20040513`, `in_kalam_accepts_vajpayee_resignation_20040513`, `in_vajpayee_requested_to_continue_20040513`, `in_pib_releases_president_communique_20040513`, `in_vajpayee_states_resignation_submitted_20040513` (claims) |
| 22 May 2004 | Communique of Manmohan Singh's appointment; the council's oaths | `in_mms_appointed_pm_communique_20040522` (Singh 2004 `attested_on`); `in_council_oath_administered_20040522` (claim) |
| 18 May 2009 | Resignation tendered, accepted and continuation requested | `in_mms_tenders_resignation_20090518`, `in_patil_accepts_mms_resignation_20090518`, `in_mms_requested_to_continue_20090518` (claims) |
| 20 May 2009 | Manmohan Singh appointed; oath announced for 22 May | `in_patil_appoints_mms_pm_20090520`, `in_mms_oath_announced_20090520` (claims) |
| 22 May 2009 | Communique of his appointment; the council's oaths | `in_mms_appointed_pm_communique_20090522` (Singh 2009 `attested_on`); `in_council_oath_administered_20090522` (claim) |
| 17 May 2014 | Resignation tendered, accepted and continuation requested | `in_mms_tenders_resignation_20140517`, `in_pranab_accepts_mms_resignation_20140517`, `in_mms_requested_to_continue_20140517` (claims) |
| 20 May 2014 | Modi appointed; oath fixed for 26 May at 18:00 | `in_pranab_appoints_modi_pm_20140520`, `in_modi_oath_announced_20140520` (claims) |
| 26 May 2014 | Communique of his appointment; the council's oaths; sworn in at 1800 hours | `in_modi_appointed_pm_communique_20140526`, `in_cabsec_modi_sworn_in_as_pm_20140526` (Modi 2014 `attested_on`); `in_council_oath_administered_20140526` (claim) |
| 24 May 2019 | Resignation tendered, accepted and continuation requested | `in_modi_tenders_resignation_20190524`, `in_kovind_accepts_modi_resignation_20190524`, `in_modi_requested_to_continue_20190524` (claims) |
| 25 May 2019 | Modi appointed under Article 75(1) | `in_kovind_appoints_modi_pm_20190525` (claim) |
| 26 May 2019 | Oath announced for 30 May at 7.00 p.m. | `in_modi_oath_announced_20190526` (claim) |
| 30 May 2019 | Communique of his appointment and oath; sworn in at 1900 hours | `in_modi_appointed_pm_communique_20190530`, `in_modi_oath_administered_20190530`, `in_cabsec_modi_sworn_in_as_pm_20190530`; Modi 2019 `attested_on` |
| 5 Jun 2024 | Resignation tendered, accepted and continuation requested | `in_modi_tenders_resignation_20240605`, `in_murmu_accepts_modi_resignation_20240605`, `in_modi_requested_to_continue_20240605` (claims) |
| 7 Jun 2024 | Modi appointed under Article 75(1); oath announced for 9 June at 7:15 pm | `in_murmu_appoints_modi_pm_20240607`, `in_modi_oath_announced_20240607` (claims) |
| 9 Jun 2024 | Communique of his appointment; the council's oaths; sworn in at 1915 hours | `in_modi_appointed_pm_communique_20240609`, `in_cabsec_modi_sworn_in_as_pm_20240609` (Modi 2024 `attested_on`); `in_council_oath_administered_20240609` (claim) |
| 23 Jun, 25 Jul 2026 | Cabinet Secretariat council lists | `in_cabsec_lists_modi_pm_20260623`, `in_cabsec_lists_modi_pm_20260725` (support) |
| 30 Aug 2026 | PMO joint statement on the state visit to Uzbekistan | `in_modi_pm_state_visit_uzbekistan_20260830` (support) |
| undated | PMO and PIB retrospective spans | eleven `*_span_*` claims (no structured date) |

Date conventions follow CLAUDE-C01-07 to C01-10: `attested_on` is the day of the observed event as the source dates it
(a letter by its date, a Gazette acceptance or appointment by its stated effect), and a page's issue date is
`published_date`. A retrospective statement is dated by the day it recalls. The eleven retrospective spans carry no
structured date, and the first-printed 1998 acceptance of Gujral's resignation carries none because it was superseded.

## Observations

### IN-PM-01 — The holder when the period opens

Evidence: the Lok Sabha Debates of 12 March 1990, the first sitting of 1990, record "THE PRIME MINISTER (SHRI
VISHWANATH PRATAP SINGH)" introducing the Minister of State for Defence (`in_vp_singh_pm_introduces_minister_19900312`).
On 7 November 1990 he moved the motion of confidence in the Council of Ministers
(`in_vp_singh_moves_confidence_motion_19901107`), which was negatived on a division, Ayes 142 and Noes 346, subject to
correction, and the House was adjourned sine die (`in_ls_confidence_motion_negatived_19901107`). The PMO's
former-prime-minister page heads his entry "December 2, 1989 - November 10, 1990"
(`in_pmo_span_vp_singh_19891202_19901110`; check A9).

Decision: accepted in part. His holder is observed on 12 March 1990, the earliest 1990 attestation pinned to a
byte-stable response, with no start or end. The 7 November motion and the vote are claims on the role.

Limits: no accessible primary source states his resignation, the President's acceptance, a request to continue or the
day his office ended. Gazette Extraordinary Part I Section 2 Nos. 15 and 16 of 1990 (published between 13 and 23
November) were never digitized in the eGazette series (the neighbouring issues' identifiers 24033 and 24034 are
consecutive), and the text of 120 extraordinary issues of 9-30 November 1990 has no notification naming the Prime
Minister. The Rajya Sabha debates store has no sitting between 1 November and 26 December 1990 and no relevant paper laid
on 27-28 December 1990 (check A13). The PIB archive of the President's and Prime Minister's releases refused connections
from the research network.

### IN-PM-02 — Chandra Shekhar, 1990-1991

Evidence: on 16 November 1990 "THE PRIME MINISTER (SHRI CHANDRA SHEKHAR)" moved the motion of confidence
(`in_chandra_shekhar_moves_confidence_motion_19901116`), carried on a division at 19.13 hrs, Ayes 269 and Noes 204,
subject to correction (`in_ls_confidence_motion_adopted_19901116`; PDF pages 98 and 107, check A2). The Rajya Sabha
records him introducing Ministers as Prime Minister on 27 December 1990
(`in_chandra_shekhar_introduces_ministers_rs_19901227`). On 6 March 1991, replying under "Re. Constitutional Crisis in
the Country", he told the Lok Sabha that his Government would resign, and the Speaker adjourned the House at 14.00 hrs
until the next day (`in_chandra_shekhar_announces_resignation_19910306`; check A3); in the Rajya Sabha the Leader of the
House said the Prime Minister was on his way to Rashtrapati Bhavan to tender his resignation
(`in_rs_leader_announces_pm_resignation_19910306`). On 7 March 1991 the Rajya Sabha laid, as papers received from the
Secretary to the President, his letter of 6 March tendering his and his Council's resignation (LT-2215/91) and the
President's letter of 6 March accepting it and "requesting them to continue in Office till a new Government is formed"
(LT-2216/91) (`in_rs_lays_chandra_shekhar_resignation_letter_19910306`, `in_rs_lays_president_acceptance_19910306`,
`in_rs_lays_continuation_request_19910306`); the Lok Sabha laid the same letters on 11 March, describing the request as
made to "him" (`in_chandra_shekhar_resignation_letter_19910306`, `in_president_accepts_chandra_shekhar_resignation_19910306`,
`in_president_requests_chandra_shekhar_continue_19910306`; check A12). On 4 June 1991 "THE PRIME MINISTER (SHRI CHANDRA
SHEKHAR)" spoke in the Rajya Sabha (`in_chandra_shekhar_pm_in_rajya_sabha_19910604`; check A11). The MHA notification
No. 10/4/91-M&G(i) of 5 July 1991 accepts his resignation of the office of Prime Minister, with the listed ministers, with
effect from 21 June 1991 (`in_president_accepts_chandra_shekhar_resignation_wef_19910621`). The PMO page gives
"November 10, 1990 - June 21, 1991" (`in_pmo_span_chandra_shekhar_19901110_19910621`).

Decision: accepted in part. His holder is observed on 16 November 1990 and ends on 21 June 1991, the stated effect of the
acceptance. The service from 6 March to 21 June 1991 at the President's request is claims only. The proposed claim that
the notification "ends the continuation" was withdrawn, because the notification mentions neither the March letter nor
the request; the reading is kept in the acceptance claim's uncertainty and the holder note (check A1).

Limits: the day he was appointed or sworn in is stated only by the PMO's retrospective list (10 November 1990); the 1990
Gazette issues that may hold it were never digitized. The 16 November 1990 and 11 March 1991 Lok Sabha records are
third-party mirror copies with nothing tying them to the official host (check A8); the Rajya Sabha records of
27 December 1990 and 7 March 1991 are on the Rajya Sabha's own store.

### IN-PM-03 — P. V. Narasimha Rao, 1991-1996

Evidence: the MHA notification No. 10/4/91-M&G(ii) of 5 July 1991 appoints him Prime Minister with effect from 12.50 P.M.
of 21 June 1991 (`in_rao_appointed_pm_wef_19910621`), and No. 10/5/96-M&G(i) of 24 May 1996 accepts his resignation of
the office, with the listed ministers, with effect from 16 May 1996 (`in_president_accepts_rao_resignation_wef_19960516`;
signature on PDF page 3, check A4). Ruling on 28 May 1996 on when a Prime Minister is recognised as Leader of the House,
the Speaker said that the Rao Government was sworn in on 21 June 1991 (`in_rao_swearing_in_recalled_by_speaker_19910621`;
check A10).

Decision: accepted in part. Rao's holder starts on 21 June 1991 and ends on 16 May 1996. The Speaker's statement is a
retrospective claim and not the basis of the start.

Limits: the day he tendered the 1996 resignation, the President's acceptance letter and any request to continue until
16 May 1996 were not found; a monitoring-service report of the 1996 acceptance is a lead only.

### IN-PM-04 — 1996: Vajpayee and Deve Gowda

Evidence: the MHA notification No. 10/5/96-M&G of 24 May 1996 (Gazette Extraordinary Part I Section 2 No. 13) appoints
Atal Bihari Vajpayee Prime Minister with effect from 12 noon of 16 May 1996 (`in_vajpayee_appointed_pm_wef_19960516`;
the English line is cropped in the scan and the Hindi text is complete; check B1). He introduced his Council of Ministers
as Prime Minister on 23 May (`in_vajpayee_introduces_council_as_pm_19960523`) and moved the motion of confidence
debated on 27 May (`in_vajpayee_moves_confidence_motion_19960527`), when a member said the government was sworn in on
the 16th (`in_vajpayee_swearing_in_recalled_in_house_19960516`). At 17:05 on 28 May the Speaker referred to the
resignation the Prime Minister had announced on the floor, held that putting the motion to the vote had become
infructuous and adjourned sine die (`in_vajpayee_resignation_announced_in_house_19960528`,
`in_vajpayee_confidence_motion_not_put_to_vote_19960528`). On 11 June 1996 the Speaker introduced the Prime Minister,
H. D. Deve Gowda (`in_deve_gowda_introduced_as_pm_19960611`), and the same member said he was sworn in on 1 June
(`in_deve_gowda_swearing_in_recalled_in_house_19960601`). The PMO pages (`in_pmo_span_vajpayee_19960516_19960601`,
`in_pmo_span_deve_gowda_19960601_19970421`) and PIB's 1998 profiles (`in_pib_profile_vajpayee_1996_span_19980316`,
`in_pib_profile_vajpayee_1996_span_19980319`) give 1996 spans that disagree on Vajpayee's last day (1 June against
31 May).

Decision: accepted in part. Vajpayee's 1996 holder starts on 16 May 1996 (check B4) and has no end. Deve Gowda's holder is
observed on 11 June 1996, the Speaker's same-day record, rather than on 1 June, which rests only on a member's statement
(check B4); his end is set in IN-PM-05.

Limits: no 1996 resignation letter, acceptance, request to continue or stated end was found for Vajpayee, and no
appointment notification for Deve Gowda (Gazette Extraordinary Part I Section 2 Nos. 15 and 16 of 1996 are not in the
mirror; No. 17 is of 9 July 1996). Vajpayee's own statement of 28 May is in Hindi in a legacy font encoding and could not
be read.

### IN-PM-05 — 1997-1998: Deve Gowda and Gujral

Evidence: on 11 April 1997 the Lok Sabha debated Deve Gowda's motion of confidence
(`in_deve_gowda_moves_confidence_motion_19970411`) and negatived it, Ayes 158 and Noes 292, subject to correction
(`in_deve_gowda_confidence_motion_negatived_19970411`). Papers laid on 22 April 1997 give his letter of 11 April tendering
his resignation and the President's letter of 12 April accepting it and requesting him to continue till alternative
arrangements are made (`in_deve_gowda_resignation_tendered_19970411`, `in_president_accepts_deve_gowda_resignation_19970412`,
`in_deve_gowda_requested_to_continue_19970412`). At 11:04 on 21 April 1997 the Speaker said the swearing-in ceremony had
just concluded (`in_gujral_swearing_in_concluded_19970421`) and at 11:31 introduced the Prime Minister, I. K. Gujral
(`in_gujral_introduced_as_pm_19970421`). The MHA notifications of 29 April 1997 accept Deve Gowda's resignation with
effect from 21 April 1997 (`in_president_accepts_deve_gowda_resignation_wef_19970421`) and appoint Gujral with effect from
10.00 hours of 21 April 1997 (`in_gujral_appointed_pm_wef_19970421`). Papers laid on 2 December 1997 give Gujral's letter
and the President's acceptance with a request to continue, both of 28 November 1997
(`in_gujral_resignation_tendered_19971128`, `in_president_accepts_gujral_resignation_19971128`,
`in_gujral_requested_to_continue_19971128`); PIB still styled him Prime Minister on 12 March 1998
(`in_gujral_styled_pm_holi_greetings_19980312`). The notification of 6 April 1998 first printed his resignation as
accepted "with effect from 19th March, 1998" (`in_gujral_resignation_accepted_wef_superseded_19980406`), and the
corrigendum of 14 August 1998 replaced those lines (`in_gazette_corrigendum_gujral_acceptance_19980814`). The PMO page
recalls his swearing-in on 21 April 1997 and gives a span to 19 March 1998 (`in_pmo_gujral_swearing_in_recalled_19970421`,
`in_pmo_span_gujral_19970421_19980319`).

Decision: accepted in part. Deve Gowda's holder ends on 21 April 1997 and Gujral's starts that day (check B1). Gujral has
no end (check B2).

Limits: no source states the day Gujral's continuation ended. The 1997 division figures were announced subject to
correction, and the corrected figures were not seen.

### IN-PM-06 — 1998-1999: Vajpayee's second and third terms

Evidence: PIB reported on 16 March 1998 that the President had invited Vajpayee to form the government
(`in_president_invites_vajpayee_to_form_government_19980316`). The MHA notification No. 10/1/98-M&G(i) of 6 April 1998
appoints him with effect from 0930 hours of 19 March 1998 (the Hindi text says 09.00; `in_vajpayee_appointed_pm_wef_19980319`;
check B8). PIB reported him sworn in "here today" on 19 March, captioned the President administering the oath of office,
and listed him first in the Council of Ministers (`in_vajpayee_sworn_in_19980319`, `in_vajpayee_oath_of_office_19980319`,
`in_vajpayee_heads_council_list_19980319`; check B6). On 17 April 1999 the Lok Sabha negatived the confidence motion,
Ayes 269 and Noes 270 (`in_vajpayee_confidence_motion_negatived_19990417`); papers laid on 19 April give his letter and the
President's acceptance with a request to continue, both of 17 April (`in_vajpayee_resignation_tendered_19990417`,
`in_president_accepts_vajpayee_resignation_19990417`, `in_vajpayee_requested_to_continue_19990417`), and PIB captioned
him as Prime Minister at the Red Fort on 15 August 1999 (`in_vajpayee_styled_pm_independence_day_19990815`). The MHA
notifications of 26 October 1999 accept his resignation with effect from 13 October 1999
(`in_president_accepts_vajpayee_resignation_wef_19991013`) and appoint him again with effect from 1030 hours of that day
(`in_vajpayee_appointed_pm_wef_19991013`). PIB's release of 13 October 1999 reports him sworn in and that he "has assumed
the august office of the Prime Minister of India for the third time" (`in_vajpayee_sworn_in_19991013`,
`in_vajpayee_assumes_office_statement_19991013`; check B5), and its captions show the oath and the new Council
(`in_vajpayee_oath_of_office_19991013`, `in_vajpayee_with_new_council_19991013`). PIB's and the PMO's retrospective
statements (`in_pib_profile_vajpayee_spans_19991013`, `in_pmo_vajpayee_took_charge_recalled_19991013`,
`in_pmo_span_vajpayee_19980319_20040522`) are claims.

Decision: accepted (check B7). The 1998 holder runs from 19 March 1998 to 13 October 1999, and the 1999 holder starts on
13 October 1999. The continuation from 17 April to 13 October 1999 is claims only.

Limits: the first session of the 13th Lok Sabha (October 1999) has no archived text, and the English and Hindi hours of
the 1998 appointment differ.

### IN-PM-07 — 2004: Vajpayee to Manmohan Singh

Evidence: the President's communique of 13 May 2004 states that Vajpayee tendered his and his colleagues' resignation,
that the President accepted it and that he asked them to continue till alternative arrangements are made
(`in_vajpayee_tenders_resignation_20040513`, `in_kalam_accepts_vajpayee_resignation_20040513`,
`in_vajpayee_requested_to_continue_20040513`); PIB's copy dates it 19:30 IST (`in_pib_releases_president_communique_20040513`;
check C5), and Vajpayee's address that evening says "This evening, I submitted my resignation"
(`in_vajpayee_states_resignation_submitted_20040513`). The President's Secretariat communique of 22 May 2004 states that
the President has appointed Dr. Manmohan Singh as the Prime Minister (`in_mms_appointed_pm_communique_20040522`) and that
the oaths were administered that day to "the above members of the Council of Ministers"
(`in_council_oath_administered_20040522`; check C1). The PMO's list gives Vajpayee to 22 May 2004 and Singh from 22 May
2004 to 26 May 2014 (`in_pmo_list_span_vajpayee_19980319_20040522`, `in_pmo_list_span_mms_20040522_20140526`; check C2).

Decision: accepted in part. Manmohan Singh's 2004 holder is observed on 22 May 2004. Vajpayee's 1999 holder gets no end;
his 13 May claims are resignation-day claims on the role and date no holder (check C3).

Limits: the President's appointment of Dr. Manmohan Singh on 19 May 2004 (PIB relid 1734) was read live, but the page
changes its SHA-256 on every request and has no archive capture, so it is a lead. No source states the day Vajpayee's
continuation ended except the PMO's retrospective list.

### IN-PM-08 — 2009: Manmohan Singh's second term

Evidence: the President's communique of 18 May 2009 records his resignation, its acceptance and the request to continue
(`in_mms_tenders_resignation_20090518`, `in_patil_accepts_mms_resignation_20090518`,
`in_mms_requested_to_continue_20090518`); on 20 May the President appointed him and announced the oath for 22 May
(`in_patil_appoints_mms_pm_20090520`, `in_mms_oath_announced_20090520`); the communique of 22 May 2009 states his
appointment and the council's oaths (`in_mms_appointed_pm_communique_20090522`, `in_council_oath_administered_20090522`).

Decision: accepted in part. His 2009 holder is observed on 22 May 2009. The 2004 holder has no end: his continuation after
the 18 May acceptance is recorded only as claims, and no source states when it ended (check C4).

Limits: no Cabinet Secretariat or Gazette record of the 2009 oath or of a day of effect was found.

### IN-PM-09 — 2014 and 2019

Evidence: for 2014, the resignation, acceptance and request to continue of 17 May (`in_mms_tenders_resignation_20140517`,
`in_pranab_accepts_mms_resignation_20140517`, `in_mms_requested_to_continue_20140517`); Modi's appointment of 20 May with
the oath fixed for 26 May at 18:00 (`in_pranab_appoints_modi_pm_20140520`, `in_modi_oath_announced_20140520`); the
communique of 26 May (`in_modi_appointed_pm_communique_20140526`, `in_council_oath_administered_20140526`); and the
Cabinet Secretariat memorandum that he was sworn in as Prime Minister at 1800 hours on 26 May 2014, read from the rendered
page because the text layer misreads the day (`in_cabsec_modi_sworn_in_as_pm_20140526`). For 2019, the communiques of
24 May (resignation, acceptance, request to continue), 25 May (appointment under Article 75(1)), 26 May (the oath
announced for 30 May at 7.00 p.m.; check C6) and 30 May (appointment and oaths, headed as the oath to the Prime Minister)
(`in_modi_tenders_resignation_20190524`, `in_kovind_accepts_modi_resignation_20190524`,
`in_modi_requested_to_continue_20190524`, `in_kovind_appoints_modi_pm_20190525`, `in_modi_oath_announced_20190526`,
`in_modi_appointed_pm_communique_20190530`, `in_modi_oath_administered_20190530`), all from 2019 captures of the
President's website (check C7), and the Cabinet Secretariat memorandum that he was sworn in at 1900 hours
(`in_cabsec_modi_sworn_in_as_pm_20190530`).

Decision: accepted in part. Modi's holders are observed on 26 May 2014 and 30 May 2019, with no start (each appointment
was made while the predecessor or Modi himself continued in office) and no end.

Limits: the end of Manmohan Singh's 2014 continuation is stated only by the PMO's retrospective list (26 May 2014), and
the end of Modi's 2019 continuation by nothing.

### IN-PM-10 — 2024, and in office before the cutoff

Evidence: the communiques of 5 June 2024 (resignation, acceptance and request to continue), 7 June (appointment under
Article 75(1), and the oath announced for 9 June at 7:15 pm) and 9 June (appointment and the council's oaths)
(`in_modi_tenders_resignation_20240605`, `in_murmu_accepts_modi_resignation_20240605`,
`in_modi_requested_to_continue_20240605`, `in_murmu_appoints_modi_pm_20240607`, `in_modi_oath_announced_20240607`,
`in_modi_appointed_pm_communique_20240609`, `in_council_oath_administered_20240609`), and the Cabinet Secretariat
memorandum that he was sworn in at 1915 hours on 9 June 2024 (`in_cabsec_modi_sworn_in_as_pm_20240609`). In 2026: the
Cabinet Secretariat council lists as on 23 June and 25 July 2026 (`in_cabsec_lists_modi_pm_20260623`,
`in_cabsec_lists_modi_pm_20260725`; check C8) and the PMO's joint statement of 30 August 2026 on the state visit to
Uzbekistan (`in_modi_pm_state_visit_uzbekistan_20260830`).

Decision: accepted in part. Modi's 2024 holder is observed on 9 June 2024 and carries the 2026 attestations, which neither
end nor extend any term.

Limits: no source states when the continuation after 5 June 2024 ended or a day from which the 2024 appointment took
effect.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `in_ls_debates_19900312` | Lok Sabha Debates, 12 Mar 1990: V. P. Singh introduces a Minister as Prime Minister | capture 2021-05-19 of eparlib.nic.in; PDF pages 1, 28 viewed |
| `in_ls_debates_19901107` | Lok Sabha Debates, 7 Nov 1990: confidence motion moved and negatived | capture 2024-09-28 of eparlib.nic.in; PDF pages 27, 36, 133, 144 viewed |
| `in_pmo_former_pm_vp_singh` | PMO former-PM page: V. P. Singh (retrospective span) | capture 2025-11-07 of www.pmindia.gov.in |
| `in_ls_debates_19901116` | Lok Sabha Debates, 16 Nov 1990: Chandra Shekhar moves the confidence motion; adopted | Internet Archive item copy (mirror); PDF pages 33, 98, 107 viewed |
| `in_rs_debates_19901227` | Rajya Sabha Debates, 27 Dec 1990: Chandra Shekhar introduces Ministers | official file, bucketapi.rajyasabha.digital; PDF page 1 viewed |
| `in_ls_debates_19910306` | Lok Sabha Debates, 6 Mar 1991: resignation announced | Internet Archive item copy (mirror); PDF pages 405, 406, 410 viewed |
| `in_rs_debates_19910306` | Rajya Sabha Debates, 6 Mar 1991: the Leader of the House on the Prime Minister's resignation | official file, bucketapi.rajyasabha.digital; PDF pages 1, 2 viewed |
| `in_rs_papers_19910307` | Rajya Sabha Debates, 7 Mar 1991: the resignation letter and the President's letter laid | official file, bucketapi.rajyasabha.digital; PDF page 1 viewed |
| `in_ls_debates_19910311` | Lok Sabha Debates, 11 Mar 1991: the resignation letter and the President's letter laid | Internet Archive item copy (mirror); PDF page 33 viewed |
| `in_rs_debates_19910604` | Rajya Sabha Debates, 4 Jun 1991: the Prime Minister replies in the House | official file, bucketapi.rajyasabha.digital; PDF page 3 viewed |
| `in_gazette_ext_p1s2_no15_19910705` | Gazette Extraordinary, 5 Jul 1991: Chandra Shekhar's resignation accepted w.e.f. 21 Jun 1991 | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_pmo_former_pm_chandra_shekhar` | PMO former-PM page: Chandra Shekhar (retrospective span) | capture 2026-04-11 of www.pmindia.gov.in |
| `in_gazette_ext_p1s2_no17_19910705` | Gazette Extraordinary, 5 Jul 1991: Rao appointed w.e.f. 12.50 P.M., 21 Jun 1991 | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_p1s2_no14_19960524` | Gazette Extraordinary, 24 May 1996: Rao's resignation accepted w.e.f. 16 May 1996 | official file, egazette.gov.in; PDF pages 1, 2, 3 viewed |
| `in_ls_debates_19960528_printed` | Lok Sabha Debates (printed), 28 May 1996: the Speaker's ruling recalling 21 Jun 1991 | capture 2025-05-13 of eparlib.nic.in; PDF page 9 viewed |
| `in_gazette_ext_p1s2_no13_19960524` | Gazette Extraordinary No. 13, 24 May 1996: Vajpayee appointed w.e.f. noon, 16 May 1996 | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_ls_debates_19960523_council_introduction` | Lok Sabha, 23 May 1996: Vajpayee introduces his Council | capture 2006-05-17 of parliamentofindia.nic.in |
| `in_ls_contents_19960527` | Lok Sabha contents, 27 May 1996: the confidence motion | capture 2004-11-22 of parliamentofindia.nic.in |
| `in_ls_debates_19960527_confidence_debate` | Lok Sabha, 27 May 1996: a member recalls the 16 May swearing-in | capture 2006-05-18 of parliamentofindia.nic.in |
| `in_ls_debates_19960528_confidence_debate_close` | Lok Sabha, 28 May 1996: resignation announced; motion not put | capture 2009-04-11 of parliamentofindia.nic.in |
| `in_ls_debates_19960611_pm_introduction` | Lok Sabha, 11 Jun 1996: the Speaker introduces Deve Gowda | capture 2006-05-19 of parliamentofindia.nic.in |
| `in_ls_debates_19960611_confidence_debate` | Lok Sabha, 11 Jun 1996: a member recalls the 1 Jun swearing-in | capture 2006-05-19 of parliamentofindia.nic.in |
| `in_pmo_former_pm_vajpayee_1996` | PMO former-PM page: Vajpayee, 1996 entry (retrospective span) | capture 2025-11-07 of www.pmindia.gov.in |
| `in_pmo_former_pm_deve_gowda` | PMO former-PM page: Deve Gowda (retrospective span) | capture 2026-05-15 of www.pmindia.gov.in |
| `in_ls_contents_19970411` | Lok Sabha contents, 11 Apr 1997: the confidence motion | capture 2004-10-27 of parliamentofindia.nic.in |
| `in_ls_debates_19970411_confidence_vote` | Lok Sabha, 11 Apr 1997: confidence motion negatived | capture 2009-04-11 of parliamentofindia.nic.in |
| `in_ls_debates_19970421_pm_introduction` | Lok Sabha, 21 Apr 1997: swearing-in concluded; Gujral introduced | capture 2009-04-11 of parliamentofindia.nic.in |
| `in_ls_debates_19970422_resignation_papers` | Lok Sabha, 22 Apr 1997: Deve Gowda's letter and the President's letter laid | capture 2004-11-05 of parliamentofindia.nic.in |
| `in_gazette_ext_p1s2_no13_19970429` | Gazette Extraordinary No. 13, 29 Apr 1997: Deve Gowda's resignation and Gujral's appointment w.e.f. 21 Apr 1997 | official file, egazette.gov.in; PDF pages 1, 2, 3 viewed |
| `in_ls_debates_19971202_resignation_papers` | Lok Sabha, 2 Dec 1997: Gujral's letter and the President's letter laid | capture 2006-05-17 of parliamentofindia.nic.in |
| `in_pib_19980312_releases` | PIB releases, 12 Mar 1998: the Prime Minister greets on Holi | capture 2002-03-11 of pib.nic.in |
| `in_pmo_former_pm_gujral` | PMO former-PM page: Gujral (recollection and retrospective span) | capture 2025-11-07 of www.pmindia.gov.in |
| `in_gazette_ext_p1s2_no28_19980814` | Gazette Extraordinary No. 28, 14 Aug 1998: corrigendum on Gujral's resignation | Internet Archive item copy (mirror); PDF page 1 viewed |
| `in_pib_19980316_releases` | PIB releases, 16 Mar 1998: the President's invitation; profile | capture 2001-03-06 of pib.nic.in |
| `in_pib_19980319_releases` | PIB releases, 19 Mar 1998: sworn in; Council list; profile | capture 2002-06-27 of pib.nic.in |
| `in_pib_photo_19980319_oath` | PIB photo caption, 19 Mar 1998: the oath of office | capture 2003-01-08 of pib.nic.in |
| `in_gazette_ext_p1s2_no6_19980406` | Gazette Extraordinary No. 6, 6 Apr 1998: Gujral's acceptance (superseded) and Vajpayee appointed w.e.f. 19 Mar 1998 | Internet Archive item copy (mirror); PDF pages 1, 2, 3 viewed |
| `in_ls_debates_19990417_confidence_vote` | Lok Sabha, 17 Apr 1999: confidence motion negatived 269-270 | capture 2004-11-23 of parliamentofindia.nic.in |
| `in_ls_debates_19990419_resignation_papers` | Lok Sabha, 19 Apr 1999: Vajpayee's letter and the President's letter laid | capture 2004-11-23 of parliamentofindia.nic.in |
| `in_pib_photo_19990815_red_fort` | PIB photo caption, 15 Aug 1999: the Prime Minister at the Red Fort | capture 2002-04-01 of pib.nic.in |
| `in_pib_photo_19991013_oath` | PIB photo caption, 13 Oct 1999: the oath of office | capture 2002-04-01 of pib.nic.in |
| `in_pib_photo_19991013_council` | PIB photo caption, 13 Oct 1999: the new Council of Ministers | capture 2002-04-01 of pib.nic.in |
| `in_pib_19991013_vajpayee_profile` | PIB releases, 13 Oct 1999: sworn in and assumed office; profile | capture 2002-03-16 of pib.nic.in |
| `in_gazette_ext_p1s2_no16_19991026` | Gazette Extraordinary No. 16, 26 Oct 1999: resignation accepted and reappointed w.e.f. 13 Oct 1999 | Internet Archive item copy (mirror); PDF pages 1, 3, 5 viewed |
| `in_pmo_former_pm_vajpayee_1998` | PMO former-PM page: Vajpayee, 1998-2004 entry (recollection and span) | capture 2025-11-07 of www.pmindia.gov.in |
| `in_rb_kalam_pm_resignation_accepted_20040513` | President's website (2004), communique of 13 May 2004 | capture 2004-06-29 of presidentofindia.nic.in |
| `in_pib_president_communique_vajpayee_20040513` | PIB, the President's communique of 13 May 2004 (19:30 IST) | capture 2014-03-28 of pib.nic.in |
| `in_pib_pmo_vajpayee_address_to_nation_20040513` | PIB (PMO), the Prime Minister's address of 13 May 2004 | capture 2004-08-19 of pib.nic.in |
| `in_pib_president_communique_council_20040522` | PIB, the President's communique of 22 May 2004 | capture 2004-08-11 of pib.nic.in |
| `in_pmo_former_prime_ministers_page` | PMO former-prime-ministers list (retrospective spans) | capture 2026-02-21 of www.pmindia.gov.in |
| `in_rb_patil_pm_resignation_20090518` | President's website (2009), communique of 18 May 2009 | capture 2009-06-19 of presidentofindia.nic.in |
| `in_rb_patil_appoints_mms_20090520` | President's website (2009), communique of 20 May 2009 | capture 2009-06-19 of presidentofindia.nic.in |
| `in_rb_patil_council_oath_20090522` | President's website (2009), communique of 22 May 2009 | capture 2009-06-19 of presidentofindia.nic.in |
| `in_rb_pranab_pm_resignation_20140517` | President's website (2014), communique of 17 May 2014 | capture 2014-05-20 of presidentofindia.nic.in |
| `in_rb_pranab_appoints_modi_20140520` | President's website (2014), communique of 20 May 2014 | capture 2014-05-23 of presidentofindia.nic.in |
| `in_rb_pranab_council_oath_20140526` | President's website (2014), communique of 26 May 2014 | capture 2014-05-29 of presidentofindia.nic.in |
| `in_cabsec_modi_sworn_in_20140526` | Cabinet Secretariat O.M., 26 May 2014: sworn in at 1800 hours | official file, cabsec.gov.in; PDF pages 1, 2 viewed |
| `in_rb_kovind_pm_resignation_20190524` | President's website (2019), communique of 24 May 2019 | capture 2019-05-29 of presidentofindia.nic.in |
| `in_rb_kovind_appoints_modi_20190525` | President's website (2019), communique of 25 May 2019 | capture 2019-07-16 of presidentofindia.nic.in |
| `in_rb_kovind_oath_schedule_20190526` | President's website (2019), communique of 26 May 2019 | capture 2019-07-16 of presidentofindia.nic.in |
| `in_rb_kovind_council_oath_20190530` | President's website (2019), communique of 30 May 2019 | capture 2019-07-16 of presidentofindia.nic.in |
| `in_cabsec_modi_sworn_in_20190530` | Cabinet Secretariat O.M., 30 May 2019: sworn in at 1900 hours | official file, cabsec.gov.in; PDF page 1 viewed |
| `in_rb_murmu_pm_resignation_20240605` | President's website, communique of 5 Jun 2024 | capture 2024-06-05 of www.presidentofindia.gov.in |
| `in_rb_murmu_appoints_modi_20240607` | President's website, communique of 7 Jun 2024 (appointment) | capture 2024-07-04 of www.presidentofindia.gov.in |
| `in_rb_murmu_oath_schedule_20240607` | President's website, communique of 7 Jun 2024 (oath announced) | capture 2024-07-03 of www.presidentofindia.gov.in |
| `in_rb_murmu_council_oath_20240609` | President's website, communique of 9 Jun 2024 | capture 2024-06-09 of www.presidentofindia.gov.in |
| `in_cabsec_modi_sworn_in_20240609` | Cabinet Secretariat O.M., 9 Jun 2024: sworn in at 1915 hours | official file, cabsec.gov.in; PDF pages 1, 2 viewed |
| `in_cabsec_council_list_20260623` | Cabinet Secretariat council list as on 23 Jun 2026 | official file, cabsec.gov.in; PDF page 1 viewed |
| `in_cabsec_council_list_20260725` | Cabinet Secretariat council list as on 25 Jul 2026 | official file, cabsec.gov.in; PDF page 1 viewed |
| `in_pmo_uzbekistan_joint_statement_20260830` | PMO joint statement, 30 Aug 2026 (Uzbekistan) | capture 2026-09-02 of www.pmindia.gov.in |

Fifty sources are raw Internet Archive captures (`id_` form) made before the cutoff; each records the capture URL as
`url`, the address inside it (without `:80`) as `original_url` and the capture time in its extract. Six are stored files
in Internet Archive items that mirror an official document (three Lok Sabha debate PDFs uploaded by a private account in
2025, and three Gazette PDFs of the Public.Resource.Org collection); each extract names what it mirrors in `mirror_of`.
Fourteen are official files: five eGazette PDFs, four Rajya Sabha debate PDFs and five Cabinet Secretariat PDFs.

Each new source has a derived factual extract under [sources/](sources/) in the packet's format
(`spheres-c01-derived-factual-table/v1`): one row per claim, keyed by `claim_id`, with `observation_id`
`in_prime_minister`, `review_observation`, `role_id` `in_pm`, `holder_name`, `role_title` "Prime Minister of India",
`event_kind`, `attested_on`, the claim's text and locator. The extract's own checksum is in the packet, separate from the
response hash. Original pages, PDFs, captions, images and renders are not checked in, and no photograph, emblem, seal or
signature is republished.

Source types: `primary_parliamentary_debate_pdf`, `primary_parliamentary_debate_pdf_archived`,
`primary_parliamentary_debate_pdf_third_party_mirror`, `primary_parliamentary_debate_html_archived`,
`primary_gazette_notification_pdf`, `primary_gazette_notification_pdf_archive_copy`, `primary_press_release_archived`,
`primary_press_photo_caption_archived`, `primary_president_communique_archived`, `primary_pmo_page_archived_retrospective`,
`primary_pmo_statement_archived`, `primary_cabinet_secretariat_memorandum_pdf` and
`primary_cabinet_secretariat_council_list_pdf`.

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least twice, at least 30 minutes apart or with a cache-busting query, with identical bytes, and no identity
is a page generated per request. How each was established:

- **Parts A and B (23 September).** The researchers downloaded each response twice, the second time 30-85 minutes later
  (Part A with a cache-busting query), and each check downloaded all of them again (Part A three times, at 17:26Z, 17:54Z
  and 18:04Z, with cache-busting queries; Part B at 11:39-11:54 PDT with no-cache headers, 52-95 minutes after the
  researcher). Every byte count and SHA-256 matched.
- **Part C (23-24 September).** The researcher downloaded each response at least twice, 30 minutes to 19 hours apart, and
  the check again on 24 September between 12:15Z and 12:30Z, the live files also with a cache-busting query. Every byte
  count and SHA-256 matched.
- **Records added by the checks.** Each was downloaded twice by its check with the same bytes. On 24 September this
  packet downloaded the Rajya Sabha files, the two PMO pages of 1990-1991, the 1996 printed debates, the 2019 captures,
  both 1990 raw captures and the 25 July 2026 council list once more (13:35Z-13:45Z), and re-hashed the checks' stored
  copies of the Gazette, PIB and 2019 files, with the same results. The two eGazette files that replace the part B check's
  Internet Archive copies (Nos. 13 of 1996 and 1997) were downloaded at 13:35Z and again with a cache-busting query at
  14:04Z-14:06Z (the last 30 minutes after the first), and are byte-identical to those copies.

Reproducibility traps met and avoided:

- The current presidentofindia.nic.in pages (Drupal) regenerate form and view tokens per request: two downloads an hour
  apart matched through an edge cache, then a download on 24 September kept the byte count and changed the SHA-256. The
  2009 and 2014 communiques are therefore recorded from captures of the static pages of those years, and the 2024 ones
  from 2024 captures.
- PIB's live release pages inject a per-request script: three downloads of relid 1734 in 13 minutes had the same size and
  three different SHA-256s, so it is not recorded.
- The Internet Archive serves the two pmindia.gov.in captures of 2026 gzip-encoded even when identity encoding is asked
  for; the recorded identity is the encoded body, and the decoded HTML's bytes and SHA-256 are recorded beside it.
- The eGazette, Rajya Sabha and Cabinet Secretariat files are static objects: a fixed Last-Modified and ETag (the Rajya
  Sabha store's ETag equals the file's MD5), PDF creation dates years before access or empty, and no per-request /ID.
- Wayback and the Internet Archive returned HTTP 429 or 620-byte "suspected bot" pages at times; those bodies were
  discarded, and no recorded identity is such a page.

| Source ID | Bytes | SHA-256 | Recorded response |
|---|---|---|---|
| `in_ls_debates_19900312` | 1,043,511 | `577512140043…5ab51b` | raw capture |
| `in_ls_debates_19901107` | 6,878,854 | `1a9a7964c8a8…7467a9` | raw capture |
| `in_pmo_former_pm_vp_singh` | 49,150 | `644d8d885f49…c01cdf` | raw capture |
| `in_ls_debates_19901116` | 6,599,374 | `49f3e736cf37…d385c4` | item copy |
| `in_rs_debates_19901227` | 125,860 | `69c9a4300dce…bddb85` | official file |
| `in_ls_debates_19910306` | 18,429,324 | `52d5806a8488…5c08e8` | item copy |
| `in_rs_debates_19910306` | 82,642 | `b5dce7e89d67…f1c11f` | official file |
| `in_rs_papers_19910307` | 11,469 | `6899ed53814d…a71410` | official file |
| `in_ls_debates_19910311` | 6,588,530 | `32eddc3a7648…882559` | item copy |
| `in_rs_debates_19910604` | 365,671 | `4e7b007de8b1…09bbda` | official file |
| `in_gazette_ext_p1s2_no15_19910705` | 66,039 | `c9c8e5c885d2…297099` | official file |
| `in_pmo_former_pm_chandra_shekhar` | 50,298 | `7338776db01c…ac2bee` | raw capture |
| `in_gazette_ext_p1s2_no17_19910705` | 100,967 | `4f7497fb3613…b7246d` | official file |
| `in_gazette_ext_p1s2_no14_19960524` | 96,678 | `2dcf9b380eb6…64465b` | official file |
| `in_ls_debates_19960528_printed` | 3,318,468 | `ee5d24c029fb…c85ed6` | raw capture |
| `in_gazette_ext_p1s2_no13_19960524` | 50,883 | `735caffcde49…f2080c` | official file |
| `in_ls_debates_19960523_council_introduction` | 3,769 | `f4c261b43f1b…265f9e` | raw capture |
| `in_ls_contents_19960527` | 5,013 | `ad9c75ed9f62…eb5258` | raw capture |
| `in_ls_debates_19960527_confidence_debate` | 20,037 | `ff6efff9ed3a…242344` | raw capture |
| `in_ls_debates_19960528_confidence_debate_close` | 17,497 | `805dac2b0421…d7e654` | raw capture; date anchor 3,095 bytes, `31c017d830fb…0742be` |
| `in_ls_debates_19960611_pm_introduction` | 2,436 | `f1f9b8b4bac1…4ecbaf` | raw capture |
| `in_ls_debates_19960611_confidence_debate` | 20,705 | `c8616a3a4750…d40f40` | raw capture; date anchor 4,377 bytes, `f0e218e283af…0f93c2` |
| `in_pmo_former_pm_vajpayee_1996` | 50,896 | `1c6a8478aa8d…f7d716` | raw capture |
| `in_pmo_former_pm_deve_gowda` | 53,596 | `6fa4edf635d4…edee42` | raw capture |
| `in_ls_contents_19970411` | 3,331 | `0e7a97297fd8…552bcd` | raw capture |
| `in_ls_debates_19970411_confidence_vote` | 18,288 | `564eb8f1f502…f6b41f` | raw capture |
| `in_ls_debates_19970421_pm_introduction` | 1,710 | `14a2eb14897b…6e2ed2` | raw capture |
| `in_ls_debates_19970422_resignation_papers` | 770 | `578786535c6a…d04bf6` | raw capture |
| `in_gazette_ext_p1s2_no13_19970429` | 119,649 | `e911c1de3967…7b23ab` | official file |
| `in_ls_debates_19971202_resignation_papers` | 1,490 | `eec9da5d0607…71dcba` | raw capture |
| `in_pib_19980312_releases` | 7,178 | `76e2c5de2ada…1b3598` | raw capture |
| `in_pmo_former_pm_gujral` | 52,144 | `7e4774192ffb…d04263` | raw capture |
| `in_gazette_ext_p1s2_no28_19980814` | 43,121 | `909711304306…432333` | item copy |
| `in_pib_19980316_releases` | 10,956 | `ed540ddb523c…631c0c` | raw capture |
| `in_pib_19980319_releases` | 10,344 | `bf038a4d633d…4c4bb7` | raw capture |
| `in_pib_photo_19980319_oath` | 2,279 | `a470249326b0…761066` | raw capture |
| `in_gazette_ext_p1s2_no6_19980406` | 125,176 | `48f8a0c28576…d898b5` | item copy |
| `in_ls_debates_19990417_confidence_vote` | 5,436 | `1e2e509fb841…c9bbf5` | raw capture; date anchor 937 bytes, `4b1c1bb87d50…dc0b81` |
| `in_ls_debates_19990419_resignation_papers` | 1,221 | `1f18018d1d82…ab7cae` | raw capture |
| `in_pib_photo_19990815_red_fort` | 1,812 | `666f9627142c…d1bd5b` | raw capture |
| `in_pib_photo_19991013_oath` | 1,857 | `9dadd19ca18d…1dc606` | raw capture |
| `in_pib_photo_19991013_council` | 1,863 | `27dbd01e28d5…d941cc` | raw capture |
| `in_pib_19991013_vajpayee_profile` | 16,058 | `19b0426323fb…7b4c01` | raw capture |
| `in_gazette_ext_p1s2_no16_19991026` | 188,787 | `a33a2822288c…24189a` | item copy |
| `in_pmo_former_pm_vajpayee_1998` | 50,761 | `cc51d96e45eb…90f86a` | raw capture |
| `in_rb_kalam_pm_resignation_accepted_20040513` | 16,811 | `578d03405ea6…dce643` | raw capture |
| `in_pib_president_communique_vajpayee_20040513` | 45,914 | `23af7200f26d…02d37d` | raw capture |
| `in_pib_pmo_vajpayee_address_to_nation_20040513` | 25,343 | `dd194bbe7ba7…7cd6ca` | raw capture |
| `in_pib_president_communique_council_20040522` | 51,513 | `b65478a12374…7201f8` | raw capture |
| `in_pmo_former_prime_ministers_page` | 14,207 | `9c191ce2b6aa…17f69d` | raw capture; gzip body (decoded 71,386 bytes, `71dac5225f5f…ea94ef`) |
| `in_rb_patil_pm_resignation_20090518` | 3,027 | `355fdbc71d92…0df225` | raw capture |
| `in_rb_patil_appoints_mms_20090520` | 3,751 | `289ef715b8ce…3ce6a0` | raw capture |
| `in_rb_patil_council_oath_20090522` | 3,889 | `7d28e8f2438f…bb563a` | raw capture |
| `in_rb_pranab_pm_resignation_20140517` | 5,669 | `748d3ceb518d…63d37e` | raw capture |
| `in_rb_pranab_appoints_modi_20140520` | 6,626 | `9c5c4cfd89ac…a70a69` | raw capture |
| `in_rb_pranab_council_oath_20140526` | 8,329 | `3d04a0f4a1dc…78a7fb` | raw capture |
| `in_cabsec_modi_sworn_in_20140526` | 93,288 | `8deb95c028cd…9f9255` | official file |
| `in_rb_kovind_pm_resignation_20190524` | 60,746 | `fcf4df3d1c5e…031224` | raw capture |
| `in_rb_kovind_appoints_modi_20190525` | 63,689 | `c76eaf2adfa1…3a0be9` | raw capture |
| `in_rb_kovind_oath_schedule_20190526` | 60,735 | `cadbecef6609…88c38c` | raw capture |
| `in_rb_kovind_council_oath_20190530` | 77,506 | `423f780579d5…da73fb` | raw capture |
| `in_cabsec_modi_sworn_in_20190530` | 963,210 | `01af62f34e6d…4ddadf` | official file |
| `in_rb_murmu_pm_resignation_20240605` | 32,505 | `a86f194b9f20…45ee88` | raw capture |
| `in_rb_murmu_appoints_modi_20240607` | 43,211 | `c2ce4bd3f6e6…b25cc4` | raw capture |
| `in_rb_murmu_oath_schedule_20240607` | 40,937 | `0627952fbe63…3a8456` | raw capture |
| `in_rb_murmu_council_oath_20240609` | 43,520 | `0210454a3b53…7352d3` | raw capture |
| `in_cabsec_modi_sworn_in_20240609` | 1,515,782 | `1d45c3c0dd8a…a11bf8` | official file |
| `in_cabsec_council_list_20260623` | 208,003 | `13df919e710e…45f0e1` | official file |
| `in_cabsec_council_list_20260725` | 2,037,089 | `9b3ea4fc75cf…811371` | official file |
| `in_pmo_uzbekistan_joint_statement_20260830` | 19,526 | `17a56d1e9f8b…239ab2` | raw capture; gzip body (decoded 81,083 bytes, `4a07b0f399e7…d87884`) |

## Leads not imported

- Foreign Broadcast Information Service daily reports of 13 November 1990 (https://archive.org/details/micro_IA1176910_0664)
  and 13 May 1996 (https://archive.org/details/micro_IA1176911_0538): Delhi radio reports of Chandra Shekhar's swearing-in
  and of the President accepting Rao's resignation and asking him to continue. A foreign monitoring service's
  translations, not Indian official records.
- Lok Sabha Debates of 27 December 1990 (https://archive.org/download/eparlib.nic.in.3196/lsd_09_06_27-12-1990.pdf):
  members' statements that the President invited Chandra Shekhar, with no date and no letters laid.
- Journal of Parliamentary Information, June 1996 (https://archive.org/download/eparlib.nic.in.764738/jpi_june_1996.pdf):
  a later Lok Sabha Secretariat summary of the 1996 changes, with no entry on Rao's resignation.
- Gazette Extraordinary Part I Section 2 Nos. 14, 17, 18 and 19 of 1990 (E-0575-1990-0014-24033 to E-0575-1990-0019-24036):
  ministers' appointments and resignations and an Additional Solicitor General's resignation; none names the Prime
  Minister.
- The President's Address of 25 October 1999 in a later compilation
  (https://web.archive.org/web/20201114121549id_/https://eparlib.nic.in/bitstream/123456789/4004/1/narayanan_25_10_1999.pdf):
  a compiled editorial header, later than the events.
- The printed Hindi debates of 28 May 1996 (lsd_11_01_28-05-1996_hindi): the only capture is truncated at 1 MiB and does not
  open. The printed debates of 11 April 1997 (lsd_11_4_11-04-1997, about 7.5 MB): not downloaded under the scratch-disk
  budget; they would give the corrected division figures.
- PIB of 17 March 1998 (PIBR170398: a minister greets the "Prime Minister designate") and of January 1998 (PIBR130198 and
  others: Gujral styled Prime Minister): superseded by the 16 and 19 March releases and the 12 March attestation.
- Lok Sabha contents of 22 April 1997 (c220497, Gujral's motion of confidence) and 12 June 1996 (c120696, Deve Gowda's):
  not among the questions asked.
- PIB's 17 April 1999 gallery (pg17ap99/170499.html) and releases of 6 May and 4 June 1999 (r060599, r040699) and of 17 and
  19 April and 12 October 1999 (r170499, r190499, r121099): nothing on the Prime Minister's resignation, acceptance or
  continuation.
- PIB photo pages 1310993 (13 October 1999; repeats the 1310992 oath caption) and 1903982 (19 March 1998; the Prime Minister
  in his office after the swearing-in): redundant with the imported captions and PIB profiles.
- PIB relid 1734 (https://www.pib.gov.in/newsite/PrintRelease.aspx?relid=1734&reg=48&lang=2): the President's Secretariat
  communique of 19 May 2004 that the President appointed Dr. Manmohan Singh Prime Minister after Singh called on him with
  letters of support, the oath to be at a date and time Singh would indicate. Not reproducible (9,546 bytes each time, with
  three different SHA-256s beginning c2500343, 91f74713 and 26c05693) and never captured in any PIB URL form checked, so it
  is a lead and its date is not a claim.
- PIB relid 1703 (the Council of Ministers' resolution thanking Vajpayee), relid 1743 (portfolios, 23 May 2004), the
  President's 2004 release id 242 (a call on the President), the 10 June 2024 portfolio communique (press-communique-18)
  and Cabinet Secretariat memorandum (1_Upload_3871), and the 2024 dissolution and Election Commission communiques
  (press-communique-13 and -14): redundant or outside the office-holder scope.
- ramnathkovind.nic.in/pr260519.html (the 26 May 2019 heading over the 25 May dissolution text, a defective page) and
  pr250519.html (the dissolution): replaced by the 2019 capture of press-release-detail.htm?1614, or out of scope. The live
  ramnathkovind.nic.in copies of the 24, 25 and 30 May 2019 communiques are byte-stable but replaced by 2019 captures.
- The Cabinet Secretariat council list as on 24 July 2026 (1_Upload_4252.pdf, 1,826,391 bytes): the 25 July list is
  imported.
- The PMO's 15 August 2026 Independence Day address (pms-address-from-the-ramparts-of-red-fort...): one PMO attestation
  suffices.
- The PMO archive profiles (archivepmo.nic.in/abv/pmsprofile.php; archivepmo.nic.in/drmanmohansingh/pmsprofile.php):
  retrospective duplicates of the PMO pages.
- The current Drupal pages of the 2009 and 2014 communiques (presidentofindia.nic.in/pranab-mukherjee/press_releases/... and
  .../smt-pratibha-devisingh-patil/press_releases/...) and PIB's mirror of the 5 June 2024 communique
  (pib.gov.in/PressReleseDetailm.aspx?PRID=2022812): regenerated per request.
- News and encyclopaedias (cnn.com, deccanherald.com, newsonair.gov.in, and wikipedia.org's Premiership of Narendra Modi):
  leads only.
- The Constitution (Articles 74 and 75) frames the procedure; constitution text establishes procedure only, never a date.

## Sources attempted

- Parliament Digital Library (eparlib.nic.in, eparlib.sansad.in, loksabhadocs.nic.in): DNS failure or TCP connection
  failure from the research network. Lok Sabha PDFs were taken from raw captures of the eparlib URLs (1990 and 1996) or,
  where none exists, from the Internet Archive item copies, disclosed as mirror-only.
- PIB archive (archive.pib.gov.in; pibarchive.nic.in): port 443 refused and the browser navigation denied; pibarchive.nic.in
  does not resolve. The President's and Prime Minister's releases of November 1990, March and June 1991, May 1996,
  1996-1997 and April 1999 remain unchecked; PIB's 1998-1999 releases were read through captures of
  pib.nic.in/archieve/lreleng/, whose 1999 releases sit at lMMYY/rDDMMYY.html (check B5).
- eGazette search (egazette.gov.in/SearchMenu.aspx): an ASP.NET session application that redirected to error.aspx or hung;
  no CAPTCHA was met or attempted. Gazette files were found by their stored names and through the Internet Archive's Gazette
  of India collection; egazette.nic.in no longer resolves.
- Gazette Extraordinary Part I Section 2 Nos. 15 and 16 of 1990: never digitized in the eGazette series. Nos. 15 and 16 of
  1996: not in the mirror.
- Rajya Sabha debates (rsdebate.nic.in, now the Rajya Sabha debates service): searched by the part A check; no records
  from 1 November to 26 December 1990 and no relevant papers laid on 27-28 December 1990 (check A13).
- PMO archive (archivepmo.nic.in/abv/parliament_searchresult.php): the year search returned an empty body.
- Internet Archive: no captures of PIB's 1996-1997 releases, of the 13th Lok Sabha's first session, of PIB relids 1730-1737
  in the old release.asp form or of the President's 2004 release ids 230-240; the President's website is first captured in
  2002. Several 2024 communique captures are 403 challenge pages; only 200 captures are used. Rate limiting (HTTP 429) and
  "Temporarily Offline" replies were retried one request at a time.
- The President's current Drupal site: live pages regenerate per request (see the stability section).
- Presidential static archive sites (pranabmukherjee.nic.in, pratibhapatil.nic.in, abdulkalam.nic.in): do not resolve.
- The live PMO former-prime-minister index (pmindia.gov.in/en/former_pm/): 404; the per-prime-minister pages were used
  through captures.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | `in_chandra_shekhar_continuation_ends_19910621` paraphrases beyond the notification | **Resolved by removal**: the claim is withdrawn; the acceptance claim's uncertainty and the holder note carry the packet's reading that the stated effect also ends the continuation; `until` is still 21 June 1991 |
| A2 | The 16 Nov 1990 division locator is incomplete | **Applied**: PDF pages 98 and 107, columns 151-152 and 169-170 |
| A3 | The 6 Mar 1991 locator and "label not legible" | **Applied**: item "Re. Constitutional Crisis in the Country", speaker label on PDF p.405 (col 804), adjournment to 11.00 on 7 March; the sentence removed |
| A4 | The 1996 Gazette signature is on PDF page 3 | **Applied**: locator pages 2 and 3 |
| A5 | 12 Mar and 7 Nov 1990 recorded from a third-party upload | **Applied**: the raw captures of the eparlib URLs (20210519104239, 20240928132810) are recorded, byte-identical; the item copies are named in the scope notes |
| A6 | Item metadata and publisher wording | **Applied**: the scope notes say the items name the eparlib handle page and were uploaded by a private account in July 2025; the hashed responses are third-party mirrors; the publisher stays the Lok Sabha Secretariat as author |
| A7 | The 2022 capture of 6 Mar 1991 dismissed | **Applied**: recorded as partial origin corroboration (its truncated first MiB equals this copy's, and the declared length 18,429,324 matches); no full equality claimed |
| A8 | 16 Nov 1990 and 11 Mar 1991 have no official-host tie | **Applied**: mirror-only provenance disclosed; the Rajya Sabha papers of 7 Mar 1991 and attestation of 27 Dec 1990 imported as official-host records |
| A9 | The PMO pages' non-import rationale is wrong | **Applied**: both pages imported through pre-cutoff raw captures as retrospective spans with no structured date |
| A10 | Rao's 1991 oath "not separately attested" | **Applied**: `in_rao_swearing_in_recalled_by_speaker_19910621` (retrospective, claims only); the start stays on the Gazette |
| A11 | No attestation of the 1991 continuation | **Applied**: `in_chandra_shekhar_pm_in_rajya_sabha_19910604`, claims only; the Rajya Sabha data-service listing of 3 June is noted, not recorded |
| A12 | The him/them wording and the President's Secretariat channel | **Applied**: both noted in the request claims; the Rajya Sabha laying of 7 March cited as the earliest |
| A13 | The Rajya Sabha search is not recorded | **Applied**: the search and its negative results are in Sources attempted and IN-PM-01; four Rajya Sabha records imported |
| A14 | `source_type`, `rights_note` and `scope_note` missing | **Applied**: every new source has them; `name` appears only in holder records and `holder_name` in rows |
| B1 | Gazette notifications with five of six boundaries missed | **Applied**: five Gazette sources and eight claims; the boundaries of Vajpayee (1996, 1998, 1999), Deve Gowda and Gujral set from them; the issuing ministry named as Home Affairs |
| B2 | The Gujral trap: the first-printed "19th March, 1998" | **Applied**: the superseded text and the corrigendum kept as separate claims; Gujral `until` null |
| B3 | Retrospective spans carry `attested_period` | **Applied**: removed; spans are claims with no structured date (`attested_on` null in rows) |
| B4 | Two holders dated by a member's statement | **Applied**: Vajpayee 1996 dated by the Gazette start; Deve Gowda observed on 11 June 1996 (the Speaker's introduction) |
| B5 | Wrong "no capture" note for PIB 1999 | **Applied**: the lMMYY path recorded; r131099 imported with three claims; r170499, r190499 and r121099 recorded as negative |
| B6 | Photo pages never retried | **Applied**: 1310991 and 1903981 imported; the two redundant pages are leads |
| B7 | Decisions | **Applied**: IN-PM-06 accepted; IN-PM-04 and IN-PM-05 accepted in part, with the reasons restated |
| B8 | 0930 against 09.00 hours in 1998 | **Applied**: day only; the discrepancy is in the claim's uncertainty |
| B9 | Keep acceptances separate | **Applied**: the President's letters stay `resignation_accepted`; only the Gazette's `resignation_accepted_with_effect` claims give an `until` |
| C1 | Communique oath rows attributed to the Prime Minister | **Applied**: four rows renamed `in_council_oath_administered_*`, kind `council_oath_administered`, no holder name; the 2004 and 2009 holders dated by the appointment communiques; the 2019 headline and the Cabinet Secretariat memoranda keep `oath_of_office` |
| C2 | One span listed under two observations | **Applied**: the PMO list spans sit under IN-PM-07, and IN-PM-09 discusses the 2014 end |
| C3 | The holder shape differs from the Brazil model | **Applied**: holders cite only start, end and same-day attestation claims; resignations, acceptances, continuations, spans and oath announcements stay on the role; every holder has a note; the Vajpayee 2004 claims date no holder |
| C4 | Prose implies an end from the 2009 swearing-in | **Applied**: "his continuation in office after that acceptance is recorded only as claims, and no source states when it ended" |
| C5 | PIB relids 1704, 1730-1737 and 1734 misdescribed | **Applied**: notes corrected; relid 1704 imported (19:30 IST); relid 1734 recorded as a non-reproducible lead with its content |
| C6 | The 2019 oath announcement dropped | **Applied**: `in_modi_oath_announced_20190526` from the 2019 capture of id 1614 |
| C7 | Three 2019 sources are live pages | **Applied**: switched to the 2019 captures of ids 1610, 1613 and 1615; the byte-stable live copies named in the scope notes |
| C8 | The 23 June 2026 list called "current" | **Applied**: reworded; the 25 July 2026 list imported; the 24 July list a lead |
| C9 | gzip-encoded captures | **Applied**: each extract records the encoding and the decoded bytes and SHA-256, and its provenance note explains the pinned identity |
| C10 | "Press release" titles | **Applied**: three titles say "press communique" |

Missing primary records the checks found: all seven of part A are imported (the Rajya Sabha records of 27 December 1990,
6 and 7 March and 4 June 1991, the printed debates of 28 May 1996 and the PMO pages of V. P. Singh and Chandra Shekhar), and
part A's two origin-tied identities replace the mirror URLs. Of part B's nine, the five Gazette notifications, PIB's release
of 13 October 1999 and two captions (13 October 1999 with the new Council; the 19 March 1998 oath) are imported, and the
remaining caption is a redundant lead. Of part C's seven, the 2019 oath announcement, PIB relid 1704, the three 2019
captures and the 25 July 2026 list are imported; relid 1734 is a lead because it is not reproducible. Part A's
still-missing records (the PIB archive releases and the 1990 Gazette Nos. 15-16) are unresolved.

Other changes made to fit the packet's rules rather than a numbered defect:

- One event-kind vocabulary for all parts: the President's acceptance letters are `resignation_accepted`, requests are
  `continuation_requested`, attestations during a continuation are `continuation_in_office_attestation`, all retrospective
  spans are `retrospective_term_span`, the 1991, 1996 and Rajya Sabha announcements are `resignation_announced`, same-day
  oath and sworn-in reports are `oath_of_office` (the Speaker's 1997 report is `swearing_in_ceremony_reported`), and
  introductions and council lists are `in_office_attestation`.
- Part C's PMO list claims were renamed `in_pmo_list_span_*`, apart from the per-prime-minister PMO pages.
- The two eGazette files of 1996 and 1997 that the part B check found in the Internet Archive are recorded from
  egazette.gov.in, byte-identical, so that five of the eight Gazette sources are on the official host.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-India-PM-002`: 1990 — V. P. Singh's resignation, its acceptance and the day his office ended, and Chandra
  Shekhar's appointment day (the PIB archive releases of 7-10 November 1990, from a network that can reach
  archive.pib.gov.in).
- `C01-India-PM-003`: 1996 — Rao's resignation letter and any request to continue, Vajpayee's resignation instrument and
  end, and Deve Gowda's appointment notification (Gazette 1996 Part I Section 2 Nos. 15-16).
- `C01-India-PM-004`: 2004-2024 — Gazette of India notifications of appointments and accepted resignations, if any, with a
  stated effective day (eGazette search).
- `C01-India-PM-005`: the end of each continuation after 1999 (Vajpayee 2004, Singh 2009 and 2014, Modi 2019 and 2024).
- `C01-India-PM-006`: Manmohan Singh's July 2008 confidence vote, and the Lok Sabha's introductions of each Prime Minister
  after 1999.
- `C01-India-PM-007`: deputy prime ministers and acting arrangements, 1990-2026 (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Base and claim:** based on `ffe54b02` (`codex/campaign-certification`); claim commit `38f36fa2` holds only the
  handoff. No other pending packet touches `india.json`.
- `research-index.json` is regenerated in a **separate commit**. New totals against `ffe54b02`: 231 sources and 1,874
  claims (previously 161 and 1,762), 28 institution observations (previously 27). India now has one institution, one role
  observation and 83 entries pending mapping; its nine discovery batches become eight of 10 members and one of 3, so the
  total of 92 batches is unchanged. The pending Brazil (CLAUDE-C01-10) and South Africa (CLAUDE-C01-09) packets add their
  own sources and claims to other packets; the index must be regenerated after they merge.
- Pinned tests in `test_india_research_s10e.py`, none loosened and no assertion removed:
  - counts (entries, sources, claims, roles) are now (83, 72, 194, 1), from (82, 2, 82, 0), and the 82 organizations are
    pinned separately;
  - `institutions == []` is replaced by exactly one institution, `in_prime_minister`, with exactly one role, `in_pm`;
  - the Kerala host, the 2024-03-28 publication date, the 2026-09-13 access date, the 2024-03-23 claim dates, the
    notification and publication fields, the response sizes above 300,000 bytes and the row-to-organization
    reconciliation still apply to the original two sources by id; the 70 new sources are pinned to their five hosts and
    the 2026-09-23 and 2026-09-24 access dates with recorded identities (exact bytes and SHA-256 in
    `test_india_prime_ministers_c01_11.py`);
  - `mapping_pending` is 83 (from 82) and the India work orders are eight of 10 members and one of 3 (from 2), with the last
    batch pinned.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and
  the sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows only "Observed on" when a holder has `attested_on`, and would
  hide an end on such a holder. Two holders here have both: Chandra Shekhar (observed 16 November 1990, until 21 June 1991)
  and Deve Gowda (observed 11 June 1996, until 21 April 1997). Each note carries the end, and the new test pins that. No UI
  code changed.
- New fields on India sources: `original_url`, `source_type` and `scope_note`; extracts carry `review_observation`,
  `archive_capture_utc`, `mirror_of`, `date_anchor` (three undated Lok Sabha continuation pages dated by a recorded
  contents page), `source_response_content_encoding` with the decoded identity, and `visual_review`. The event-kind
  vocabulary is pinned in the new test.
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
