# ANC Presidents 16: the party office, 1990-2026

Packet: **CLAUDE-C01-16**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-za-16`, **stacked on CLAUDE-C01-09** (`claude/c01-za-09`
at `82a23f9d`, itself based on `codex/campaign-certification` at `ffe54b02`); claim commit `6e9a489b`. Research access:
24 September 2026 (local). The historical cutoff stays **7 September 2026**.

This packet adds one party role, `za_anc_president` (President of the African National Congress, kind `party_leader`),
to the existing `AFRICAN NATIONAL CONGRESS` organization observation (`za_iec_n2024_014`) in
[south-africa.json](south-africa.json), and reviews ten observations: the President when the period opens, the eight
National Conferences from 1991 to 2022, and an ANC attestation of its President before the cutoff. It adds 54 sources
and 73 claims, the role with ten holder observations (one per reviewed observation), a role scope note, and coverage
notes on the organization and the packet. The organization's IEC identity, unknown lifecycle and empty game mapping are
unchanged, and so is every CLAUDE-C01-09 role, claim and holder of `za_presidency`. It adds no organization,
institution, game mapping, lifespan, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1990-1994; B: 1997-2007; C: 2012-2026), and each part was checked
independently before this packet was written. Every checker defect is applied or explained below (see
[Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| ZA-ANC-01 | Who was ANC President when the period opens, and what 1990 acting or deputy arrangement do ANC records show? | **Accepted in part:** Oliver Tambo, observed 8 Jan 1990 (NEC statement) and again 2 Mar 1990 and 2 Jul 1991; no acting President named in any record reviewed; the NEC elected Mandela Deputy President on 1 or 2 Mar 1990 (claim only); no start or end |
| ZA-ANC-02 | July 1991: the 48th National Conference's election | **Accepted in part:** election reported on 13 Jul 1991 for a conference of 2-7 Jul (2-6 Jul on the ANC's documents pages); the closing address says "elected here today" and the ANC's later web edition heads it 6 Jul 1991; Mandela observed 18 Jul 1991 presiding over the NEC; day of the election, declaration and handover not stated |
| ZA-ANC-03 | December 1994: the 49th National Conference | **Accepted in part:** NEC list "as elected ... December 1994" (later web edition headed 20 Dec 1994, against voting results of 21 Dec for other members); Mandela observed 22 Dec 1994 in his closing address; election day not stated |
| ZA-ANC-04 | December 1997: the 50th National Conference | **Accepted in part:** Mbeki unanimously elected "on the second day of conference" (Mayibuye, March 1998; no calendar date stored); Mbeki observed 20 Dec 1997; Mandela's handover speech, the report's "departure" and the Declaration's farewell are claims, with no end |
| ZA-ANC-05 | December 2002: the 51st National Conference | **Accepted in part:** NEC list as elected at the conference of 16-20 Dec 2002; Mbeki observed 20 Dec 2002 in his closing statement; no results record or election day found |
| ZA-ANC-06 | December 2007: the 52nd National Conference | **Accepted in part:** nominations (undated pre-conference list), first round of voting 18 Dec, results expected that evening, results list published under 19 Dec (Zuma 2,329, Mbeki 1,505), acceptance 20 Dec; Zuma observed 20 Dec 2007; time of the declaration and Mbeki's end not stated |
| ZA-ANC-07 | December 2012: the 53rd National Conference | **Accepted in part:** Zuma "shortly after being elected" (ANC statement of 18 Dec 2012) and his "re-election" (19 Dec 2012); Zuma observed 20 Dec 2012 in his closing remarks; vote day, declaration and totals not found |
| ZA-ANC-08 | December 2017: the 54th National Conference | **Accepted in part:** the ANC biography says he was elected on 18 Dec 2017 (an election reference); Ramaphosa observed 20 Dec 2017; Zuma "Outgoing" and "Former", with no end; no declaration record or totals |
| ZA-ANC-09 | December 2022: the 55th National Conference | **Accepted in part:** Nasrec session 16-20 Dec 2022, concluded 5 Jan 2023 (Declaration); the election dated only "December 2022" (2026 statement); Ramaphosa observed 8 Jan 2023 |
| ZA-ANC-10 | An ANC attestation of its President before the cutoff | **Accepted:** the NEC's "full and continuing support" for its President, statement of 15 May 2026 (briefing on or before the evening of 14 May); Ramaphosa observed 15 May 2026; no acting arrangement or vacancy found |

The resulting holder observations of `za_anc_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Oliver Tambo | 1990-01-08 | null | null | NEC anniversary statement: "The President of your movement, the ANC, Comrade Oliver Tambo" |
| Nelson Mandela | 1991-07-18 | null | null | NEC statement: the NEC meeting "presided over by President Comrade Nelson Mandela" |
| Nelson Mandela | 1994-12-22 | null | null | closing address: congratulates the incoming NEC he is "proud to lead" |
| Thabo Mbeki | 1997-12-20 | null | null | saluted "My President" by Mandela; closing statement and stadium address as President of the ANC |
| Thabo Mbeki | 2002-12-20 | null | null | closing statement as President, welcoming the incoming NEC |
| Jacob Zuma | 2007-12-20 | null | null | closing statement "as the 12th President of the ANC" |
| Jacob Zuma | 2012-12-20 | null | null | closing remarks for "the incoming leadership collective" |
| Cyril Ramaphosa | 2017-12-20 | null | null | ANC statement: "the new President of the African National Congress"; closing address as ANC President |
| Cyril Ramaphosa | 2023-01-08 | null | null | January 8 Statement: "Address by ANC President Cyril Ramaphosa" |
| Cyril Ramaphosa | 2026-05-15 | null | null | Secretary-General's statement: the NEC's support for "the President of the African National Congress" |

### How a start and an end are decided

The packet applies the CLAUDE-C01-09 rule unchanged. A holder has `from` only where a source states the day the office
was assumed or took effect, and `until` only where a source states the day it ended. No ANC record reviewed states
either for any President, so every holder is a dated observation. Each holder cites only in-office attestations made on
its own `attested_on` day; the test pins that every cited claim carries exactly the holder's date. Everything else is a
claim that never feeds a holder: National Conference and NEC elections, result lists and their publication, election
references, acceptances, nominations, handover speeches, departure statements, predecessor references ("former",
"outgoing"), the deputy and chair offices, conference spans, and continuation attestations (the incumbent before a
vote, or a later attestation of the same observation).

No end is inferred from a successor's election. Mayibuye's "second day of conference" would make 17 December 1997 the
day of the 1997 officials' election, but that day is derived from two sources and is not stored; it is pinned as a date
that is never a holder boundary. Retrospective index pages, biographies, month-only pages and later web-edition date
headings carry no structured date (their printed words are kept as `printed_range` text in the extract rows); none is a
boundary. The ANC Constitution appears only in the 2022 political report's reference to Rule 16.1.2, as procedure.

### One rule for "President" headings

Check B found that the 2002 opening address was left out because its heading says only "the President", while
Mandela's 1997 political report, with the same kind of heading, was imported (defect B3). One rule now applies to every
such document: a political report or an opening or closing address that the ANC publishes as delivered by its President
to an ANC National Conference is the party President's act, because the ANC's own procedure makes the President present
the NEC's report to Conference. So the 1994, 1997, 2002, 2007, 2012 and 2022 political reports are continuation claims
for the preceding term (never holders), and the 1994 and 2012 closing addresses, whose headings also say only
"President", can date holders. The state office is never read from such a heading, and a document that uses
"President" alongside the Republic's government (the 1994 Declaration) is not imported.

### Party office and the Presidency of the Republic

`za_anc_president` sits on the ANC organization observation; the Presidency of the Republic stays in `za_presidency`.
No `za_presidency` claim or source feeds the ANC role, and no ANC claim or source feeds `za_presidency`. Every ANC
claim and source id begins `za_anc`, and none of the presidency's does. The CLAUDE-C01-09 presidency holders are
unchanged. The existing `test_south_africa_research_s10h.py` guard that the ANC has no roles is re-expressed as an exact
pin: the ANC's roles are exactly `za_anc_president`, its holders are the pinned list, and the role shares no claim or
source with the presidency (the 14 June 2024 election as President-elect is still not evidence for the party office).
Zuma's 2007 closing statement itself recognises "two Presidents", one of state and one of the party, recorded as context
(`za_anc_zuma_two_presidents_state_and_party_20071220`).

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| 8 Jan 1990 | NEC statement: Tambo "The President of your movement, the ANC", ill; the NEC speaks on his instructions | `za_anc_tambo_president_jan8_statement_19900108`; Tambo `attested_on` |
| 1 or 2 Mar 1990 | NEC elects Mandela Deputy President | `za_anc_nec_elects_mandela_deputy_president_19900302` (no structured date; never a holder) |
| 2 Mar 1990 | NEC greets "Comrade President Oliver Tambo" | `za_anc_nec_greets_president_tambo_19900302` (continuation) |
| 2 Jul 1991 | 48th conference opens: Tambo's "my Presidency" farewell; he salutes Deputy President Mandela; Mandela greets "our President" Tambo | `za_anc_tambo_valedictory_presidency_19910702`, `za_anc_tambo_salutes_deputy_president_mandela_19910702`, `za_anc_mandela_greets_president_tambo_19910702` |
| 2-6 or 2-7 Jul 1991 | Conference span; Mandela elected President, Tambo National Chairman; closing address "entrusted me with the presidency", "elected here today"; the NEC "released" Tambo (undated) | `za_anc_48th_conference_dates_19910702_19910706`, `za_anc_48th_conference_elects_mandela_president_199107`, `za_anc_48th_conference_elects_tambo_national_chairman_199107`, `za_anc_mandela_accepts_presidency_199107`, `za_anc_mandela_recounts_tambo_release_199107`, `za_anc1912_heading_mandela_48th_closing_19910706`, `za_anc1912_48th_conference_mandela_elected_president` (no structured date) |
| 13 Jul 1991 | Statement reporting the election (never a holder date) | text of `za_anc_48th_conference_elects_mandela_president_199107` |
| 18 Jul 1991 | NEC meeting of 17-18 July presided over by President Mandela | `za_anc_nec_meeting_president_mandela_19910718`; Mandela 1991 `attested_on` |
| 17 Dec 1994 | Mandela presents the outgoing NEC's political report | `za_anc_mandela_presents_nec_political_report_19941217` (continuation of the 1991 observation) |
| 17-22 Dec 1994 | Conference span; NEC list "as elected ... December 1994" (web edition headed 20 Dec 1994) | `za_anc_49th_conference_dates_19941217_19941222`, `za_anc_49th_conference_elects_mandela_president_199412`, `za_anc1912_49th_nec_list_heading_19941220` (no structured date) |
| 22 Dec 1994 | Closing address: "proud to lead" the incoming NEC | `za_anc_mandela_leads_incoming_nec_19941222`; Mandela 1994 `attested_on` |
| 16 Dec 1997 | Mandela's political report; his generation hands over the baton (prospective) | `za_anc_mandela_political_report_19971216` (continuation), `za_anc_mandela_handover_prospective_19971216` |
| 16-20 Dec 1997 | Conference span; Mbeki elected (office bearers on "the second day of conference", unanimously); Mandela's "departure"; the Declaration salutes the "outgoing President" | `za_anc_conf50_mbeki_elected_president_19971216_19971220`, `za_anc_conf50_report_mbeki_elected_19971216_19971220`, `za_anc_mayibuye_office_bearers_elected_second_day_1997`, `za_anc_conf50_report_mandela_departure_19971216_19971220`, `za_anc_conf50_declaration_salutes_outgoing_president_mandela`, `za_anc_bio_mbeki_became_president_199712` (no structured date) |
| 20 Dec 1997 | Mandela's handover speech; "My President, Comrade Thabo Mbeki"; Mbeki's closing statement and stadium address | `za_anc_mandela_hands_over_baton_19971220`, `za_anc_mandela_salutes_mbeki_my_president_19971220`, `za_anc_mbeki_president_closing_statement_19971220`, `za_anc_mbeki_president_stadium_address_19971220`; Mbeki 1997 `attested_on` |
| 16 Dec 2002 | Mbeki presents the Political Report to the 51st conference | `za_anc_mbeki_opening_political_report_20021216` (continuation of the 1997 observation) |
| 16-20 Dec 2002 | Conference span; NEC list as elected; the programme scheduled the NEC announcement for 20 Dec | `za_anc_conf51_briefing_mbeki_president_as_elected_20021216_20021220`, `za_anc_conf51_programme_nec_announcement_scheduled`, `za_anc_mbeki_recalls_leadership_elected_stellenbosch_2002` (no structured date) |
| 20 Dec 2002 | Mbeki's closing statement as President | `za_anc_mbeki_president_closing_statement_20021220`; Mbeki 2002 `attested_on` |
| before 16 Dec 2007 | Nominations for President: Zuma and Mbeki "Accepted" | `za_anc_conf52_zuma_nomination_accepted_pre_conference`, `za_anc_conf52_mbeki_nomination_accepted_pre_conference` (no structured date) |
| 16 Dec 2007 | Mbeki's political report as President of the ANC | `za_anc_mbeki_political_report_20071216` (continuation of the 2002 observation) |
| 18 Dec 2007 | First round of voting "today"; results expected "this evening" | `za_anc_conf52_first_round_voting_20071218`, `za_anc_conf52_results_announcement_expected_20071218` |
| 19 Dec 2007 | Results for the election of officials published | `za_anc_conf52_officials_results_statement_listed_20071219`; the undated list `za_anc_conf52_officials_results_zuma_president` |
| 20 Dec 2007 | Zuma's closing statement as "the 12th President of the ANC"; he accepts the mandate, "succeeding Cde Thabo Mbeki"; "two Presidents" | `za_anc_zuma_president_closing_statement_20071220` (Zuma 2007 `attested_on`), `za_anc_zuma_accepts_mandate_20071220`, `za_anc_zuma_succeeding_mbeki_20071220`, `za_anc_zuma_two_presidents_state_and_party_20071220` |
| 21 Dec 2007 | ANC Today: "newly-elected ANC President"; Mbeki's letter as "Outgoing President" | `za_anc_today_zuma_newly_elected_president_20071221`, `za_anc_today_mbeki_outgoing_president_20071221`; the undated officials block `za_anc_conf52_newly_elected_nec_zuma_president` |
| 16 Dec 2012 | Zuma's political report | `za_anc_zuma_political_report_as_president_20121216` (continuation of the 2007 observation) |
| 18 Dec 2012 | ANC statement: Zuma "shortly after being elected" | `za_anc_zuma_shortly_after_being_elected_20121218` |
| 19 Dec 2012 | ANC statement: "the re-election of Cde President Jacob Zuma" | `za_anc_zuma_re_election_referenced_20121219` |
| 20 Dec 2012 | Zuma's closing remarks for "the incoming leadership collective" | `za_anc_zuma_closing_remarks_incoming_leadership_20121220`; Zuma 2012 `attested_on` |
| 21-22 Dec 2012 | Conference documents index; site panel "President: Jacob Zuma"; NEC list "December 2012" | `za_anc_53rd_conference_document_index_20121221`, `za_anc_site_officials_panel_zuma_president_20121222`, `za_anc_nec_members_elected_53rd_officials_2012` |
| 6 Jan 2013 | "Address by ANC President Jacob Zuma" | `za_anc_zuma_address_as_anc_president_20130106` (continuation) |
| 16-20 Dec 2017 | Conference span (the web Declaration and the 2018 report); officials "duly elected" | `za_anc_54th_declaration_web_conference_dates`, `za_anc_54th_declaration_conference_dates_20171216_20171220`, `za_anc_54th_report_officials_president_ramaphosa_201712` (no structured date) |
| 18 Dec 2017 | Ramaphosa elected (ANC biography; an election reference) | `za_anc_ramaphosa_elected_president_54th_20171218` |
| 20 Dec 2017 | "The new President of the African National Congress"; closing address as ANC President; Zuma "Former" and "Outgoing" | `za_anc_ramaphosa_new_anc_president_statement_20171220`, `za_anc_ramaphosa_closing_address_as_president_20171220` (Ramaphosa 2017 `attested_on`), `za_anc_zuma_former_anc_president_20171220`, `za_anc_zuma_outgoing_president_20171220` |
| 16 Dec 2022 | Ramaphosa's political report as ANC President | `za_anc_ramaphosa_political_report_as_president_20221216` (continuation of the 2017 observation) |
| 16-20 Dec 2022 and 5 Jan 2023 | Nasrec session; the conference concluded at Imvelo; election "December 2022" | `za_anc_55th_conference_dates_20221216_20221220`, `za_anc_55th_declaration_nasrec_and_imvelo_sessions`, `za_anc_ramaphosa_elected_five_year_term_55th_202212` (no structured date) |
| 8 Jan 2023 | "Address by ANC President Cyril Ramaphosa" | `za_anc_ramaphosa_january8_address_as_president_20230108`; Ramaphosa 2023 `attested_on` |
| 29 Jan 2023 | Officials page: "Our New ANC Officials", President Ramaphosa | `za_anc_officials_new_officials_ramaphosa_president_20230129` (continuation) |
| 15 May 2026 | NEC's "full and continuing support" for its President (briefing on or before 14 May) | `za_anc_nec_reaffirms_ramaphosa_anc_president_20260515`; Ramaphosa 2026 `attested_on` |

Date conventions follow CLAUDE-C01-09: `attested_on` is the day of the observed event as the source dates it; a page's
own issue date is `published_date`; a prospective notice is dated by the day it was made, or not at all when that day
is unknown; a statement that refers to an election without dating it is dated by the statement, never used as the
election's day.

## Observations

### ZA-ANC-01 — The ANC President when the period opens

Evidence: the NEC's anniversary statement dated 8 January 1990 says that "The President of your movement, the ANC, Comrade
Oliver Tambo" cannot address the movement because he has not fully recovered his health, and that the NEC speaks on his
instructions (`za_anc_tambo_president_jan8_statement_19900108`). The NEC's press statement of 2 March 1990 reports that
its meeting of 1 and 2 March elected Nelson Mandela Deputy President (`za_anc_nec_elects_mandela_deputy_president_19900302`)
and greets "Comrade President Oliver Tambo" (`za_anc_nec_greets_president_tambo_19900302`). On 2 July 1991 Tambo's
opening address to the 48th conference thanks those who made "my Presidency" worthwhile
(`za_anc_tambo_valedictory_presidency_19910702`) and salutes "Comrade Nelson Mandela, Deputy President of the ANC"
(`za_anc_tambo_salutes_deputy_president_mandela_19910702`); Mandela's opening address greets "our President, comrade OR
Tambo" (`za_anc_mandela_greets_president_tambo_19910702`).

Decision: accepted in part. Tambo's holder is dated 8 January 1990, the earliest ANC record found inside the period
(nothing dated 1 January 1990 was found), with no start or end. The later attestations are continuation claims. The
deputy election and the deputy salutation sit on `za_anc_president` with the role title "Deputy President of the
African National Congress" and are never holders (check defect A5); no deputy role is added.

Limits: no ANC record reviewed names an acting President during Tambo's illness; the check searched the statements of
14, 15, 19, 21 and 22 August 1989 and 5 and 16 February 1990 without finding one. Tambo's end is not stated: the NEC
"released" him on a day not given (ZA-ANC-02).

### ZA-ANC-02 — July 1991: the 48th National Conference

Evidence: the ANC's statement dated 13 July 1991 says 2,244 delegates met from 2 to 7 July 1991 and that "Oliver Tambo
and Nelson Mandela were unanimously elected National Chairman and President respectively"
(`za_anc_48th_conference_elects_mandela_president_199107`, `za_anc_48th_conference_elects_tambo_national_chairman_199107`).
The ANC's documents page (last modified 22 January 1997) gives 2 to 6 July (`za_anc_48th_conference_dates_19910702_19910706`),
as does the ANC's later conference page, which says "Nelson Mandela was elected President of the ANC"
(`za_anc1912_48th_conference_mandela_elected_president`). Mandela's closing address, dated only "July 1991", says "You
have entrusted me with the presidency of the ANC" and speaks of "This executive we have just elected" and "The
leadership we have elected here today" (`za_anc_mandela_accepts_presidency_199107`); it recounts that Tambo, "as I told
you the other day", told the NEC he was not available and "We then released him"
(`za_anc_mandela_recounts_tambo_release_199107`). The ANC's later web edition heads the same address "6 July 1991"
(`za_anc1912_heading_mandela_48th_closing_19910706`). The NEC's statement of 18 July 1991 says the newly elected NEC met
on 17 and 18 July "presided over by President Comrade Nelson Mandela" (`za_anc_nec_meeting_president_mandela_19910718`).
Mandela's political report of 17 December 1994 is a continuation claim for this observation
(`za_anc_mandela_presents_nec_political_report_19941217`).

Decision: accepted in part. Mandela's 1991 holder is dated 18 July 1991, the first dated in-office attestation after the
conference. The election, acceptance, release and conference-span claims carry no structured date and never feed the
holder; 13 July 1991 (the statement's date) is pinned as a never-holder date (defect A2).

Limits: no ANC record states the day of the election, a declaration of results or a handover. The conference's last day
is 6 July (documents pages, web edition, the Declaration's dateline) or 7 July (the 13 July statement). The earlier
statement to the conference about Tambo's release was not found.

### ZA-ANC-03 — December 1994: the 49th National Conference

Evidence: the ANC's documents page (last modified 7 May 1997) dates the conference 17 to 22 December 1994
(`za_anc_49th_conference_dates_19941217_19941222`). The ANC's page of the NEC "as elected by the 49th National
Conference, Bloemfontein, December 1994" begins "President: Nelson Mandela" (`za_anc_49th_conference_elects_mandela_president_199412`);
the ANC's later web edition of the same list is headed "20 December 1994" (`za_anc1912_49th_nec_list_heading_19941220`).
In his closing address dated 22 December 1994 Mandela says four top officials were returned unopposed, congratulates
the incoming National Executive and says "I am proud to lead such men and women of a high calibre"
(`za_anc_mandela_leads_incoming_nec_19941222`). Mandela's political report of 16 December 1997 is a continuation claim
for this observation (`za_anc_mandela_political_report_19971216`).

Decision: accepted in part. Mandela's 1994 holder is dated 22 December 1994. Only the sentence after the congratulation
of the incoming NEC is relied on; the sentence before it, about leading "a Cabinet", concerns the Republic and is not
used (defect A10).

Limits: the day of the President's election, whether he was among the four unopposed officials and any declaration are
not stated. The web edition's 20 December 1994 conflicts with the ANC's voting-results page dated "Wednesday 21 December
1994" for other members of the same list (a lead).

### ZA-ANC-04 — December 1997: the 50th National Conference

Evidence: the ANC's documents index (last modified 21 January 1998) says that at the conference in Mafikeng, 16 to 20
December 1997, "Thabo Mbeki was elected the new President of the African National Congress"
(`za_anc_conf50_mbeki_elected_president_19971216_19971220`); the Secretary General's introduction to the conference
report speaks of new leaders "with President Thabo Mbeki at the helm" and of Mandela's "departure ... as the President of
the ANC" (`za_anc_conf50_report_mbeki_elected_19971216_19971220`, `za_anc_conf50_report_mandela_departure_19971216_19971220`).
The ANC journal Mayibuye of March 1998 says the office bearers' elections "were held on the second day of conference",
that "Thabo Mbeki was unanimously elected as ANC President", and that the only two contested positions were Deputy
Secretary General and National Chairperson (`za_anc_mayibuye_office_bearers_elected_second_day_1997`; defect B1). The
conference's Declaration salutes "our outgoing President. Comrade Nelson Mandela": "you take leave of the Presidency"
(`za_anc_conf50_declaration_salutes_outgoing_president_mandela`; defect B2). Mandela's political report of 16 December
announces that his generation hands over the baton to successors still to be elected
(`za_anc_mandela_handover_prospective_19971216`); his closing-session speech of 20 December says the time has come to
hand over the baton (`za_anc_mandela_hands_over_baton_19971220`) and opens "My President, Comrade Thabo Mbeki"
(`za_anc_mandela_salutes_mbeki_my_president_19971220`). Mbeki's closing statement and stadium address of 20 December are
headed as by the President of the ANC (`za_anc_mbeki_president_closing_statement_19971220`,
`za_anc_mbeki_president_stadium_address_19971220`). The ANC's 1998 biography says he "became the new President" in
December 1997 (`za_anc_bio_mbeki_became_president_199712`). Mbeki's opening address and Political Report to the 51st
conference on 16 December 2002 is a continuation claim for this observation (`za_anc_mbeki_opening_political_report_20021216`).

Decision: accepted in part. Mbeki's 1997 holder is dated 20 December 1997 on three in-office attestations of that day.
The election claims carry no structured date: "the second day of conference" would be 17 December 1997 only by
combining Mayibuye with the report's opening day, so that day is not stored and is pinned as a never-holder date.
Mandela's 1994 observation has no end: the departure, farewell and handover-speech claims state no day on which his
party office ended.

Limits: no ANC press statement survives for 17 or 18 December 1997, and the results page of 19 December lists only the
60 additional NEC members; no vote count, declaration or handover day was found.

### ZA-ANC-05 — December 2002: the 51st National Conference

Evidence: the ANC's pre-conference programme scheduled the "Announcement of National Executive Committee" and a
"Closing Address by Incoming President" for Friday 20 December 2002 (`za_anc_conf51_programme_nec_announcement_scheduled`).
The ANC's Conference Briefing of February 2003 says the conference met from 16 to 20 December 2002 and lists "President:
Mbeki, Thabo" in the NEC as elected (`za_anc_conf51_briefing_mbeki_president_as_elected_20021216_20021220`). Mbeki's
closing statement dated 20 December 2002, headed as by "the President of the African National Conference" (sic),
welcomes the incoming NEC (`za_anc_mbeki_president_closing_statement_20021220`). His 2007 political report refers to
"the national leadership we elected in Stellenbosch in 2002" (`za_anc_mbeki_recalls_leadership_elected_stellenbosch_2002`)
and is itself a continuation claim for this observation (`za_anc_mbeki_political_report_20071216`).

Decision: accepted in part. Mbeki's 2002 holder is dated 20 December 2002. The programme is prospective and undated,
and the briefing gives only the conference span; neither dates a holder.

Limits: the archived ANC press releases for December 2002 stop at 5 December and no ANC Today issue covering the
conference was captured, so no day of the vote, declaration or vote record was found.

### ZA-ANC-06 — December 2007: the 52nd National Conference

Evidence: the ANC's pre-conference list of nominations for officials shows Zuma and Mbeki "Accepted" for President
(`za_anc_conf52_zuma_nomination_accepted_pre_conference`, `za_anc_conf52_mbeki_nomination_accepted_pre_conference`;
undated, embedded title "PROVISIONAL PRE-CONFERENCE LIST OF CANDIDATES ..."; defect B9). Mbeki's political report as
President of the ANC is dated 16 December 2007. The ANC Electoral Commission's statement of 18 December congratulates
delegates on "the first round of voting today" and expects results "some time this evening"
(`za_anc_conf52_first_round_voting_20071218`, `za_anc_conf52_results_announcement_expected_20071218`). The ANC's press
index lists "19 December - Results for the Election of ANC Officials" (`za_anc_conf52_officials_results_statement_listed_20071219`),
and the undated results page gives Zuma 2,329 and Mbeki 1,505 (`za_anc_conf52_officials_results_zuma_president`;
defect B10). Zuma's closing statement dated 20 December 2007 says he stands before the delegates "as the 12th President
of the ANC" (`za_anc_zuma_president_closing_statement_20071220`), that "the incoming NEC and I accept the mandate"
(`za_anc_zuma_accepts_mandate_20071220`), that he leads "succeeding Cde Thabo Mbeki", whom he calls "former President of
the ANC" (`za_anc_zuma_succeeding_mbeki_20071220`), and that there are "two Presidents", of state and of the party
(`za_anc_zuma_two_presidents_state_and_party_20071220`). The newly-elected NEC page lists "President: Zuma, Jacob"
(`za_anc_conf52_newly_elected_nec_zuma_president`). ANC Today of 21 December calls him "newly-elected ANC President"
and carries Mbeki's "Letter from the Outgoing President" (`za_anc_today_zuma_newly_elected_president_20071221`,
`za_anc_today_mbeki_outgoing_president_20071221`). Zuma's political report of 16 December 2012 is a continuation claim
for this observation (`za_anc_zuma_political_report_as_president_20121216`).

Decision: accepted in part. Zuma's 2007 holder is dated 20 December 2007. The vote, the expected declaration, the
publication and the acceptance stay separate claims; Mbeki's 2002 observation has no end, because "former" and
"outgoing" state no day.

Limits: the day and time of the declaration in the hall are not recorded (expected on the evening of 18 December,
published under 19 December). ANC Today signs its extract of Zuma's closing address 21 December 2007, against the 20
December dateline of the full text.

### ZA-ANC-07 — December 2012: the 53rd National Conference

Evidence: an ANC statement dated 18 December 2012 says "President Jacob Zuma, shortly after being elected, made a
passionate plea to Delegates" (`za_anc_zuma_shortly_after_being_elected_20121218`), and one dated 19 December refers to
"the re-election of Cde President Jacob Zuma" (`za_anc_zuma_re_election_referenced_20121219`; both located by the check,
defect C3). Zuma's closing remarks dated 20 December 2012 congratulate "the newly elected leadership of the ANC" and
speak "as the incoming leadership collective" (`za_anc_zuma_closing_remarks_incoming_leadership_20121220`). The ANC
site's conference page, last updated 21 December 2012, indexes the conference documents
(`za_anc_53rd_conference_document_index_20121221`), and its officials panel captured on 22 December 2012 reads
"President: Jacob Zuma" (`za_anc_site_officials_panel_zuma_president_20121222`). The NEC page lists the officials
"elected at the 53rd National Conference in Mangaung December 2012", Zuma first
(`za_anc_nec_members_elected_53rd_officials_2012`). His address of 6 January 2013 is headed "Address by ANC President
Jacob Zuma" (`za_anc_zuma_address_as_anc_president_20130106`; defect C4).

Decision: accepted in part. Zuma's 2012 holder is dated 20 December 2012. The remarks say only "President", so the
holder's uncertainty states the inference (the rule for addresses to Conference, the "incoming leadership collective",
and the corroborating claims of 18 and 19 December 2012 and 6 January 2013). The two statements date references to the
election, not the vote, and are never holder dates.

Limits: the day of the vote, any declaration and the totals were not found; old-site pages 9990, 9991, 9996 and 9998
were never captured.

### ZA-ANC-08 — December 2017: the 54th National Conference

Evidence: the ANC's biography of Ramaphosa, served on 22 December 2017, says he "has been elected President of the
African National Congress at its 54th National Conference in Nasrec, Soweto, on 18 December 2017"
(`za_anc_ramaphosa_elected_president_54th_20171218`; an election reference, defect C18). An ANC statement dated
20 December 2017 reports Zuma congratulating "the new President of the African National Congress, Comrade President
Cyril Ramaphosa" and calls Zuma "Former ANC President" (`za_anc_ramaphosa_new_anc_president_statement_20171220`,
`za_anc_zuma_former_anc_president_20171220`). Ramaphosa's closing address of that day is titled as by "ANC President
Cyril Ramaphosa" and greets the "Outgoing President of the African National Congress, Cde Jacob Zuma"
(`za_anc_ramaphosa_closing_address_as_president_20171220`, `za_anc_zuma_outgoing_president_20171220`). The Declaration's
web page dated 20 December 2017 and the conference report compiled in 2018 give the conference as "16th - 20th December
2017" (`za_anc_54th_declaration_web_conference_dates`, `za_anc_54th_declaration_conference_dates_20171216_20171220`),
and the report lists him first among the officials "duly elected" (`za_anc_54th_report_officials_president_ramaphosa_201712`).
His political report of 16 December 2022 is a continuation claim for this observation
(`za_anc_ramaphosa_political_report_as_president_20221216`).

Decision: accepted in part (defect C7). Ramaphosa's 2017 holder is dated 20 December 2017 on two in-office attestations
of that day. 18 December 2017 is an election reference and never a start; Zuma's 2012 observation has no end.

Limits: no ANC Electoral Commission declaration, totals or handover record was found.

### ZA-ANC-09 — December 2022: the 55th National Conference

Evidence: the ANC's conference page captured on 23 December 2022 gives "16 - 20 December 2022", Nasrec
(`za_anc_55th_conference_dates_20221216_20221220`); the conference's Declaration, dated 5 January 2023, Imvelo,
Bloemfontein, says the delegates "gathered at Nasrec in Gauteng on 16-20th December 2022 and concluded on 5 January 2023
at Imvelo Lodge" (`za_anc_55th_declaration_nasrec_and_imvelo_sessions`; defect C9). The NEC's January 8 Statement of
2023 is headed "Address by ANC President Cyril Ramaphosa" (`za_anc_ramaphosa_january8_address_as_president_20230108`),
and the officials page captured on 29 January 2023 lists him as President of "Our New ANC Officials"
(`za_anc_officials_new_officials_ramaphosa_president_20230129`). The ANC's statement of 15 May 2026 says the branches
"elected him to a five-year term at the 55th National Conference at Nasrec in December 2022"
(`za_anc_ramaphosa_elected_five_year_term_55th_202212`).

Decision: accepted in part. Ramaphosa's 2023 holder is dated 8 January 2023, the first dated ANC primary found after the
conference concluded. The Nasrec page is relabelled as the scheduled Nasrec session only.

Limits: the ANC dates the election only as December 2022, retrospectively; the day of the vote, the declaration, totals
and any handover were not found.

### ZA-ANC-10 — An ANC attestation of its President before the cutoff

Evidence: the Secretary-General's statement dated 15 May 2026 records that the NEC neither considered nor was asked to
consider recalling the ANC President and reaffirmed "full and continuing support" for "the President of the African
National Congress, Comrade Cyril Ramaphosa" (`za_anc_nec_reaffirms_ramaphosa_anc_president_20260515`).

Decision: accepted. A separate holder observation dated 15 May 2026 (one per reviewed observation; defect C6), with no
start or end. The page was captured at 09:15 UTC that day yet reports a meeting that "has just risen" "this evening", so
the briefing was on or before the evening of 14 May 2026 (defect C14); the date line is the publication date.

Limits: the officials page captured on 31 August 2026 was dropped: its own metadata shows its content was last modified
on 1 February 2023 (defect C2). No acting arrangement, vacancy or change was found in ANC statements between May and
7 September 2026; three later statements were read live only and are leads.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `za_anc_jan8_statement_19900108` | January 8 Statement - 1990: Statement of the National Executive Committee on the occasion of the 78th anniversary of the African National Congress (8 January 1990) | 44,048 bytes, `38277ded…a69fff`; capture 2001-05-05 |
| `za_anc_nec_statement_19900302` | Press statement of the National Executive Committee of the African National Congress, Lusaka, Zambia (2 March 1990) | 5,363 bytes, `754b4849…9ad7f2`; capture 2001-05-05 |
| `za_anc_tambo_48th_opening_19910702` | Oliver Tambo's opening address to the ANC 48th National Conference, Durban (2 July 1991) | 23,902 bytes, `845207bf…d1fcc8`; capture 1997-07-09 |
| `za_anc_mandela_48th_opening_19910702` | Nelson Mandela's opening address to the 48th National Conference of the African National Congress, Durban (2 July 1991), 'Presidential Report II' | 38,670 bytes, `47f53e01…354b80`; capture 1997-07-08 |
| `za_anc_48th_conference_index_1997` | 48th National Conference Documents, Durban, 2-6 July 1991 (ANC website index page) | 3,569 bytes, `dbd246de…660c01`; capture 1997-07-09 |
| `za_anc_48th_conference_statement_19910713` | ANC National Conference: statement of the ANC Department of Information and Publicity (13 July 1991) | 7,606 bytes, `170a20ef…88c9e4`; capture 2000-10-02 |
| `za_anc_mandela_48th_closing_199107` | Nelson Mandela's closing address to National Conference (48th National Conference, July 1991) | 24,345 bytes, `e9d364f2…02ec96`; capture 1997-07-08 |
| `za_anc_nec_statement_19910718` | Meeting of the National Executive Committee: ANC statement (18 July 1991) | 5,344 bytes, `2ccf364c…3a3bd0`; capture 2000-10-02 |
| `za_anc_49th_conference_index_1997` | 49th National Conference Documents (ANC website index page) | 3,473 bytes, `c9929f59…0ade36`; capture 1997-07-09 |
| `za_anc_mandela_49th_opening_19941217` | Opening address by President Nelson Mandela: Political Report of the National Executive Committee to the 49th National Conference of the African National Congress (17 December 1994) | 60,145 bytes, `691a00d7…568269`; capture 1997-10-17 |
| `za_anc_49th_nec_as_elected_199412` | National Executive Committee as elected by the 49th National Conference, Bloemfontein, December 1994 (ANC website page) | 4,004 bytes, `378bcd62…19132a`; capture 2000-10-02 |
| `za_anc_mandela_49th_closing_19941222` | Closing address by President Nelson Mandela, 49th National Conference of the African National Congress, Bloemfontein (22 December 1994) | 21,566 bytes, `967eb375…9935da`; capture 1997-10-17 |
| `za_anc1912_mandela_48th_closing_address_page` | 48th National Conference: President Nelson Mandela's closing address (ANC website, anc1912.org.za) | 206,816 bytes, `da6f822a…de06a0`; capture 2021-10-28; located by the check |
| `za_anc1912_48th_conference_page` | 48th National Conference 1991 (ANC website conference page, anc1912.org.za) | 156,498 bytes, `a958ee3c…c4a502`; capture 2021-09-19; located by the check |
| `za_anc1912_49th_nec_as_elected_page` | 49th National Conference: National Executive Committee as elected at Conference (ANC website, anc1912.org.za) | 186,330 bytes, `c6507c27…e740ac`; capture 2021-10-19; located by the check |
| `za_anc_conf50_documents_index_19980121` | 50th National Conference Documents (ANC website index page; 'Last modified: 21 January 1998') | 7,826 bytes, `c0015916…0cd0b3`; capture 1998-02-12 |
| `za_anc_conf50_report_introduction` | ANC 50th National Conference Report: contents and introduction by Secretary General Kgalema Motlanthe | 4,138 bytes, `59c8088a…85e535`; capture 1999-02-24 |
| `za_anc_mandela_political_report_19971216` | Political Report of the President, Nelson Mandela, to the 50th National Conference of the African National Congress (16 December 1997) | 155,970 bytes, `1f47d2d6…3d7725`; capture 2002-05-04 |
| `za_anc_mandela_closing_address_19971220` | Address by Nelson Mandela to the Closing Session of the 50th National Conference of the ANC (Mafikeng, 20 December 1997) | 11,036 bytes, `a55c4a73…fd7d57`; capture 1998-02-12 |
| `za_anc_mbeki_closing_statement_19971220` | Statement of the President of the African National Congress, Thabo Mbeki, at the Closing of the 50th National Conference of the ANC (Mafikeng, December 20, 1997) | 12,372 bytes, `1c35392d…251f9f`; capture 1998-02-12 |
| `za_anc_mbeki_stadium_address_19971220` | Address by the President of the ANC, Thabo Mbeki, at Mafikeng Stadium (20 December 1997) | 10,645 bytes, `904bc318…cd0ac8`; capture 1998-02-12 |
| `za_anc_biography_mbeki_1998` | Biography of Thabo Mbeki (ANC website people page, captured 12 February 1998) | 4,499 bytes, `d848299f…39d8c8`; capture 1998-02-12 |
| `za_anc_conf51_programme` | 51st National Conference Programme, Stellenbosch University (15-20 December 2002) | 8,771 bytes, `e49d09ca…1dd562`; capture 2002-12-14 |
| `za_anc_conf51_briefing_200302` | Conference Briefing: briefing notes on the ANC 51st National Conference, December 2002 (February 2003) | 122,638 bytes, `a8679d79…15db2b`; capture 2003-08-02; PDF pages 1, 16 viewed |
| `za_anc_mbeki_closing_statement_20021220` | Statement of the President of the African National Congress, Thabo Mbeki, at the Closing of the 51st National Conference of the ANC (Stellenbosch, 20 December 2002) | 17,777 bytes, `4aaac5ef…493d36`; capture 2003-01-16 |
| `za_anc_conf52_officials_nominations` | African National Congress 52nd National Conference: Nominations for Officials | 26,094 bytes, `28bade60…9fd771`; capture 2008-12-02; PDF page 1 viewed |
| `za_anc_mbeki_political_report_20071216` | Opening address and political report of the President of the ANC, Thabo Mbeki, to the 52nd National Conference (University of Limpopo, 16 December 2007) | 122,472 bytes, `105cf529…81fe31`; capture 2007-12-20 |
| `za_anc_electoral_commission_statement_20071218` | Statement by the Chairperson of the [ANC] Electoral Commission (18 December 2007) | 2,567 bytes, `7e3ba46f…8b8111`; capture 2009-01-07 |
| `za_anc_conf52_press_statements_index` | ANC 52nd National Conference: Press Statements (index page) | 1,468 bytes, `6e558b32…27c8b8`; capture 2008-04-25 |
| `za_anc_officials_election_results_2007` | Results for the Election of ANC Officials (52nd National Conference, 2007) | 1,051 bytes, `59f8d09a…85520a`; capture 2007-12-22 |
| `za_anc_zuma_closing_statement_20071220` | Statement by the President of the African National Congress, Cde Jacob Zuma, to the closing of the 52nd National Congress of the ANC (sic, for Conference; Polokwane, 20 December 2007) | 14,941 bytes, `aadbc727…64b972`; capture 2007-12-24 |
| `za_anc_newly_elected_nec_2007` | ANC 52nd National Conference: Newly-elected National Executive Committee | 3,578 bytes, `62ae26bc…99a415`; capture 2007-12-26 |
| `za_anc_today_v7n50_20071221` | ANC Today, Volume 7, No. 50, 21-27 December 2007 | 39,662 bytes, `22e7c961…7dd1bd`; capture 2007-12-24 |
| `za_anc_mayibuye_199803` | Mayibuye, March 1998: 'Power to the people - We dare not fail' (ANC journal, archived www.anc.org.za) | 87,296 bytes, `310f4135…caaf51`; capture 2004-04-17; located by the check |
| `za_anc_conf50_declaration` | Declaration of the ANC 50th National Conference (Conference Report page; page title 'Draft Declaration of the ANC 50th National Conference') | 7,007 bytes, `31492be0…7383cb`; capture 1999-02-24; located by the check |
| `za_anc_mbeki_opening_address_51st_20021216` | Address of the President, Thabo Mbeki, at the opening of the 51st National Conference of the African National Congress (Stellenbosch, 16 December 2002) | 104,348 bytes, `cd93efda…7772b3`; capture 2003-01-15; located by the check |
| `za_anc_political_report_zuma_53rd_20121216` | Political Report by President Jacob Zuma to the 53rd National Conference of the ANC | 66,057 bytes, `43390c9f…babd01`; capture 2013-08-11 |
| `za_anc_events_53rd_conference_20121222` | 53rd National Conference - Mangaung (ANC website event page) | 31,288 bytes, `166898ce…728189`; capture 2012-12-22 |
| `za_anc_closing_remarks_zuma_53rd_20121220` | Closing Remarks to the 53rd National Conference of the ANC by President Jacob Zuma | 40,166 bytes, `5cf87657…e67af0`; capture 2013-08-11 |
| `za_anc_nec_members_53rd_2012` | ANC National Executive Committee (as elected at the 53rd National Conference) | 30,832 bytes, `3b0dbc77…f31825`; capture 2013-08-11 |
| `za_anc_statement_vavi_response_20121218` | ANC response to Cde Vavi's comments (ANC press statement, 18 December 2012) | 29,125 bytes, `d8aa0f94…b4b589`; capture 2013-08-18; located by the check |
| `za_anc_statement_zille_remarks_20121219` | Zille's remarks are political jealousy (ANC press statement, 19 December 2012) | 29,344 bytes, `cad89d48…a02507`; capture 2013-08-18; located by the check |
| `za_anc_zuma_centenary_concert_address_20130106` | Address by ANC President Jacob Zuma to the concert marking the end of the ANC Centenary Celebrations (6 January 2013) | 38,812 bytes, `970bf103…253748`; capture 2013-08-18; located by the check |
| `za_anc_officials_ramaphosa_profile_20171222` | Welcome to the personal page of Cyril Ramaphosa (ANC Officials: President) | 71,494 bytes, `beb7e76a…e75607`; capture 2017-12-22 |
| `za_anc_statement_zuma_congratulates_ramaphosa_20171220` | President Zuma congratulates President Ramaphosa as new ANC President | 47,085 bytes, `ec7a959a…f144ec`; capture 2017-12-24 |
| `za_anc_closing_address_ramaphosa_54th_20171220` | Closing Address by ANC President Cyril Ramaphosa to the 54th National Conference of the African National Congress | 34,165 bytes, `22378630…b161a9`; capture 2017-12-25 |
| `za_anc_54th_conference_report` | 54th National Conference: Report and Resolutions (Report of the 54th National Conference) | 750,729 bytes, `80feb094…53b969`; capture 2021-11-09; PDF pages 1, 11, 13, 82 viewed |
| `za_anc_54th_declaration_20171220` | Declaration of the 54th National Conference of the African National Congress (ANC website, 20 December 2017) | 25,499 bytes, `2c8f2bb2…b11f95`; capture 2017-12-26; located by the check |
| `za_anc_political_report_ramaphosa_55th_20221216` | Political Report presented by ANC President Cyril Ramaphosa. 16 December 2022 | 55,991 bytes, `c6432fa2…3b94f6`, gzip as served (decoded 274,319 bytes, `3c1b2958…f6a1d8`); capture 2023-01-03 |
| `za_anc_55th_conference_page_20221223` | ANC 55th National Conference 2022 (ANC website conference page) | 29,197 bytes, `2a5def8f…07d3ca`, gzip as served (decoded 192,283 bytes, `82523409…0ed407`); capture 2022-12-23 |
| `za_anc_january_8_statement_2023` | Statement of the National Executive Committee on the occasion of the 111th Anniversary of the ANC: Address by ANC President Cyril Ramaphosa, 8 January 2023 | 257,284 bytes, `4b8cc59d…8abe99`; capture 2023-01-26 |
| `za_anc_officials_page_20230129` | Officials - ANC ("Our New ANC Officials") | 215,841 bytes, `e909e458…c30df1`; capture 2023-01-29 |
| `za_anc1912_55th_declaration_20230105` | 55th National Conference Declaration (Imvelo, Bloemfontein, 5 January 2023) | 208,104 bytes, `7bad5a2c…6ca2fc`; capture 2023-02-06; located by the check |
| `za_anc_sg_statement_special_nec_20260515` | Statement by the Secretary-General of the African National Congress, Cde Fikile Mbalula, on behalf of the National Officials, on the outcomes of the special meeting of the National Executive Committee following the Constitutional Court judgment on the Section 89 matter | 329,216 bytes, `ca973967…d304ec`; capture 2026-05-15 |

All 54 sources are the ANC's own records, served as raw Internet Archive captures (`id_` form) made between 1997 and
15 May 2026, before the cutoff: 42 HTML pages of the old www.anc.org.za site (1997-2017 captures), nine HTML pages of
the current anc1912.org.za site (2021-2026 captures), and three PDFs (the 2002 Conference Briefing and the 2007
nominations list from the old site, and the 54th conference report from the current one). Each source records the capture URL as `url` and the original address as `original_url`
(without `:80`), and its extract records the capture time. Eleven were located by the checks (marked above). No live
page is used, because the current site regenerates its HTML per request (Cloudflare challenge tokens, e-mail
obfuscation keys and a reordered font block give a new SHA-256 each time) and the old site no longer serves its pages.

Each new source has a derived factual extract under [sources/](sources/) in the packet's existing format: one row per
claim, keyed by `claim_id`, with `observation_id` `za_iec_n2024_014`, `review_observation`, `role_id`
`za_anc_president`, `holder_name` (normalised: "Oliver Tambo" for "OR Tambo", "Jacob Zuma" for "Zuma, Jacob", "Cyril
Ramaphosa" for "Cyril Matamela Ramaphosa"), `role_title`, `event_kind` and `attested_on`, plus the claim's text and
locator and, for undated rows, the printed words as `printed_range`. The extract's checksum is in the packet, separate
from the response hash. Original pages, PDFs and renders are not checked in, and no photograph, logo or signature is
republished.

Source types: `primary_party_statement_archived`, `primary_party_speech_archived`, `primary_party_web_page_archived`,
`primary_party_record_archived`, `primary_party_publication_archived` and `party_biography_archived` (the two
retrospective biographies of 1998 and 2017).

## Response identities and stability checks

Codex re-downloads every recorded response and compares its byte count and SHA-256, so each extract carries the recipe
the identities depend on (defect C1): fetch the recorded `url` exactly, with **no `Accept-Encoding` request header and
no automatic decoding** (for example `curl -s -o FILE URL`, not `--compressed`), and hash the bytes as received. The
check found that a client sending `Accept-Encoding: gzip` receives an on-the-fly gzip body (the 2012 political report
came back as 24,336 bytes instead of 66,057), and that a decoding client unpacks the captures the archive stores
compressed. Two captures are stored and served gzip-encoded, and their extracts record the decoded identity beside the
served one: `za_anc_political_report_ramaphosa_55th_20221216` (274,319 bytes decoded) and
`za_anc_55th_conference_page_20221223` (192,283 bytes decoded). The one zstd-encoded capture in the dossiers was dropped
for content reasons (C2). Every extract also records the base32 SHA-1 of the bytes; for the part C sources the check
confirmed that it equals the Internet Archive's CDX digest of the capture, so the bytes are the stored WARC payload.

Stability: every response was downloaded at least twice, at least 30 minutes apart, with the same byte count and
SHA-256. Part A: the researcher downloaded each twice (34-55 minutes apart) and the check twice more (22:44-22:55Z and
23:15-23:24Z on 24 September 2026). Part B: the researcher twice (31-80 minutes apart) and the check twice more
(16:25-16:37 and 17:31-17:34 PDT). Part C: the researcher twice and the check once more with plain curl, at least 49
minutes after the kept copy. The eleven records located by the checks were each downloaded twice by the check, at least
30 minutes apart; the three part B records (Mayibuye, the 1997 Declaration and the 2002 opening address) were downloaded
a third time for this packet at 2026-09-25T00:42Z (17:42 PDT on 24 September), and the eight part A and C records were
re-hashed from the check's kept copies. Every kept copy of the 43 dossier sources was re-hashed for this packet with the
same result.

Per-request traps: no response is generated per request. The live 54th report PDF is a static upload; the check and the
researcher found the live file, a cache-busted fetch and every Internet Archive capture byte-identical. The PDFs'
embedded creation dates are stored in the archived files, not regenerated, and no page-range or generated PDF is used.
The test pins every URL to the raw `id_` form on web.archive.org and rejects search, API, e-mail-token and cache-busting
URL shapes.

## Leads not imported

- The ANC's live anc1912.org.za pages (for example https://www.anc1912.org.za/49th-national-conference-nelson-mandelas-closing-address/,
  the 1997, 2002 and 2007 conference pages, the 2012 officials pages): regenerated per request (two downloads of the
  1991 closing address page were 327,623 bytes with different SHA-256 values; a cache-busted copy was 327,698 bytes).
  Where a pre-cutoff capture exists and adds something, the capture is imported instead (defect A7).
- ANC 49th National Conference page, capture https://web.archive.org/web/20210919042555id_/https://www.anc1912.org.za/49th-national-conference-1994/
  (157,185 bytes, `60a48c45…e0ecd7`): "17-22 December 1994, Bloemfontein"; repeats the 1997 index, so not imported.
- 1991 Independent Electoral Commission report (`iecrep48.html`, 6,733 bytes, `3cee3be6…15b5f4`): ballots for Deputy
  President, Secretary General, Deputy Secretary General and the NEC; names no President. Procedure only.
- 1991 Declaration (`declare48.html`): datelined "Durban, 6 July 1991"; cited only for the end-date conflict.
- 1994 NEC voting results (`necvot49.html`, "Bloemfontein, Wednesday 21 December 1994"): the 60 additional members only.
- 1994 Declaration (`declare49.html`): mentions "President Nelson Mandela" alongside the Government of National Unity,
  so it is kept out to keep the two offices apart.
- ANC statements of 5 February 1990 (`pr0205.html`, 1,425 bytes, `8e1a84be…b973ed`) and 16 February 1990 (`pr0216.html`,
  9,337 bytes, `6003c835…c4fe76`); Tambo's address of 14 December 1990 (`or90-4.html`); the statement of 19 July 1991 by
  "ANC President Nelson Mandela" (`pr0719.html`, 2,513 bytes, `4e44ff25…bb5f82`); Tambo's Fort Hare address of
  19 October 1991 (`or91-4.html`); the 1991 press-statement index. They repeat recorded attestations or fall outside
  the observations.
- ANC 1997 NEC results (`conference50/necresults.html`, issued 19 December 1997): the 60 additional members only.
- The 1997 conference index captured with "Last modified: 08 June 1998"
  (https://web.archive.org/web/19981205082127id_/http://www.anc.org.za:80/ancdocs/history/conf/conference50/index.html,
  8,279 bytes, `3a9b89ec…4e6eb9`): needed only for a phrase that is now dropped (defect B6).
- The 2002 NEC page (`conference51/nec51.html`): the same list as the Conference Briefing, undated.
- The 2007 Declaration (`declaration1220-07.html`): does not mention the President's election. The Electoral
  Commission statement of 12 December 2007 (`pr1212.html`): procedure only. The ANC biography of Zuma captured
  21 January 2008 (`people/zumaj.html`): its body predates the 2007 election.
- ANC homepage capture https://web.archive.org/web/20071217172822id_/http://www.anc.org.za:80/ (20,510 bytes,
  `91d7fb28…050c2a`): says the nominations list "has been released", with no day; the list itself is recorded as undated
  (defect B9).
- 2007 conference programme https://web.archive.org/web/20081202224216id_/http://www.anc.org.za/ancdocs/history/conf/conference52/programme.pdf
  (89,317 bytes, `afb7f901…a34f7d`): a schedule only, and its text layer drops digits.
- ANC Today of 14 December 2007 (`at49.htm`, 25,272 bytes, `b6751f0f…394f46`) and 11 January 2008 (`at01.htm`, 18,738
  bytes, `c1cf0bee…606647`): their "Letter from the President" signatures are images titled "Thabo Mbeki" and "Jacob G
  Zuma"; an image title is too weak for a claim.
- The 2012 conference programme (live file `Programme-2012.pdf`, 487,968 bytes, `e23c87a9…15dc6b`): a prospective
  schedule "subject to change". ANC statements of 13 and 19 December 2012 (`show.php?id=9988`, `id=9994`, `id=9995`):
  procedure and additional-member ballots. ANC Today of January 2013 (`docs/anctoday/2013/at01.htm`): adds nothing.
  The same 6 January 2013 address as `show.php?id=10006` (38,216 bytes, `52b6a13c…1539d7`): a duplicate.
- The 2 January 2018 capture of `officials/current`: internally inconsistent ("elected deputy president at the 54th
  National Conference"). The 2023 profile `anc-president-cyril-ramaphosa-3`: reuses the 2017 text. The 2017 biography
  of Zuma (`officials/jacob-zuma`): no 2012 re-election or end.
- Opening remarks to the second part of the 55th conference (capture 20230105214749, 203,880 bytes, `4fbc8a0f…9c2a8e`):
  its title says 5 January 2023 but its body date line reads "29 July 2022, Bloemfontein".
- The officials page captured 31 August 2026 (https://web.archive.org/web/20260831192551id_/http://www.anc1912.org.za/officials/,
  46,643 bytes zstd-encoded, `f4fe0417…a3995f`; 317,585 bytes decoded, `33211892…58d676`): content last modified
  1 February 2023, so not a 2026 attestation; removed (defect C2).
- The NEC page captured 25 August 2026 (https://web.archive.org/web/20260825152506id_/https://www.anc1912.org.za/nec/,
  534,807 bytes, `b5bc8fda…a80b47`): an attestation as served, but its content was last modified on 21 May 2025 and the
  dated 15 May 2026 statement is later in substance.
- ANC Secretary-General annual report 2023 (live PDF, 3,497,634 bytes): no election day. ANC media advisory of
  5 July 2026, statement of 30 June 2026 and statement of 13 August 2026: read live only (no pre-cutoff capture); no
  change to the party office. The site's `wp-json` pages listing: discovery only.
- History sites, encyclopaedias and news: https://sahistory.org.za/people/thabo-mvuyelwa-mbeki (Mbeki elected at
  Mafikeng in 1997, no day) and the commonly reported vote days (18 December 2012, 18 December 2017, 19 December 2022)
  are leads to be matched to ANC or Electoral Commission records; none is imported.

## Sources attempted

- Internet Archive: connection refusals (curl exit 7), timeouts and HTTP 429 for all three researchers and the checks on
  a shared address; every capture used was eventually retrieved. Part A left three unused 1990 statements (`pr0319`,
  `pr0328`, `pr0410`) unretried.
- The live anc1912.org.za HTML: HTTP 200 but not reproducible (see above). The live 2022 statements index returned
  404; the WordPress posts search returns nothing because the site publishes pages.
- Redirecting captures: 20221218223817 (political report, to 20230103184831), 20230108155702 (January 8 Statement, to
  20230126132648) and 20230106072304 (55th Declaration, to 20230206054451, followed by the check). Capture 20230107092032
  of `national-executive-committee-as-elected/` is the 52nd conference NEC.
- Old-site pages `show.php?id=9990`, `9991`, `9996` and `9998` have no capture; `9992` and `9993` are captured and are
  imported (defect C3 corrects part C's note).
- CDX searches that found nothing further: ANC press statements for 17-18 December 1997; press releases after
  5 December 2002; ANC Today covering the 2002 conference; December 1997 ANC Daily News Briefing; conference51 and
  conference52 folders, pr/2007 and ANC Today 2007-2008 for the 2007 declaration time; Umrabulo 17 and 18; Mayibuye
  issues of 2003; ANC Today 2012 after `at46`; captures of the ANC sites for an Electoral Commission declaration or
  totals in 2012, 2017 or 2022.
- Acting President: the check read the ANC statements of 14, 15, 19, 21 and 22 August 1989 and 5 and 16 February 1990;
  none names one. The day of the 1991 or 1994 election: July 1991 statements `pr0704`, `pr0704a`, `pr0710`, `pr0710a`,
  the speeches folders and the 1994 press list (nothing after 8 December 1994); none dates it.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | Mandela holders cited the election report, the acceptance and the NEC list | **Applied**: each holder cites only in-office attestations of its own day (18 July 1991; 22 December 1994); elections, acceptances and lists are never-holder claims pinned in the test |
| A2 | Alternative `attested_on` of 13 July 1991 | **Applied**: dropped; 13 July 1991 is pinned as a never-holder date |
| A3 | Whole-month ranges on month-only claims | **Applied**: no structured date; "July 1991" and "December 1994" kept as `printed_range` text; no `attested_period` field anywhere in the packet |
| A4 | Retrospective 1997 index pages carried structured dates | **Applied**: event kind `retrospective_conference_dates`, no structured date |
| A5 | Three claims with `role_id` null | **Applied**: on `za_anc_president` with role titles "Deputy President of the African National Congress" and "National Chairman of the African National Congress"; never holders; no deputy or chair role added |
| A6 | "OR Tambo" as a holder name | **Applied**: "Oliver Tambo"; the printed form stays in the text and is noted in the uncertainty |
| A7 | anc1912.org.za leads dismissed as unreproducible | **Applied in part**: the 2021 captures of the closing address (6 July 1991 heading), the 48th conference page and the 1994 NEC list (20 December 1994 heading) are imported as claims with no structured date; the 49th conference page capture repeats the 1997 index and is a lead |
| A8 | Acceptance claim omitted key lines | **Applied**: "This executive we have just elected", "The leadership we have elected here today" and "During the past five days we have witnessed a process" are quoted; still undated |
| A9 | Release claim omitted "as I told you the other day" | **Applied**: quoted; the earlier statement is listed as unresolved |
| A10 | 22 December 1994 claim did not say which sentence it relies on | **Applied**: the uncertainty says the preceding Cabinet sentence is not relied on |
| A11 | Proposed holders lacked packet fields | **Applied**: packet holder shape (name, `attested_on`, `from`, `until`, `sources`, `claim_ids`, `note`, `uncertainty`) |
| B1 | *Mayibuye*, March 1998, missed: office bearers elected on the second day | **Applied**: imported (`za_anc_mayibuye_199803`) with `printed_range` "the second day of conference" and no structured date; 17 December 1997 is stated as derived, not stored, and pinned as a never-holder date; summary and unresolved notes updated |
| B2 | The 50th Conference Declaration missed | **Applied**: imported as an undated departure claim with no `until` (no structured period, by the packet convention in A3) |
| B3 | "The President" headings treated inconsistently | **Applied**: one rule adopted and stated (a political report or address to Conference is the party President's act); the 2002 opening address is imported as a continuation claim for the 1997 Mbeki observation |
| B4 | Report introduction locator (paragraph 7) | **Applied** |
| B5 | Report introduction locator (paragraph 6) | **Applied** |
| B6 | "Modified 8 June 1998" cited without a source | **Applied**: phrase dropped; the later index capture is a lead |
| B7 | Stadium address locator | **Applied** |
| B8 | Summary called Mandela's closing address "the actual handover" | **Applied**: event kind `handover_speech`; described as a closing-session handover speech |
| B9 | Nomination ids carried the PDF creation date; embedded title not recorded | **Applied in part**: ids renamed without the date and the embedded title recorded; the homepage capture of 17 December 2007 is a lead (it gives no day, and the list is a claim only) |
| B10 | Results list labelled a result declaration | **Applied**: renamed `za_anc_conf52_officials_results_zuma_president`, event kind `national_conference_election_result_list`, no date; the 19 December publication stays its own claim |
| B11 | New-NEC id carried 21 December 2007 | **Applied**: renamed `za_anc_conf52_newly_elected_nec_zuma_president`; the uncertainty says the dateline belongs to the additional-member results |
| B12 | Zuma's heading silently corrected | **Applied**: the title quotes "52nd National Congress" as printed, "(sic, for Conference)" |
| B13 | Acceptance locator | **Applied** |
| B14 | Mbeki 2007 report locator | **Applied**: second passage added |
| B15 | ANC Today letter locator | **Applied** |
| B16 | Test pins and holder shape not spelled out | **Applied**: the s10h guards re-expressed as exact pins, counts and pins updated; holders in packet shape; the role scope note forbids filling the gaps |
| C1 | Hashes depend on the download method | **Applied**: every extract records `fetch_recipe`, content encoding and the base32 SHA-1; decoded identities for the two gzip captures |
| C2 | 31 August 2026 officials page is content of February 2023 | **Resolved by removal**: dropped and listed as a lead with its identity; the NEC page of 25 August 2026 is also a lead |
| C3 | ANC statements of 18 and 19 December 2012 missed | **Applied**: both imported as election references dated by the statements, never holder dates; summary, unresolved and failed-sources notes corrected |
| C4 | 20 December 2012 remarks never say "ANC President" | **Applied**: locator extended; the holder stays 20 December 2012 with the inference stated; the 6 January 2013 "Address by ANC President Jacob Zuma" imported as a continuation claim |
| C5 | Holders cited elections and lists; names in printed order | **Applied**: in-office attestations only; names normalised |
| C6 | 2022-term holder mixed in the ZA-ANC-10 claims | **Applied**: separate holder, Cyril Ramaphosa, 15 May 2026 |
| C7 | ZA-ANC-08 marked accepted | **Applied**: accepted in part |
| C8 | 54th officials row borrowed the Declaration's dates | **Applied**: no structured date |
| C9 | The 55th conference concluded on 5 January 2023 | **Applied**: the Declaration imported; the conference page relabelled as the scheduled Nasrec session; reliance on the 5 January lead dropped |
| C10 | `attested_period` invented day bounds | **Applied**: none remains; `printed_range` text in the rows |
| C11 | 2022 political report locator | **Applied** |
| C12 | Five-year-term claim period | **Applied**: no structured date, `printed_range` "December 2022" |
| C13 | 53rd officials list name and period | **Applied**: "Jacob Zuma", no structured date, `printed_range` "December 2012" |
| C14 | 15 May 2026 briefing day understated | **Applied**: on or before the evening of 14 May 2026; the date line is the publication date |
| C15 | More test guards than the dossier covered | **Applied**: s10h's organization-roles and ANC-roles guards re-expressed as exact pins; the C01-09 lead-marker loop excuses "anc1912" and "anc.org.za" only for this packet's sources and adds that no presidency source uses them |
| C16 | Continuation claims listed under the wrong observation | **Applied**: the 2012 report under ZA-ANC-06 and the 2022 report under ZA-ANC-08; the same rule for the 1994, 1997, 2002 and 2007 reports; the 2012 report is imported once |
| C17 | Integration fields missing | **Applied**: `observation_id`, `review_observation` and `source_type` everywhere; no archive header value copied |
| C18 | 2017 biography claim was a conference-election record | **Applied**: event kind `election_reference_retrospective` |

Missing primary records found by the checks: imported are the three 2021 anc1912 captures (part A, less the 49th
conference page), *Mayibuye*, the 1997 Declaration and the 2002 opening address (part B), and the ANC statements of 18
and 19 December 2012, the 6 January 2013 address, the 54th Declaration web page and the 55th Declaration (part C).
Leads, for the reasons given above: the 49th conference page capture, the 1998 index capture, the 2007 homepage, the
2007 programme, the two ANC Today letters and the 2026 NEC page.

Other changes made to fit the packet's rules rather than a numbered defect:

- Every ANC claim id begins `za_anc` (part B ids renamed), so the separation from `za_presidency` can be pinned by id.
- In-office attestations that do not date a holder are relabelled `in_office_continuation_attestation`; the holders cite
  only claims of their own day.
- `published_date` of the 2002 Conference Briefing is null (the day is not printed; "2003-02" is not an ISO day).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SouthAfrica-ANC-002`: the day each National Conference elected or declared its President (1991-2022) from ANC
  Electoral Commission records, conference reports or minutes, including the uncaptured old-site pages 9990, 9991, 9996
  and 9998.
- `C01-SouthAfrica-ANC-003`: Tambo's release by the NEC and any acting arrangement during his illness (1989-1991),
  including the earlier conference statement Mandela refers to.
- `C01-SouthAfrica-ANC-004`: whether ANC records state the day a President assumes or leaves the party office.
- `C01-SouthAfrica-ANC-005`: Deputy Presidents, National Chairpersons and Secretaries General as separate roles
  (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** the branch is stacked on `claude/c01-za-09` (`82a23f9d`), which is based on `ffe54b02`
  (`codex/campaign-certification`) and is ready for review but not integrated; merge CLAUDE-C01-09 first. Before this
  work `origin/claude/c01-za-09` was fetched and had no commit missing from this branch, so no merge was needed. The
  claim commit `6e9a489b` holds only the handoff. `south-africa.json` is shared only with the stacked CLAUDE-C01-09.
- `research-index.json` is regenerated in a **separate commit** and is the only file this packet shares with other
  pending packets; regenerate it when integrating. New totals: 265 sources and 1,913 claims (previously 211 and 1,840).
  South Africa role observations rise from 5 to 6; organization, institution, packet and batch counts are unchanged,
  `mapping_pending` stays 53 and South Africa keeps six open batches.
- Pinned tests, none loosened and no assertion removed:
  - `test_south_africa_research_s10h.py`: counts (entries, sources, claims, roles) are now (53, 109, 260, 6); the rule
    that only the DA has roles is now an exact pin (the DA's three roles, the ANC's one, every other organization none);
    the ANC guard `anc['roles'] == []` is now an exact pin of the ANC role, its holders and its separation from the
    presidency; response pins, the source list, hosts and access dates (2026-09-24 for this packet) and the undated
    claim count (7 + 31) are extended exactly; the organization holders keep the null-only rule.
  - `test_south_africa_heads_of_state_c01_09.py`: the exact source list is extended, and the lead-marker loop excuses
    "anc1912" and "anc.org.za" only for this packet's sources while adding that no presidency source uses them.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and
  the sparse checkout was not widened.
- New extract fields: `source_response_sha1_base32`, `source_response_content_encoding`, `decoded_response_bytes` and
  `decoded_response_sha256` (gzip captures), `fetch_recipe` and `stability_check`; rows reuse `printed_range` and
  `review_observation` from CLAUDE-C01-09.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this
  handoff is self-proposed and not registered there. No UI code changed.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_south_africa*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
