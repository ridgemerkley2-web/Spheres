# Deputy Presidents 21: South Africa, 1994-2026

Packet: **CLAUDE-C01-21**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-za-21`, **stacked on CLAUDE-C01-16** (`claude/c01-za-16`
at `2df4a0a6`, itself stacked on CLAUDE-C01-09, `claude/c01-za-09`, which is based on `codex/campaign-certification` at
`ffe54b02`); claim commit `e2d1a1ec`. Research access: 25 September 2026. The historical cutoff stays **7 September
2026**.

This packet adds one role, `za_deputy_president` (Deputy President of the Republic of South Africa, kind
`institutional_office`), to the existing `za_presidency` institution in [south-africa.json](south-africa.json), and
reviews ten observations from 10 May 1994 to the cutoff. It adds 59 sources and 82 claims, the role with twelve holder
observations, a role scope note, and coverage notes on the institution and the packet. The CLAUDE-C01-09 roles
(`za_president_election`, `za_state_president`), their claims and holders, and the CLAUDE-C01-16 ANC role and holders
are unchanged, and none of this packet's claims or sources feeds them. It adds no organization, institution, game
mapping, lifespan, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1994-1999; B: 2005-2009; C: 2014-2026), and each part was checked
independently before this packet was written. Every checker defect is applied, and every missing primary record the
checks found is imported or explained (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| ZA-DP-01 | 1994: the appointments of Thabo Mbeki and F. W. de Klerk as Executive Deputy Presidents | **Accepted in part:** de Klerk observed 10 May 1994 ("my Second Deputy President" in the President's inauguration statement); Mbeki observed 25 May 1994 (UN Security Council record, with the Department of Foreign Affairs' copy of his statement); no designation, appointment or oath record, so no start; the Presidency's list starts both on 13 May 1994 (claim only) |
| ZA-DP-02 | 1996: de Klerk's withdrawal from the Government of National Unity and the end of his office | **Accepted in part:** on 9 May 1996 he told the President of the National Party's withdrawal and was thanked as Deputy President; the President's statement of 13 May 1996 dates ministerial vacancies "as from 1 July 1996" and names nobody; no source states the day his office ended (30 June 1996 only in the retrospective list) |
| ZA-DP-03 | 1999: Jacob Zuma's appointment | **Accepted:** General Notice 1392 of 1999 appoints him "with effect from 17 June 1999" (the start); the notice was published on 2 July 1999; he is in office on 28 June 1999 (Hansard) |
| ZA-DP-04 | 2005: Zuma's release and Phumzile Mlambo-Ngcuka's appointment | **Accepted in part:** the release was announced at the Joint Sitting of 14 June 2005 and accepted that day, with no stated effective day; Mlambo-Ngcuka observed 22 June 2005 (the Assembly noted "the appointment today"; the Presidency profile says she assumed her duties that day); her oath was scheduled for 23 June 2005 at 14h00 and no record says it took place; no start |
| ZA-DP-05 | 2008: Mlambo-Ngcuka's resignation and Baleka Mbete's appointment | **Accepted in part:** the resignation was accepted on 23 September 2008 and made effective only by reference to the President's resignation, so no end; Mbete was designated on 25 September 2008 (intention, Cabinet list as announced, members' remarks) and is observed on 21 October 2008, when the Speaker named "the Deputy President, Ms B Mbete"; no oath record |
| ZA-DP-06 | 2009: Kgalema Motlanthe's appointment | **Accepted:** announced 10 May 2009 ("will be"); sworn in 11 May 2009 (Presidency photo caption), which dates the observation |
| ZA-DP-07 | 2014: Cyril Ramaphosa's appointment | **Accepted in part:** announced 25 May 2014; observed 30 May 2014 (the President's letter naming "the Deputy President of the Republic, Mr M C Ramaphosa"); no primary record of the oath |
| ZA-DP-08 | 2018 and 2019: David Mabuza's appointments | **Accepted in part:** 26 February 2018 intention and stated appointment day, Assembly seat from 26 February, sworn in 27 February 2018 (Presidency profile); announced 29 May 2019, Assembly oath 28 May, sworn in again 30 May 2019 (profile only) |
| ZA-DP-09 | 2023: Mabuza's resignation and Paul Mashatile's appointment | **Accepted in part:** step-down request announced 16 February 2023; Assembly seat resigned "as of 28 February 2023"; the Presidency said on 1 March 2023 that this ended his term, without a day, so no end; Mashatile announced 6 March and sworn in 7 March 2023 |
| ZA-DP-10 | 2024: Mashatile's reappointment, and an official attestation before the cutoff | **Accepted in part:** reappointment announced 30 June 2024; oath scheduled for 3 July 2024 with no primary record that it took place; observed 4 July 2024 (the President's letter) and 30 August 2026 (a statement issued by the Presidency for his office) |

The resulting holder observations of `za_deputy_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| F. W. de Klerk | 1994-05-10 | null | null | the President's inauguration statement: "my Second Deputy President, the Honourable F.W. de Klerk" |
| Thabo Mbeki | 1994-05-25 | null | null | UN Security Council record S/PV.3379 ("First Executive Deputy President") and the Department of Foreign Affairs' copy of his statement ("First Deputy Vice-President", as printed) |
| Jacob Zuma | null | 1999-06-17 | null | General Notice 1392 of 1999: "with effect from 17 June 1999" |
| Phumzile Mlambo-Ngcuka | 2005-06-22 | null | null | the Presidency profile's stated assumption of duties and the National Assembly's note of "the appointment today" |
| Baleka Mbete | 2008-10-21 | null | null | National Assembly Hansard: "the Deputy President, Ms B Mbete" |
| Kgalema Motlanthe | 2009-05-11 | null | null | Presidency caption "being sworn in as Deputy President - 11 May 2009" |
| Cyril Ramaphosa | 2014-05-30 | null | null | the President's letter of 30 May 2014 (Announcements, Tablings and Committee Reports No 6 of 2014) |
| David Mabuza | 2018-02-27 | null | null | Presidency profile: "first sworn in as Deputy President ... on 27 February 2018" |
| David Mabuza | 2019-05-30 | null | null | Presidency profile: sworn in "again on Thursday, 30 May 2019" |
| Paul Mashatile | 2023-03-07 | null | null | Presidency statement of 11 April 2023 and profile: sworn in on 7 March 2023 |
| Paul Mashatile | 2024-07-04 | null | null | the President's letter of 4 July 2024 (Announcements, Tablings and Committee Reports No 4 of 2024) |
| Paul Mashatile | 2026-08-30 | null | null | statement issued by the Presidency for the Office of the Deputy President |

### How a start and an end are decided

The packet applies the CLAUDE-C01-09 rule unchanged. A holder has `from` only where a source states the day the office
was assumed or took effect, and `until` only where a source states the day it ended; otherwise it is a dated
observation (`attested_on`). Each holder cites only claims of its own day that may date a holder: an oath of office, a
stated assumption, an in-office attestation, the Assembly's note of the appointment and, for Zuma's start, the stated
effective day of the appointment. The test pins that every cited claim carries exactly the holder's date and one of
those event kinds. Everything else is a claim that never feeds a holder: announcements and stated intentions, the
Cabinet list "as Announced", appointment notices and retrospective appointment statements, scheduled oaths, releases,
resignations, their acceptance and stated effect, "former" stylings, Assembly-seat events, retrospective lists,
profiles and directory spans, and continuation attestations (a later in-office attestation of the same observation).

Only one start is stated (Zuma, 17 June 1999). No end is stated for any holder:

- De Klerk: the withdrawal was communicated on 9 May 1996; the President dated ministerial vacancies "as from 1 July
  1996" without naming him; only the Presidency's retrospective list gives 30 June 1996.
- Zuma: the release was announced and accepted on 14 June 2005 with no effective day.
- Mlambo-Ngcuka: the Presidency's statement of 23 September 2008 names only "the Deputy President" and makes the
  resignations effective on the day the President's resignation takes effect. The National Assembly fixed that day as
  25 September 2008 (a CLAUDE-C01-09 claim), but setting her `until` would combine two sources and identify an unnamed
  holder, so no end is set. This decides the part B researcher's first requested ruling, as the check recommended.
- Mabuza: his letter resigned his Assembly seat "as of 28 February 2023", and the Presidency said on 1 March 2023 that
  the resignation ended his term but gave no day; the end of an Assembly seat is not the end of the office.
- Every other end could come only from a successor's appointment or oath, which is never used.

Two stated or apparent days are deliberately not used as starts. Mlambo-Ngcuka's profile says she "assumed her
duties" on 22 June 2005, but the same profile dates her appointment to 21 June, and the Presidency's own advisory
scheduled her oath for 23 June at 14h00; with that conflict, 22 June dates the observation only (check defect B3; the
part B researcher's second requested ruling). Oath statements (Motlanthe 2009, Mabuza 2018 and 2019, Mashatile 2023)
date observations and are never starts, as in CLAUDE-C01-09.

### The Deputy President and the President's roles

`za_deputy_president` is an `institutional_office`, never a head-of-state role; it sits in `za_presidency` after the
two CLAUDE-C01-09 roles. No claim or source of this role is cited by `za_president_election`, `za_state_president` or
`za_anc_president`, and none of theirs is cited by this role; the test pins the President's, State President's and ANC
holders exactly and rejects any cross-role holder or claim. A Deputy President acting as President (for example the
July 1996 proclamations signed "T. M. MBEKI Acting President", a lead) is a claim only and never a holder of
`za_president_election` or of this role; no acting claim is imported in this packet. Printed titles ("Second Deputy
President", "First Executive Deputy President", "Executive Deputy President", "First Deputy Vice-President") stay in
the claim text; every row carries the role title "Deputy President of the Republic of South Africa".

### Responses already cited by CLAUDE-C01-09

Four responses that CLAUDE-C01-09 cites also carry deputy-president rows: the Presidency's history list, the Hansard
of 25 September 2008, the Presidency's Cabinet list of 25 September 2008 and the gov.za directory entry for Motlanthe.
Their CLAUDE-C01-09 extracts say deputy presidents are out of scope, and the CLAUDE-C01-09 test pins those extracts'
rows, observations and access dates exactly. So this packet does not edit them: it adds a separate source record for
each (`za_presidency_history_list_deputy_presidents`, `za_parliament_hansard_na_20080925_deputy_president`,
`za_presidency_cabinet_list_20080925_deputy_president`, `za_govza_directory_motlanthe_deputy_president`) with the same
URL and response identity and its own extract of the deputy-president rows only (check defects A7, A8, B9). The
validator rejects duplicate source and claim ids, not a second record of the same URL; the new test pins that each pair
has the same URL and identity, that the new rows are absent from the CLAUDE-C01-09 extracts and that those extracts
keep their scope. No existing extract is edited.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| 10 May 1994 | The President's inauguration statement: "my Second Deputy President, the Honourable F.W. de Klerk" | `za_de_klerk_styled_second_deputy_president_inauguration_19940510`; de Klerk `attested_on` |
| 13 May 1994 | Retrospective list start for both Executive Deputy Presidents | `za_presidency_list_deputy_president_mbeki_19940513_19990616`, `za_presidency_list_deputy_president_de_klerk_19940513_19960630` (no structured date) |
| 25 May 1994 | UN Security Council receives Mbeki as "First Executive Deputy President"; the Department of Foreign Affairs' copy heads him "First Deputy Vice-President" | `za_mbeki_first_executive_deputy_president_unsc_19940525`, `za_dfa_mbeki_first_deputy_statement_security_council_19940525`; Mbeki `attested_on` |
| 3 June 1994 | Notice 1065: ministers appointed after consulting "the Executive Deputy Presidents" (unnamed) | `za_executive_deputy_presidents_consulted_on_cabinet_19940603` |
| May 1994 - 13 June 1999 | Mbeki profile span | `za_presidency_profile_mbeki_executive_deputy_president_span` (no structured date) |
| 18 April 1996 | Statement of the Office of Executive Deputy President T M Mbeki | `za_mbeki_executive_deputy_president_rdp_statement_19960418` (continuation) |
| 9 May 1996 | De Klerk tells the President of the National Party's withdrawal; thanked as Deputy President | `za_np_gnu_withdrawal_communicated_by_de_klerk_19960509`, `za_de_klerk_deputy_president_thanked_in_office_19960509` (continuation) |
| 13 May 1996 | One Executive Deputy President; portfolios vacant "as from 1 July 1996" (names nobody) | `za_single_executive_deputy_president_and_vacancies_from_19960701_announced_19960513` |
| 30 June 1996 | Retrospective list end for de Klerk (never an end) | `za_presidency_list_deputy_president_de_klerk_19940513_19960630` |
| 17 June 1999 | Zuma's appointment "with effect from 17 June 1999" | `za_zuma_deputy_president_appointment_effective_19990617`; Zuma `from` |
| 28 June 1999 | Speaker: "the Deputy President, Mr J G Zuma", Leader of Government Business | `za_zuma_deputy_president_leader_of_government_business_announced_19990628` (continuation) |
| 2 July 1999 | Notice 1392 published | `za_zuma_deputy_president_appointment_gazetted_19990702` |
| 1999 | Profile: "In 1999 Mr Zuma was appointed" | `za_presidency_profile_zuma_appointed_deputy_president_1999` (no structured date) |
| 14 June 2005 | Joint Sitting (14:02-14:19): the President releases Zuma; the "void" left by his departure; Zuma accepts | `za_mbeki_announces_release_of_zuma_joint_sitting_20050614`, `za_mbeki_releases_zuma_as_deputy_president_20050614`, `za_zuma_departure_void_in_executive_20050614`, `za_zuma_accepts_release_20050614` |
| 20 June 2005 | Presidency: "former Deputy President Jacob Zuma" | `za_zuma_styled_former_deputy_president_20050620` |
| 21 June 2005 | Profile: appointed on 21 June 2005 (contradicted) | `za_mlambo_ngcuka_appointed_per_profile_20050621` |
| 22 June 2005 | The President tells Cabinet of his decision; the Assembly notes "the appointment today"; profile: "assumed her duties" | `za_mlambo_ngcuka_appointment_decision_announced_20050622`, `za_mlambo_ngcuka_appointment_noted_by_assembly_20050622`, `za_mlambo_ngcuka_assumed_duties_20050622`; Mlambo-Ngcuka `attested_on` |
| 23 June 2005 | Swearing-in scheduled for 14h00 at Tuynhuys (no record it took place) | `za_mlambo_ngcuka_swearing_in_scheduled_20050623` |
| 23 September 2008 | Resignation announced (release list); accepted (title only); effective by reference to the President's resignation | `za_mlambo_ngcuka_announces_resignation_20080923`, `za_deputy_president_resignation_accepted_20080923`, `za_deputy_president_resignation_effect_by_reference_20080923` |
| 25 September 2008 | The President intends appointing Mrs Baleka Mbete; she vacates the Speakership; members' remarks; Cabinet list "as Announced" | `za_motlanthe_intends_appointing_mbete_deputy_president_20080925`, `za_mbete_vacates_speakership_for_deputy_presidency_20080925`, `za_mbete_designation_congratulated_in_house_20080925`, `za_mbete_deputy_president_cabinet_list_20080925` |
| 21 October 2008 | Speaker: "the Deputy President, Ms B Mbete"; the Minister of Finance: "former Deputy President Mlambo-Ngcuka" | `za_mbete_deputy_president_logb_announced_20081021` (Mbete `attested_on`), `za_mlambo_ngcuka_styled_former_deputy_president_20081021` |
| since September 2008 | Mbete profile (captured 30 April 2009) | `za_presidency_profile_mbete_deputy_since_september_2008` (no structured date) |
| 10 May 2009 | The Deputy President "will be" Motlanthe | `za_motlanthe_deputy_president_announced_20090510` |
| 11 May 2009 | "Kgalema Motlanthe being sworn in as Deputy President - 11 May 2009" | `za_motlanthe_sworn_in_deputy_president_caption_20090511`; Motlanthe `attested_on` |
| 25 May 2014 | "The Deputy President is Mr Cyril Ramaphosa"; profile: appointed 25 May 2014 | `za_ramaphosa_dp_appointment_announced_20140525`, `za_ramaphosa_styled_dp_20140525`, `za_presidency_profile_ramaphosa_dp_appointed_20140525` |
| 30 May 2014 | The President's letter: "the Deputy President of the Republic, Mr M C Ramaphosa" | `za_ramaphosa_deputy_president_logb_letter_20140530`; Ramaphosa `attested_on` |
| 18 June 2014 | Assembly announcement (Minutes; digest) | `za_ramaphosa_logb_designation_announced_minutes_20140618`, `za_ramaphosa_dp_logb_designation_announced_20140618` (continuation) |
| 16 July 2014 | Appointments communicated to the Assembly | `za_ramaphosa_dp_appointment_communicated_na_20140716` |
| 26 February 2018 | Intention "pending ... swearing-in as Members"; Assembly seat filled; advisory: appointed that day | `za_mabuza_dp_appointment_intended_20180226`, `za_mabuza_assembly_seat_filled_20180226`, `za_mabuza_dp_appointed_20180226` |
| 27 February 2018 | Swearing-in scheduled for 16h00; profile: first sworn in | `za_mabuza_dp_swearing_in_scheduled_20180227`, `za_presidency_profile_mabuza_sworn_in_20180227`; Mabuza 2018 `attested_on` |
| 1 March 2018 | The President's letter: "the Deputy President ..., Mr David Dabede Mabuza" | `za_mabuza_deputy_president_logb_letter_20180301` (continuation) |
| 28 May 2019 | Assembly oath as a Member | `za_mabuza_assembly_oath_20190528` |
| 29 May 2019 | "Deputy President is David Mabuza" | `za_mabuza_dp_appointment_announced_20190529` |
| 30 May 2019 | Ministers' swearing-in scheduled (no Deputy President named); profile: sworn in again | `za_ministers_swearing_in_scheduled_20190530`, `za_presidency_profile_mabuza_sworn_in_20190530`; Mabuza 2019 `attested_on` |
| 5 and 6 June 2019 | The President's letter; Parliament's release | `za_mabuza_deputy_president_logb_letter_20190605`, `za_mabuza_styled_dp_parliament_20190606` (continuation) |
| 16 February 2023 | Reply to the State of the Nation debate: styled Deputy President; step-down request announced (and two later references to it) | `za_mabuza_styled_dp_sona_reply_20230216`, `za_mabuza_step_down_request_sona_reply_20230216`, `za_mabuza_step_down_request_referenced_presidency_20230216`, `za_mabuza_step_down_request_referenced_cabinet_statement_20230216` |
| 28 February 2023 | Letter resigning his Assembly seat "as of 28 February 2023"; resignation received | `za_mabuza_assembly_resignation_letter_20230228`, `za_mabuza_assembly_resignation_received_20230228` |
| 1 March 2023 | Presidency: the resignation ends his term (no day); a new Deputy President to be announced | `za_mabuza_resignation_ends_dp_term_20230301`, `za_deputy_president_successor_to_be_announced_20230301` |
| 6 March 2023 | "I have decided to appoint Mr Paul Mashatile" | `za_mashatile_dp_appointment_announced_20230306` |
| 7 March 2023 | Swearing-in scheduled (18h00, Tuynhuys); sworn in (statement of 11 April 2023; profile) | `za_national_executive_swearing_in_scheduled_20230307`, `za_mashatile_dp_sworn_in_20230307`, `za_presidency_profile_mashatile_sworn_in_20230307`; Mashatile 2023 `attested_on` |
| 8 March 2023 | The President's letter: "Mr Shipokosa Paulus Mashatile, the Deputy President" | `za_mashatile_deputy_president_logb_letter_20230308` (continuation) |
| 30 June 2024 | "The Deputy President is Paul Mashatile" | `za_mashatile_dp_reappointment_announced_20240630` |
| 1 July 2024 | Swearing-in scheduled for 3 July 2024 | `za_national_executive_swearing_in_scheduled_20240701` |
| 4 July 2024 | The President's letter: "Deputy President S P Mashatile" | `za_mashatile_deputy_president_logb_letter_20240704`; Mashatile 2024 `attested_on` |
| 18 July 2024 | Opening of Parliament Address: "Deputy President Paul Mashatile" | `za_mashatile_styled_dp_20240718` (continuation) |
| 30 August 2026 | Statement for the Office of the Deputy President | `za_mashatile_dp_in_office_20260830`; Mashatile 2026 `attested_on` |

Retrospective directory and profile spans (Motlanthe "from 11 May 2009 to 25 May 2014"; Mabuza "27 February 2018 until
6 March 2023"; Mashatile "from 3 July 2024" and "from 6 March 2023 to 19 June 2024"; Mlambo-Ngcuka "22 June 2005 - 23
September 2008"; the history list rows) and the Mashatile profile's current title as captured on 11 June 2026 carry no
structured date. Date conventions follow CLAUDE-C01-09: `attested_on` is the day of the observed event as the source
dates it; a page's own issue date is `published_date`; a prospective notice is dated by the day it was made; a later
statement that refers to an earlier announcement is dated by the day it refers to and labelled a retrospective
reference.

## Observations

### ZA-DP-01 — 1994: the two Executive Deputy Presidents

Evidence: the gov.za copy of the President's statement at his inauguration, dated 10 May 1994 and "Issued by The
Presidency", thanks the leaders of the transition and names "my Second Deputy President, the Honourable F.W. de Klerk"
(`za_de_klerk_styled_second_deputy_president_inauguration_19940510`). The UN Security Council's provisional verbatim
record of its 3379th meeting (New York, 25 May 1994) escorts "Mr. Thabo Mbeki, First Executive Deputy President of the
Republic of South Africa" to the Council table and invites him to speak first
(`za_mbeki_first_executive_deputy_president_unsc_19940525`); the Department of Foreign Affairs' archived copy of the
statement he gave there heads him "First Deputy Vice-President of South Africa" and conveys the greetings of "our
President, Nelson Mandela" (`za_dfa_mbeki_first_deputy_statement_security_council_19940525`, located by the check).
Government Notice 1065 of 3 June 1994 appoints ministers with effect from 11 May 1994 "after consultation with the
Executive Deputy Presidents", naming neither (`za_executive_deputy_presidents_consulted_on_cabinet_19940603`). A press
statement "Issued by: The Office of Executive Deputy President T M Mbeki" on 18 April 1996 names "Executive Deputy
President Thabo Mbeki" (`za_mbeki_executive_deputy_president_rdp_statement_19960418`, located by the check). The
Presidency's history list gives both men "13 May 1994" and Mbeki's profile "May 1994 - 13 June 1999" (claims only).

Decision: accepted in part. De Klerk is observed on 10 May 1994 and Mbeki on 25 May 1994, with no start: no
designation, appointment or oath record of May 1994 was found, and the list's 13 May 1994 is unexplained. Mbeki's
observation rests on the UN record, an international body's record of the title under which the Republic's delegate
was received, together with the South African Department of Foreign Affairs' copy of the same day, which mis-styles
the office; his own office's statement of 18 April 1996 is the correctly styled South African attestation, recorded
as a continuation claim (check defect A5, decided by keeping 25 May 1994 and citing both same-day records).

Limits: the 1994 Hansard is not in Parliament's digital archive, and the large 1994 Gazettes listed in the part A
dossier were not searched (see [Suggested next work orders](#suggested-next-work-orders)).

### ZA-DP-02 — 1996: de Klerk's withdrawal and the end of his office

Evidence: the President's statement of 9 May 1996 (government memorial-site copy of the info.gov.za text) says that
"Deputy-President FW de Klerk informed me earlier today that the National Party had decided to withdraw from the
Government of National Unity" (`za_np_gnu_withdrawal_communicated_by_de_klerk_19960509`) and thanks "Deputy President
FW de Klerk and his colleagues", being "confident that we shall continue to work together"
(`za_de_klerk_deputy_president_thanked_in_office_19960509`). His statement of 13 May 1996 appoints ministers to
portfolios "that will be left vacant as from 1 July 1996" and says "South Africa will now have one Executive Deputy
President" (`za_single_executive_deputy_president_and_vacancies_from_19960701_announced_19960513`); it names nobody.

Decision: accepted in part. The withdrawal is recorded, but no source states the day de Klerk's office ended; 1 July
1996 is the day the portfolios become vacant, and 30 June 1996 appears only in the Presidency's retrospective list. His
holder has no `until`.

### ZA-DP-03 — 1999: Jacob Zuma's appointment

Evidence: General Notice 1392 of 1999 (Office of the Presidency, Government Gazette No. 20261, Pretoria, 2 July 1999)
notifies the decision "to appoint Mr J. G. Zuma as Deputy President with effect from 17 June 1999"
(`za_zuma_deputy_president_appointment_effective_19990617`), published on 2 July 1999
(`za_zuma_deputy_president_appointment_gazetted_19990702`). On 28 June 1999 the Speaker announced that "the Deputy
President, Mr J G Zuma" had been appointed Leader of Government Business
(`za_zuma_deputy_president_leader_of_government_business_announced_19990628`). The Presidency's profile says "In 1999"
and its list "17 June 1999 - 14 June 2005" (claims only).

Decision: accepted. Zuma's `from` is 17 June 1999, the effective day the notice states. The archive file carries only
page 1 of No. 20261, so the rest of the notice (signature, decision date) was not seen, and no announcement or oath
record was found.

### ZA-DP-04 — 2005: Zuma's release and Phumzile Mlambo-Ngcuka's appointment

Evidence: at the Joint Sitting of 14 June 2005 (members assembled at 14:02; the sitting rose at 14:19) the President
said "it would be best to release hon Jacob Zuma from his responsibilities as Deputy President of the Republic and
member of the Cabinet" and spoke of "the void that the departure of Deputy President Jacob Zuma has created"
(`za_mbeki_announces_release_of_zuma_joint_sitting_20050614`, Parliament's Hansard, located by the check; the
government information site's copy gives the same words: `za_mbeki_releases_zuma_as_deputy_president_20050614`,
`za_zuma_departure_void_in_executive_20050614`). Zuma accepted the decision that day
(`za_zuma_accepts_release_20050614`), and on 20 June 2005 the Presidency called him "former Deputy President Jacob
Zuma" (`za_zuma_styled_former_deputy_president_20050620`). On 22 June 2005 the President told Cabinet of his decision
to appoint Mlambo-Ngcuka (`za_mlambo_ngcuka_appointment_decision_announced_20050622`), and the National Assembly agreed
to a resolution that "notes the appointment today of the Hon Phumzile Mlambo-Ngcuka as Deputy President of the
Republic of South Africa" (`za_mlambo_ngcuka_appointment_noted_by_assembly_20050622`, located by the check). The
Presidency's advisory scheduled her swearing-in for Thursday 23 June 2005 at 14h00 at Tuynhuys
(`za_mlambo_ngcuka_swearing_in_scheduled_20050623`). The Presidency's profile captured in February 2007 says she
"assumed her duties as the Deputy President of the Republic of South Africa on 22 June 2005"
(`za_mlambo_ngcuka_assumed_duties_20050622`) and, in its last paragraph, that the President appointed her on 21 June
2005 (`za_mlambo_ngcuka_appointed_per_profile_20050621`).

Decision: accepted in part. No source states the day or time Zuma's release took effect, so his holder has no
`until`; the list's 14 June 2005 and the 20 June "former" styling are claims. Mlambo-Ngcuka is observed on 22 June 2005
by the Assembly's note of the appointment and the profile's stated assumption, with no start (check defect B3): the
profile contradicts itself (21 June) and the Presidency's advisory put the oath a day later, and no record that the
oath took place was found.

### ZA-DP-05 — 2008: Mlambo-Ngcuka's resignation and Baleka Mbete's appointment

Evidence: the Presidency's list of its releases (captured 14 October 2008) dates to 23 September 2008 "Deputy
President Phumzile Mlambo-Ngcuka announces her resignation as Deputy President of the Republic and as a Member of
Parliament" (`za_mlambo_ngcuka_announces_resignation_20080923`); the release itself was never archived. The Presidency's
statement of 23 September 2008 says the President had accepted letters of resignation, the first from "the Deputy
President" (title only), effective from the day the President's resignation takes effect
(`za_deputy_president_resignation_accepted_20080923`, `za_deputy_president_resignation_effect_by_reference_20080923`).
On 25 September 2008 President Motlanthe told the Assembly that he intended appointing, "For the position of Deputy
President, Mrs Baleka Mbete" (`za_motlanthe_intends_appointing_mbete_deputy_president_20080925`); she rose to vacate the
Speakership (`za_mbete_vacates_speakership_for_deputy_presidency_20080925`); members congratulated her on her
"nomination as the Deputy President" and "this wonderful opportunity she is being given to become the Deputy President"
(`za_mbete_designation_congratulated_in_house_20080925`); the Presidency's "Members of Cabinet as Announced" lists "The
Deputy President: Ms B Mbete" (`za_mbete_deputy_president_cabinet_list_20080925`). On 21 October 2008 the Speaker
announced that the President had appointed "the Deputy President, Ms B Mbete", as Leader of Government Business with
effect from 6 October 2008, and the Minister of Finance thanked "former Deputy President Mlambo-Ngcuka"
(`za_mbete_deputy_president_logb_announced_20081021`, `za_mlambo_ngcuka_styled_former_deputy_president_20081021`, both
located by the check). The Presidency's 2009 profile says "since September 2008" and its list "5 September 2008".

Decision: accepted in part. No end for Mlambo-Ngcuka (see [How a start and an end are decided](#how-a-start-and-an-end-are-decided)).
Mbete is observed on 21 October 2008, not 25 September 2008: every record of 25 September is a designation (check
defect B6, option b), and no oath, effective day or stated assumption was found. The 6 October 2008 effect is the
Leader of Government Business appointment's.

### ZA-DP-06 — 2009: Kgalema Motlanthe's appointment

Evidence: the President's statement of 10 May 2009 says the Deputy President "will be" Mr Kgalema Petros Motlanthe
(`za_motlanthe_deputy_president_announced_20090510`); the same Presidency page, captured on 18 May 2009, captions a
photograph "Kgalema Motlanthe being sworn in as Deputy President - 11 May 2009"
(`za_motlanthe_sworn_in_deputy_president_caption_20090511`). The gov.za directory's "from 11 May 2009 to 25 May 2014" is
a claim only.

Decision: accepted. Motlanthe is observed on 11 May 2009, the day of his oath (not a start); time and officiant are not
stated.

### ZA-DP-07 — 2014: Cyril Ramaphosa's appointment

Evidence: the President's statement of 25 May 2014 says "The Deputy President is Mr Cyril Ramaphosa" and wishes
"Deputy President Ramaphosa" well (`za_ramaphosa_dp_appointment_announced_20140525`, `za_ramaphosa_styled_dp_20140525`);
the Presidency's profile says he was appointed on 25 May 2014
(`za_presidency_profile_ramaphosa_dp_appointed_20140525`). Announcements, Tablings and Committee Reports No 6 of 9 June
2014 record the President's letter of 30 May 2014 appointing "the Deputy President of the Republic, Mr M C Ramaphosa"
as Leader of Government Business (`za_ramaphosa_deputy_president_logb_letter_20140530`, located by the check); the
Assembly heard the designation on 18 June 2014 (Minutes No 2 of 2014 and Procedural Developments Issue 21), and on 16
July 2014 the President communicated his appointments to the Assembly.

Decision: accepted in part. Ramaphosa is observed on 30 May 2014 (check defect C1), the earliest official record found
that styles him the serving Deputy President after the announcement; no primary record of his oath was found (a news
agency's report of 26 May 2014 is a lead), so there is no start. No source reviewed states the day his deputy
presidency ended in February 2018.

### ZA-DP-08 — 2018 and 2019: David Mabuza's appointments

Evidence: on 26 February 2018 the President said that, "Pending the completion of their swearing-in as Members of the
National Assembly", he intended to appoint Mr David Mabuza as Deputy President
(`za_mabuza_dp_appointment_intended_20180226`); the Assembly's Minutes of 27 February 2018 record Mabuza's seat filled
"with effect from 26 February 2018" and an undated oath in the Speaker's office (`za_mabuza_assembly_seat_filled_20180226`,
located by the check); the Presidency's advisory of 27 February says he was appointed on Monday 26 February and
schedules the swearing-in of the new Deputy President for 16h00 that day (`za_mabuza_dp_appointed_20180226`,
`za_mabuza_dp_swearing_in_scheduled_20180227`); the Presidency's profile says he was "first sworn in as Deputy President
... on 27 February 2018" (`za_presidency_profile_mabuza_sworn_in_20180227`); the President's letter of 1 March 2018
names "the Deputy President of the Republic of South Africa, Mr David Dabede Mabuza"
(`za_mabuza_deputy_president_logb_letter_20180301`, located by the check). In 2019: "Deputy President is David Mabuza"
(29 May; `za_mabuza_dp_appointment_announced_20190529`); his Assembly oath on 28 May (`za_mabuza_assembly_oath_20190528`);
the advisory of 30 May names only Ministers and Deputy Ministers (`za_ministers_swearing_in_scheduled_20190530`); the
profile says he was sworn in "again on Thursday, 30 May 2019" (`za_presidency_profile_mabuza_sworn_in_20190530`); the
President's letter of 5 June 2019 and Parliament's release of 6 June 2019 style him Deputy President
(`za_mabuza_deputy_president_logb_letter_20190605`, located by the check; `za_mabuza_styled_dp_parliament_20190606`).

Decision: accepted in part. Two observations, 27 February 2018 and 30 May 2019, each dated by the oath the profile
states (not starts); the 2019 oath rests on the retrospective profile alone. The intention, the stated appointment day,
the Assembly seat and the scheduling notices are claims.

### ZA-DP-09 — 2023: Mabuza's resignation and Paul Mashatile's appointment

Evidence: closing his reply to the State of the Nation debate on 16 February 2023, the President said that "Deputy
President Mabuza has indicated his wish to step down" (`za_mabuza_step_down_request_sona_reply_20230216`; two later
statements refer back to it). Mabuza's letter dated 28 February 2023 tendered "his resignation as a Member of
Parliament as of 28 February 2023" (`za_mabuza_assembly_resignation_letter_20230228`, Announcements, Tablings and
Committee Reports No 25 of 2023, located by the check), and Parliament's release says it "was received on 28 February
2023, and is effective immediately" (`za_mabuza_assembly_resignation_received_20230228`). The Presidency advised on 1
March 2023 that he "has resigned as a Member of Parliament, ending his term as Deputy President" and that the President
"will make an announcement in due course about the appointment of a new Deputy President"
(`za_mabuza_resignation_ends_dp_term_20230301`, `za_deputy_president_successor_to_be_announced_20230301`). On 6 March
2023 the President decided "to appoint Mr Paul Mashatile as Deputy President of Republic"
(`za_mashatile_dp_appointment_announced_20230306`); the advisory of 7 March scheduled the swearing-in for 18h00 at
Tuynhuys; the Presidency's statement of 11 April 2023 and its profile say he was sworn in on 7 March 2023
(`za_mashatile_dp_sworn_in_20230307`, `za_presidency_profile_mashatile_sworn_in_20230307`); the President's letter of 8
March 2023 names "Mr Shipokosa Paulus Mashatile, the Deputy President" (`za_mashatile_deputy_president_logb_letter_20230308`,
located by the check).

Decision: accepted in part. Mabuza's 2019 observation has no `until`: the Presidency gives no day, and the Assembly
seat's stated day is not the office's (check defect C4); the statement does not say the office is vacant (C5).
Mashatile is observed on 7 March 2023, his oath day (not a start).

### ZA-DP-10 — 2024: the reappointment and an attestation before the cutoff

Evidence: on 30 June 2024 the President announced "The Deputy President is Paul Mashatile"
(`za_mashatile_dp_reappointment_announced_20240630`); the advisory page-dated 1 July 2024 scheduled the swearing-in of
the new Deputy President for 3 July 2024 (`za_national_executive_swearing_in_scheduled_20240701`); Announcements,
Tablings and Committee Reports No 4 of 5 July 2024 record the President's letter dated "4 July" appointing "Deputy
President S P Mashatile" as Leader of Government Business (`za_mashatile_deputy_president_logb_letter_20240704`,
located by the check); the Opening of Parliament Address of 18 July 2024 addresses "Deputy President Paul Mashatile"
(`za_mashatile_styled_dp_20240718`). A statement dated 30 August 2026 and issued by the Presidency for the Office of the
Deputy President says his programme and official responsibilities continue to be managed
(`za_mashatile_dp_in_office_20260830`).

Decision: accepted in part. Mashatile is observed on 4 July 2024 (check defect C2) and on 30 August 2026, with no start
and no end; no primary record was found that the 3 July 2024 oath took place. The Presidency profile's current title
captured on 11 June 2026 is a live page's title and a claim only (C3).

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `za_govza_mandela_inauguration_19940510` | President Nelson Mandela: 1994 Presidential Inauguration (gov.za speech page, dated 10 May 1994) | 47,416 bytes, `a8e8a97a…a35c1f`; capture 2026-06-22 |
| `za_un_spv3379_19940525` | United Nations Security Council, 3379th meeting, provisional verbatim record S/PV.3379 (New York, 25 May 1994) | 145,245 bytes, `c6ba5d28…c23469`; UN Official Document System; PDF pages 1, 2 viewed |
| `za_dfa_mbeki_security_council_statement_19940525` | Statement by Mr. Thabo Mbeki, First Deputy Vice-President of South Africa, at the 3379th meeting of the United Nations Security Council, 25 May 1994 (Department of Foreign Affairs) | 28,919 bytes, `2efd3e35…b37e51`; capture 2004-10-28; located by the check |
| `za_gazette_15792_19940603` | Government Gazette No. 15792 (Pretoria, 3 June 1994): Government Notice No. 1065, Office of the President, Appointment as Ministers and Deputy Ministers | 2,117,040 bytes, `3f3735f0…5868bf`; gazettes.africa archive host; PDF page 1 viewed |
| `za_presidency_profile_former_president_mbeki` | Former President Thabo Mvuyelwa Mbeki (The Presidency profile) | 68,368 bytes, `55478200…b00799`; capture 2025-11-14 |
| `za_infogov_executive_deputy_president_rdp_statement_19960418` | Press statement by the Office of Executive Deputy President Mbeki: Relocation of RDP projects, programmes and institutions, 18 April 1996 | 13,463 bytes, `3983491c…8034e5`; capture 2008-01-27; located by the check |
| `za_mandela_gov_np_withdrawal_statement_19960509` | Statement by President Nelson Mandela on the National Party's withdrawal from the Government of National Unity (GNU), 9 May 1996 | 6,755 bytes, `3f1caf60…7f5a81`; capture 2013-12-10 |
| `za_mandela_gov_new_cabinet_members_statement_19960513` | Statement by President Nelson Mandela on the appointment of new Cabinet members, 13 May 1996 | 5,876 bytes, `6a8fb704…48b2bb`; capture 2013-12-10 |
| `za_gazette_20261_19990702` | Government Gazette No. 20261 (Pretoria, 2 July 1999): General Notice 1392 of 1999, Office of the Presidency | 1,932,684 bytes, `9e8e6237…473be1`; gazettes.africa archive host; PDF page 1 viewed |
| `za_parliament_hansard_na_19990628` | Proceedings of the National Assembly, Monday 28 June 1999 (Hansard NA280699) | 330,752 bytes, `715a3397…86fa1f`; Parliament's document store |
| `za_presidency_profile_former_deputy_president_zuma` | Former Deputy President Jacob Zuma (The Presidency profile) | 83,907 bytes, `f3abdd3e…952dab`; capture 2026-02-15 |
| `za_presidency_history_list_deputy_presidents` | History: 'Deputy Presidents of the Republic' (The Presidency; the deputy-president rows of the capture also cited as za_presidency_history_list) | 64,973 bytes, `bf8b63b0…269bac`; capture 2026-04-15; same response as `za_presidency_history_list` |
| `za_parliament_hansard_joint_sitting_20050614` | Proceedings at Joint Sitting, Tuesday 14 June 2005 (Hansard JS140605) | 41,984 bytes, `f9cbda1a…d311a8`; Parliament's document store; located by the check |
| `za_infogov_mbeki_release_of_zuma_statement_20050614` | T Mbeki: Release of Jacob Zuma from his responsibilities as Deputy President (Statement of the President at the Joint Sitting of Parliament, 14 June 2005) | 18,939 bytes, `19899ee8…aacf80`; capture 2006-04-27 |
| `za_infogov_zuma_statement_on_release_20050614` | J Zuma: Statement on decision by T Mbeki to release him as Deputy President (Statement by the Honourable Jacob Zuma on the decision taken by the President of the Republic, 14 June 2005) | 11,335 bytes, `37beb00d…019d40`; capture 2008-12-01 |
| `za_presidency_statement_npa_decision_20050620` | Media statement on the decision of the prosecuting authority (The Presidency, 20 June 2005) | 24,358 bytes, `7ae6b8a6…15913a`; capture 2007-11-13 |
| `za_dfa_gcis_cabinet_statement_20050622` | Statement on Cabinet Meeting of 22 June 2005 (Government Communications, GCIS) | 17,742 bytes, `c87e98a3…91daf8`; capture 2006-09-24 |
| `za_parliament_hansard_na_20050622` | Proceedings of the National Assembly, Wednesday 22 June 2005 (Hansard NA220605) | 275,968 bytes, `d0d032d2…4d876c`; Parliament's document store; located by the check |
| `za_dfa_presidency_swearing_in_advisory_20050623` | Media Advisory on Swearing in Ceremony of Deputy President, Minister and Deputy Ministers (The Presidency) | 13,420 bytes, `fd7c4047…d7f026`; capture 2006-09-24 |
| `za_presidency_deputy_profile_mlambo_ngcuka_2007` | Deputy President: Profile (Phumzile Mlambo-Ngcuka), The Presidency, captured 8 February 2007 | 25,332 bytes, `6712eb12…9611e9`; capture 2007-02-08 |
| `za_presidency_profile_mlambo_ngcuka_20260219` | Former Deputy President Phumzile Mlambo-Ngcuka (The Presidency profile, captured 19 February 2026) | 65,525 bytes, `0f377b62…30e5e1`; capture 2026-02-19 |
| `za_infogov_presidency_cabinet_resignations_20080923` | Presidency on resignation of members of Cabinet and Deputy Ministers (Statement on the resignation of members of Cabinet and Deputy Ministers, 23 September 2008) | 7,731 bytes, `f5e9cfd6…7ee0fb`; capture 2008-09-25 |
| `za_presidency_latest_news_list_20081014` | Latest News (The Presidency's list of releases), captured 14 October 2008 | 85,540 bytes, `9c5501f2…7236cd`; capture 2008-10-14 |
| `za_parliament_hansard_na_20080925_deputy_president` | Proceedings of the National Assembly, Thursday 25 September 2008 (Hansard NA250908): the Deputy President's designation and the Speaker's resignation | 178,176 bytes, `ad14dad0…2a179f`; Parliament's document store; same response as `za_parliament_hansard_na_20080925` |
| `za_presidency_cabinet_list_20080925_deputy_president` | Members of Cabinet as Announced by the President, Mr. Kgalema Motlanthe, on 25 September 2008 (the Deputy President row) | 33,333 bytes, `14e0bd21…015a9f`; capture 2008-10-02; same response as `za_presidency_cabinet_list_20080925` |
| `za_parliament_hansard_na_20081021` | Proceedings of the National Assembly, Tuesday 21 October 2008 (Hansard NA211008) | 507,904 bytes, `7ee380c4…14cf72`; Parliament's document store; located by the check |
| `za_presidency_deputy_profile_mbete_2009` | Deputy President: Profile (Baleka Mbete), The Presidency, captured 30 April 2009 | 36,104 bytes, `0cfdbc6d…b0ba19`; capture 2009-04-30 |
| `za_presidency_zuma_cabinet_statement_20090510` | Statement by President Jacob Zuma on the appointment of the new cabinet, 10 May 2009 (The Presidency) | 28,712 bytes, `1c3e0630…6df760`; capture 2009-05-18 |
| `za_govza_directory_motlanthe_deputy_president` | Mr Kgalema Petrus Motlanthe (South African Government contact directory): the deputy-presidency line | 35,579 bytes, `88436ae8…c3780e`; capture 2026-06-24; same response as `za_govza_directory_motlanthe` |
| `za_govza_zuma_national_executive_20140525` | President Jacob Zuma announces members of the National Executive (25 May 2014) | 52,362 bytes, `a0dff1bd…eb065a`; capture 2026-07-05 |
| `za_parliament_atc6_20140609` | Announcements, Tablings and Committee Reports No 6-2014, Monday 9 June 2014 (First Session, Fifth Parliament) | 126,866 bytes, `1c4f6177…f87f30`; Parliament's document store; located by the check |
| `za_parliament_na_minutes_20140618` | Minutes of Proceedings of National Assembly No 2-2014, Wednesday 18 June 2014 (First Session, Fifth Parliament) | 102,826 bytes, `52220ece…cb2e79`; Parliament's document store; located by the check |
| `za_parliament_procedural_developments_21_2014` | Procedural Developments in the National Assembly, Issue 21: First Session, Fifth Parliament, May to December 2014 | 355,964 bytes, `9d20ae19…d49ee2`; Parliament's document store; PDF pages 1, 7, 27 viewed |
| `za_presidency_profile_former_deputy_president_ramaphosa` | Former Deputy President Cyril Ramaphosa (The Presidency profile) | 68,591 bytes, `dbf134ba…268f3e`; capture 2025-08-14 |
| `za_govza_ramaphosa_national_executive_changes_20180226` | President Cyril Ramaphosa announces changes to the National Executive (26 Feb 2018) | 56,872 bytes, `b0de5d41…fcb945`; capture 2023-01-29 |
| `za_parliament_na_minutes_20180227` | Minutes of Proceedings of National Assembly No 3-2018, Tuesday 27 February 2018 (Fifth Session, Fifth Parliament) | 238,856 bytes, `2c7ceb41…994695`; Parliament's document store; located by the check |
| `za_govza_presidency_swearing_in_advisory_20180227` | Presidency hosts swearing in ceremony of new National Executive members, 27 Feb (media advisory) | 44,521 bytes, `992ae239…9d5cb8`; capture 2018-02-27 |
| `za_parliament_atc21_20180305` | Announcements, Tablings and Committee Reports No 21-2018, Monday 5 March 2018 (Fifth Session, Fifth Parliament) | 97,958 bytes, `fcb24b76…54939c`; Parliament's document store; located by the check |
| `za_presidency_profile_mabuza` | Former Deputy President David Mabuza (The Presidency profile) | 66,591 bytes, `c4503a0e…16fb3b`; capture 2026-08-08 |
| `za_govza_ramaphosa_national_executive_20190529` | President Cyril Ramaphosa: Cabinet announcement - statement on the appointment of members of the National Executive (29 May 2019) | 51,202 bytes, `acd3350b…62942d`; capture 2026-06-26 |
| `za_govza_presidency_swearing_in_advisory_20190530` | Chief Justice Mogoeng Mogoeng swears in new Ministers and Deputy Ministers, 30 May (media advisory) | 54,267 bytes, `0b891ff5…9ddf44`; capture 2019-05-30 |
| `za_parliament_sixth_parliament_composition_20190606` | Current Composition of the Newly Sworn-In 6th Parliament (Parliament release, 6 June 2019) | 56,431 bytes, `552b74f7…983900`; capture 2021-06-17 |
| `za_parliament_atc6_20190612` | Announcements, Tablings and Committee Reports No 6-2019, Wednesday 12 June 2019 (First Session, Sixth Parliament) | 67,674 bytes, `da17455c…a9160d`; Parliament's document store; located by the check |
| `za_govza_ramaphosa_sona_reply_20230216` | President Cyril Ramaphosa: Reply to Debate on State of the Nation Address (16 Feb 2023) | 68,292 bytes, `ce65860b…aa314e`; capture 2026-06-21 |
| `za_parliament_atc25_20230301` | Announcements, Tablings and Committee Reports No 25-2023, Wednesday 1 March 2023 (Fifth Session, Sixth Parliament) | 201,134 bytes, `050a78a2…830059`; Parliament's document store; located by the check |
| `za_govza_presidency_mabuza_resignation_20230301` | Presidency on resignation of Deputy President David Mabuza (1 Mar 2023) | 54,351 bytes, `4fd05f78…b8cefc`; capture 2023-03-02 (08:46:17, the earliest capture, served uncompressed) |
| `za_parliament_mabuza_assembly_resignation_20230301` | Media Release: Resignation of Deputy President Mabuza as MP (1 March 2023) | 59,685 bytes, `b0503c59…cc2ee3`; capture 2025-12-07 |
| `za_govza_directory_mabuza` | David Dabede Mabuza, Mr (South African Government contact directory) | 34,408 bytes, `3e10b25f…226ba1`; capture 2026-06-22 |
| `za_govza_ramaphosa_new_national_executive_20230306` | President Cyril Ramaphosa: New members of National Executive (6 Mar 2023) | 50,649 bytes, `740037e6…ceafb6`; capture 2026-06-27 |
| `za_govza_presidency_swearing_in_advisory_20230307` | President Cyril Ramaphosa officiates swearing-in ceremony of new members of National Executive, 7 Mar (media advisory) | 43,590 bytes, `e70113ce…809667`; capture 2026-06-27 |
| `za_parliament_atc31_20230309` | Announcements, Tablings and Committee Reports No 31-2023, Thursday 9 March 2023 (Fifth Session, Sixth Parliament) | 1,759,642 bytes, `41717dbf…09111e`; Parliament's document store; located by the check; PDF page 3 viewed |
| `za_presidency_deputy_president_office_appointments_20230411` | Appointment of key positions in the Office of the Deputy President (The Presidency, 11 April 2023) | 13,746 bytes, `f215d57a…fdb15c`, gzip as served (decoded 63,123 bytes, `68862c8b…312b94`); capture 2025-04-05 |
| `za_presidency_profile_mashatile` | Deputy President Paul Mashatile (The Presidency profile) | 66,973 bytes, `da02a48f…845476`; capture 2026-06-11 |
| `za_presidency_national_executive_20240630` | Statement by President Cyril Ramaphosa on the appointment of Members of the National Executive (30 June 2024) | 72,987 bytes, `712b0081…7062f1`; capture 2026-06-10 |
| `za_govza_presidency_swearing_in_advisory_20240701` | Swearing-in ceremony of new Deputy President, Cabinet Ministers and Deputy Ministers as members of National Executive, 3 Jul (media advisory) | 46,239 bytes, `eb8a28e8…7c56bd`; capture 2024-07-03 |
| `za_parliament_atc4_20240705` | Announcements, Tablings and Committee Reports No 4-2024, Friday 5 July 2024 (First Session, Seventh Parliament) | 155,962 bytes, `f4a050d3…0cb1d1`; Parliament's document store; located by the check |
| `za_presidency_opening_of_parliament_address_20240718` | Opening of Parliament Address by President Cyril Ramaphosa at the Cape Town City Hall (18 July 2024) | 91,572 bytes, `175b965a…cac7ae`; capture 2024-07-23 |
| `za_govza_directory_mashatile` | Paul Shipokosa Mashatile, Mr (South African Government contact directory) | 36,842 bytes, `e6066074…e805d7`; capture 2026-06-21 |
| `za_govza_deputy_president_recovering_20260830` | Deputy President Paul Mashatile recovering well while work continues in the office (30 Aug 2026) | 13,095 bytes, `ddc3525b…d0f69e`, gzip as served (decoded 43,514 bytes, `ee919811…bc927e`); capture 2026-08-31 |

Of the 59 sources, 42 are raw Internet Archive captures (`id_` form) made between 2004 and 31 August 2026, all before
the cutoff; 14 are static files in Parliament's document store (Hansard `.doc` files, Announcements, Tablings and
Committee Reports, Minutes and a Procedural Developments digest, all with Last-Modified 2 March 2026); two are scanned
Government Gazettes on the gazettes.africa archive host; and one is the UN Official Document System's PDF of S/PV.3379.
Thirteen were located by the checks (marked above), and four are separate records of responses CLAUDE-C01-09 already
cites (see [Responses already cited by CLAUDE-C01-09](#responses-already-cited-by-claude-c01-09)). Each archived
source records the capture URL as `url` and the original address as `original_url` (without `:80`), and its extract
records the capture time; the other hosts have no `original_url`. No live page is used: the live Presidency and gov.za
pages are Drupal pages with form tokens and rotating blocks, and the gov.za captures made in 2026 carry a site-wide
keywords tag naming the 2026 office-holders, which is template text and never an attestation.

Each new source has a derived factual extract under [sources/](sources/) in the packet's existing format: one row per
claim, keyed by `claim_id`, with `observation_id` `za_presidency`, `review_observation` (ZA-DP-01..10), `role_id`
`za_deputy_president`, `holder_name` (normalised: "F. W. de Klerk" for "FW de Klerk", "Jacob Zuma" for "Mr J. G.
Zuma", "Baleka Mbete" for "Ms B Mbete", "Kgalema Motlanthe" for "Kgalema Petros/Petrus Motlanthe", "Paul Mashatile" for
"Mr Shipokosa Paulus Mashatile"; null on the eight rows whose source names nobody), `role_title`, `event_kind` and
`attested_on`, plus the claim's text and locator and, for undated rows, the printed words as `printed_range` (the Mashatile profile's current-title row prints no date, so its `printed_range` records the capture instead, "as captured on 11 June 2026"). The
extract's checksum is in the packet, separate from the response hash. Original pages, PDFs and renders are not checked
in, and no photograph, seal or signature is republished (the scanned letter in No 31 of 2023 is read, not copied).

## Response identities and stability checks

Codex re-downloads every recorded response and compares its byte count and SHA-256, so each extract carries the recipe
the identity depends on (`fetch_recipe`) and its `source_response_content_encoding`:

- Internet Archive captures: fetch the recorded `url` exactly, with no `Accept-Encoding` header (or `Accept-Encoding:
  identity`) and no automatic decoding (`curl -s -o FILE URL`, not `--compressed`). Served that way, 40 captures come
  back uncompressed, and their recorded identity is the uncompressed body. The part A check showed that a request
  accepting gzip receives a different body (2,666 bytes for the 9 May 1996 statement), and the part B check the same
  for the 2026 Mlambo-Ngcuka profile (17,292 bytes).
- Two captures are stored gzip-encoded and are served with `Content-Encoding: gzip` even to an identity request: the
  Presidency's statement of 11 April 2023 and the statement of 30 August 2026 (each the only pre-cutoff capture of its
  page). Following the CLAUDE-C01-16 convention, their recorded identity is the gzip body as served, and
  `decoded_response_bytes` and `decoded_response_sha256` give the decoded HTML (what a decoding client returns);
  neither is described as an uncompressed body (check part C integration note). For the statement of 1 March 2023 the
  earliest capture (20230302084617), served uncompressed, is used instead of the gzip-encoded capture made four
  seconds later; its body is byte-identical to that capture's decoded body.
- Parliament's files are static (ETag size part equals the byte count); use GET, since the server rejects HEAD.
- The gazettes.africa PDFs are static files behind Cloudflare: plain requests were answered from Cloudflare's cache and
  cache-busting requests from the origin, with the same bytes; the ETag equals the MD5 of the bytes.
- The UN PDF is static (Last-Modified 5 April 2023); plain and cache-busting requests matched.

Stability: every response was downloaded at least twice, at least 30 minutes apart, with the same byte count and
SHA-256. Part A: the researcher downloaded each source at 12:03-12:57Z and again at 12:57-13:00Z and 13:28-13:30Z, and
the check again at 13:42-13:43Z and 14:13Z, with cache-busting requests for the four non-archive files. Part B: the
researcher at 11:59-13:18Z and again at 13:43-13:50Z; the check at 13:52Z (and the 25 September 2008 Hansard with a
cache-busting query). Part C: the researcher at 12:21-12:33Z and again at 13:05-13:08Z; the check at 13:14-13:18Z. The
records located by the checks were downloaded twice by the checks (part B's Parliament files from 14:01Z to 14:47Z;
part C's from 13:19-13:31Z and at 14:02Z; part A's two captures about 10 minutes apart), and each was downloaded again
for this packet at 14:55Z. All 59 responses were then downloaded once more for this packet at 15:28-15:31Z (all times
UTC, 25 September 2026), more than 30 minutes after every earlier download; every one matched, and the three
gzip-encoded captures also matched their decoded size and SHA-256. The independent source verification then
replaced the 1 March 2023 statement's gzip-encoded capture with the earlier, uncompressed capture 20230302084617
(54,351 bytes, `4fd05f78…b8cefc`), downloaded at 15:54Z, 15:57Z and 16:25Z with the same identity. The bodies were
deleted after hashing.

Per-request traps: no recorded URL is a search, listing API, generated PDF or cache-busting address, and the test
rejects those shapes. Parliament's paper index (`docsjson`) and the gov.za and Presidency site searches were used only
to find documents; the index grows over time and answers HTTP 500 unless called with the paging parameters the site
sends. The Presidency's "Latest News" list of 2008 is used only as a fixed capture of 14 October 2008. The live Drupal
pages are replaced by pre-cutoff captures. Gazette No. 20261 and the UN record are static PDFs, not regenerated per
request.

## Leads not imported

- News reports (leads only): https://www.sanews.gov.za/south-africa/new-deputy-president-ministers-sworn (Ramaphosa
  sworn in on 26 May 2014), https://www.sanews.gov.za/south-africa/david-mabuza-sworn-mp,
  https://www.sanews.gov.za/south-africa/new-ministers-ready-work-after-swearing (7 March 2023),
  https://www.sanews.gov.za/south-africa/now-work-begins-dp-ministers-and-deputy-ministers-sworn (the only report found
  that the 3 July 2024 oath took place) and https://www.news24.com/southafrica/news/live-cabinetreshuffle-what-next-for-ramaphosas-new-cabinet-20180227
  (Mabuza sworn in on 27 February 2018).
- Encyclopaedia: https://en.wikipedia.org/wiki/Deputy_President_of_South_Africa.
- Duplicates of imported statements: the Department of Foreign Affairs copies of the 14 June 2005 statements
  (`mbek0614.htm`, `zuma0614.htm`); the government information site's copies of the 10 May 2009 statement and media
  briefing (`09051016451001`, `09051017051001`); the GCIS copy of the 25 May 2014 announcement (gcis.gov.za); the
  DIRCO republication of the 1 March 2023 statement (https://dirco.gov.za/statement-on-resignation-of-deputy-president-david-mabuza/);
  the Presidency's own live copies of the 2014, 2018, 2023 and 2024 statements and advisories (dynamic pages without a
  pre-cutoff capture); the later gov.za addresses of the 2018 statement and advisory.
- The Presidency's copy of the 30 August 2026 statement, capture https://web.archive.org/web/20260831043754id_/https://www.thepresidency.gov.za/deputy-president-mashatile-recovering-well-while-work-continues-office
  (13,495 bytes gzip-encoded, decoded 62,368 bytes): the issuer's copy of the imported statement; it prints only the
  surname, so the gov.za copy, which names him, is kept (a missing record from check C, declined).
- The Presidency's "Former Principals" list (https://www.thepresidency.gov.za/former-principals): "2014 - February 15,
  2018" for Ramaphosa, a retrospective span adding nothing to the profiles.
- Parliament papers located by the checks and declined: NA Hansard of 14 June 2005, members' statements after the Joint
  Sitting (https://www.parliament.gov.za/storage/app/media/Docs/hansard/50604_1.doc; members' remarks with no effective
  day); NCOP Hansard of 22 June 2005 (https://www.parliament.gov.za/storage/app/media/Docs/hansard/50510_1.doc; a
  member's remark that "the country now has a new Deputy President", the same day as the Assembly's formal
  resolution); NA Minutes No 6 of 2018 (https://www.parliament.gov.za/storage/app/media/Docs/min/1154bc2c-4ff9-4d4b-b3ab-2d66d05f005a.pdf;
  the House announcement of the letter recorded in ATC No 21 of 2018, naming no one); ATC No 5 of 2024
  (https://www.parliament.gov.za/storage/app/media/Docs/atc/28af2ecb-a748-4279-b162-f8dbfab648c9.pdf; a counsellor
  request from "the Deputy President", naming no one). Each was downloaded twice by the checks with a stable identity.
- Procedural Developments Issue 30 (https://www.parliament.gov.za/storage/app/media/NA-Procedural-Devs/30.pdf): prints
  2018 events under a 2022 label; ATC No 21 of 2018 is imported instead.
- NCOP Internal Question Paper No 32-2026 (`quest_inte/01vchl7hbraac72m6ehrd2fum3hfwcv5x3.pdf`): questions to "the
  Deputy President", naming no one.
- Acting service: Gazette No. 17332 of 12 July 1996 (https://archive.gazettes.africa/archive/za/1996/za-government-gazette-regulation-gazette-dated-1996-07-12-no-17332.pdf),
  proclamations signed on 11 July 1996 by "T. M. MBEKI Acting President", and the Presidency advisory styling Mashatile
  "Acting President" (health compact): a Deputy President acting as President is a claim only, outside ZA-DP-01..10,
  never a holder.
- Other 1994-1999 records: the State of the Nation Address of 24 May 1994 (`940524_sona`; salutes the "Deputy
  Presidents" unnamed); Hansard of 29 June 1999 (`92089_1.doc`) and the Joint Sitting of 25 June 1999 (`90959_1.doc`);
  the Presidency statement of 23 July 1999 (`990723444p1003`); the government release of 10 May 1996 quoting Mbeki
  (`960513_0w531`); the 2021 eulogy for "former Deputy President FW De Klerk" (`eulogy-former-deputy-president`); the
  ANC's statement on the National Party's withdrawal (anc.org.za, a party's statement on a state office); Proclamation
  No. 98 of 1994 (already cited by CLAUDE-C01-09 for the State President).
- Other 2008 records: the Presidency advisory of 26 September 2008 (`08092610451002`; ministers only); the Presidency
  release naming "Deputy President Baleka Mbethe" (`pr10081111`; a date conflict); the profile page captured on 30
  September 2008 (`20080930232338`; her Speaker's biography); the National Treasury statement of 23 September 2008
  (`08092314451001`); remarks in the existing Hansards of 23 and 25 September 2008.
- Live gov.za searches (for example `https://www.gov.za/search?search_query=Mlambo-Ngcuka`): growing result lists,
  used for discovery only.

## Sources attempted

- Internet Archive: connection refusals, HTTP 429, 500 and 504 and "temporarily offline" pages for all three
  researchers and the checks; every capture used was eventually retrieved and checked for HTTP 200. A rate-limited
  body was discarded.
- Parliament's digital Hansard begins with the sitting of 14 June 1999; no 1994 or 1996 debates or minutes are held.
  Parliament's paper index was not blocked (defects B1 and C8): it answers HTTP 500 only when called without the paging
  parameters the site sends.
- Gazettes: No. 15771 of 3 June 1994 is a Regulation Gazette at
  https://archive.gazettes.africa/archive/za/1994/za-government-gazette-regulation-gazette-dated-1994-06-03-no-15771.pdf
  (11,433,468 bytes, `f199cc8c…90cc51`); the check searched all 32 pages without a relevant hit (the part A 404 came
  from the wrong address kind). No separate file exists for No. 20262, so the rest of Notice 1392 of 1999 cannot be
  recovered from this archive. The gazettes.africa year listings return 404 and the landing pages sit behind a
  Cloudflare challenge, which was not attempted. Large 1994 and 1996 issues listed in the part A dossier were not
  searched.
- No capture exists for the info.gov.za original of the 9 May 1996 statement, the President's 17 June 1999 Cabinet
  announcement, any June 2005 Presidency or GCIS release on the appointment or oath, the Presidency's own release of
  Mlambo-Ngcuka's resignation (`deputy/pr/2008/pr09231034.htm`), its releases of 25 and 26 September 2008, or pre-cutoff
  copies on thepresidency.gov.za of the 2014, 2018 and 2023 statements (live pages are dynamic).
- Parliament's Procedural Developments Issues 31 and 33 to 36 return 404; Issues 26 and 32 were searched without a
  Deputy President appointment entry; ATC No 38 and No 91 of 2023 and NA Minutes No 9 of 2023 have no Mabuza or
  Mashatile entry.
- The Presidency's `former-deputy-presidents` and `former-deputy-president-baleka-mbete` addresses return 404.
- A redirecting capture of the 26 February 2018 statement (20230308192054) leads to the recorded capture
  20230129215710; a gzip-encoded 2026 capture of the 30 June 2024 statement was replaced by an uncompressed one.

## Checker defects

Part A (nine defects):

| # | Defect | Outcome |
|---|---|---|
| A1 | The 9 May 1996 thanks claim paraphrased "hopes they will keep working together" | **Applied**: reworded to the statement's "confident that we shall continue to work together" |
| A2 | The gov.za inauguration page's `published_date` was the speech's dateline | **Applied**: null, with the dateline explained in the scope note |
| A3 | The 28 June 1999 Hansard titled "(unrevised)" | **Applied**: dropped; the scope note says the document is not so labelled and has "col 000" placeholders |
| A4 | Gazette No. 15771 recorded as a 404 | **Applied**: recorded under Sources attempted with the correct Regulation Gazette address and the check's result |
| A5 | Mbeki's holder rested on the UN record alone | **Applied**: the Department of Foreign Affairs' copy of the same day added as a South African holder claim and his office's statement of 18 April 1996 as a continuation claim; the holder stays 25 May 1994, citing both same-day records |
| A6 | Row holder names did not match the holders | **Applied**: normalised names on every row; printed forms kept in the text |
| A7 | Adding rows to the CLAUDE-C01-09 history-list extract would break its pinned tests | **Applied**: a separate source record and extract for the same response |
| A8 | The Zuma list row was shared with part B | **Applied**: one record carries all five list rows; part B's duplicate id is not used |
| A9 | Extract fields missing (`claim_id`, `role_title`, `observation_id`, `review_observation`) | **Applied** |

Part B (eleven defects):

| # | Defect | Outcome |
|---|---|---|
| B1 | Parliament's index reported as blocked | **Applied**: the Joint Sitting Hansard of 14 June 2005 and the NA Hansard of 22 June 2005 imported; the blocked-source entry and unresolved item removed |
| B2 | The release statement rested on the web copy only | **Applied**: the Joint Sitting Hansard imported as a separate claim beside the government information site's copy |
| B3 | Mlambo-Ngcuka's `from` rested on a self-contradicting profile | **Applied**: `attested_on` 2005-06-22, `from` null, citing the profile (event kind `assumption_of_office`) and the Assembly's resolution; the uncertainty names the 23 June 14h00 oath schedule and the profile's 21 June date |
| B4 | The 21 June profile claim was weighed only against the Cabinet statement | **Applied**: its uncertainty cites the Assembly's resolution of 22 June 2005 |
| B5 | Holders cited announcements, schedules and remarks, and lacked packet fields | **Applied**: each holder cites only same-day dating claims and has `sources`, `claim_ids`, `note` and `uncertainty` |
| B6 | Mbete dated by designation records of 25 September 2008 | **Applied**: option (b), dated 21 October 2008 by the Assembly record; the members' remarks claim renamed `za_mbete_designation_congratulated_in_house_20080925` and quotes "nomination" and "to become the Deputy President" |
| B7 | `attested_period` with an invented day | **Applied**: no structured date; `printed_range` "since September 2008" |
| B8 | Printed names used as `holder_name` | **Applied** |
| B9 | Editing four CLAUDE-C01-09 extracts | **Applied**: four separate source records with new extracts; the CLAUDE-C01-09 extracts and their tests' exact sets are untouched |
| B10 | Non-house event kinds | **Applied**: `oath_scheduled_prospective` and `assumption_of_office` |
| B11 | The gov.za search lead is a growing list | **Applied**: kept as a discovery lead only (no change needed) |

Part C (ten defects):

| # | Defect | Outcome |
|---|---|---|
| C1 | Ramaphosa dated by the 25 May 2014 announcement | **Applied**: dated 30 May 2014 by the President's letter (ATC No 6 of 2014, imported); the 25 May claims are never-holder claims |
| C2 | Mashatile 2024 dated by the 30 June announcement | **Applied**: dated 4 July 2024 by the President's letter (ATC No 4 of 2024, imported) |
| C3 | The 2026 holder rested on a live page's current title | **Applied**: that claim is undated, event kind `current_title_at_capture`, never a holder; the scope note says the page was submitted on 4 December 2023 |
| C4 | Receipt and effect of the Assembly resignation collapsed | **Applied**: renamed `za_mabuza_assembly_resignation_received_20230228`; ATC No 25 of 2023 imported with the letter's "as of 28 February 2023" (Assembly seat only) |
| C5 | The 1 March 2023 statement labelled a vacancy | **Applied**: renamed `za_deputy_president_successor_to_be_announced_20230301`; the summaries no longer say the post was vacant |
| C6 | Later references to the 16 February announcement under the same event kind | **Applied**: `step_down_request_reference_retrospective`, renamed, stale sentence removed |
| C7 | The 2019 ministers-only advisory filed as a Deputy President oath schedule | **Applied**: renamed `za_ministers_swearing_in_scheduled_20190530`, context only; ATC No 6 of 2019 imported |
| C8 | Parliament's listing reported as client-side only | **Applied**: corrected under Sources attempted |
| C9 | Medical details in the 30 August 2026 claim | **Applied**: trimmed to what attests office |
| C10 | Holders' supporting claims mixed dating and non-dating claims | **Applied**: holders cite only dating claims; the rest are never-holder claims pinned in the test |

Missing primary records found by the checks: imported are, from part A, the Office of the Executive Deputy President's
statement of 18 April 1996 and the Department of Foreign Affairs' copy of the 25 May 1994 statement; from part B, the
Joint Sitting Hansard of 14 June 2005, the NA Hansard of 22 June 2005 and the NA Hansard of 21 October 2008; from part C,
ATC No 6 of 2014, NA Minutes No 2 of 2014, NA Minutes No 3 of 2018, ATC No 21 of 2018, ATC No 6 of 2019, ATC No 25 of
2023, ATC No 31 of 2023 and ATC No 4 of 2024. Declined, for the reasons under [Leads not imported](#leads-not-imported):
the NA Hansard of 14 June 2005 and the NCOP Hansard of 22 June 2005 (members' remarks), NA Minutes No 6 of 2018 and ATC
No 5 of 2024 (naming no one) and the Presidency's copy of the 30 August 2026 statement (a duplicate printing the
surname only).

Other changes made to fit the packet's rules rather than a numbered defect:

- In-office attestations that do not date a holder are labelled `in_office_continuation_attestation`, and every
  holder cites only claims of its own day, as in CLAUDE-C01-16.
- The Mabuza profile's 2019 claim quotes the profile exactly ("again on Thursday, 30 May 2019 for the 6th democratic
  Administration").
- Mbete's cabinet-list and designation claims, which the part B dossier proposed as holder claims, are never-holder
  claims (`cabinet_list_as_announced`, `members_remarks_on_designation`).

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SouthAfrica-DP-002`: the designation, appointment or oath of the Executive Deputy Presidents in May 1994 (the
  unsearched large 1994 Gazettes; any Presidency or Constitutional Assembly record) and the day de Klerk's office ended
  in 1996 (the larger mid-1996 Gazettes; any National Party record).
- `C01-SouthAfrica-DP-003`: the effective day of Zuma's release in June 2005 and whether Mlambo-Ngcuka's scheduled oath
  of 23 June 2005 took place; the Presidency's uncaptured release of her 2008 resignation.
- `C01-SouthAfrica-DP-004`: primary records of the oaths of Mbete (2008), Ramaphosa (2014), Mabuza (2019) and Mashatile
  (2024), and of the day Ramaphosa's and Mabuza's terms ended.
- `C01-SouthAfrica-DP-005`: acting service as President by Deputy Presidents (for example Gazette No. 17332 of 1996), as
  claims only on `za_president_election`, never holders.

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** the branch `claude/c01-za-21` is stacked on `claude/c01-za-16` (`2df4a0a6`), which is
  stacked on `claude/c01-za-09` and based on `ffe54b02` (`codex/campaign-certification`); neither stacked packet is
  integrated yet, so merge CLAUDE-C01-09, then CLAUDE-C01-16, then this packet. Before this work
  `origin/claude/c01-za-16` was fetched and had no commit missing from this branch, so no merge was needed. The claim
  commit `e2d1a1ec` holds only the handoff. `south-africa.json` is shared only with the two stacked packets.
- Files touched: the handoff, this report, `south-africa.json`, 59 new extracts
  `docs/campaign-certification/C01/research/sources/south-africa-*-facts.json`, the new
  `tools/avatars/test_south_africa_deputy_presidents_c01_21.py`, and pinned values in
  `tools/avatars/test_south_africa_research_s10h.py`, `tools/avatars/test_south_africa_heads_of_state_c01_09.py` and
  `tools/avatars/test_south_africa_anc_presidents_c01_16.py`. No existing extract is edited; the four responses
  CLAUDE-C01-09 already cites have separate records.
- `research-index.json` is regenerated in a **separate commit** and is the only file this packet shares with other
  pending packets; regenerate it when integrating. New totals: 324 sources and 1,995 claims (previously 265 and 1,913). South Africa role observations rise
  from 6 to 7 and source claims from 260 to 342; organization, institution, packet and batch counts are unchanged,
  `mapping_pending` stays 53 and South Africa keeps six open batches.
- Pinned tests, none loosened and no assertion removed:
  - `test_south_africa_research_s10h.py`: counts (entries, sources, claims, roles) are now (53, 168, 342, 7); the
    presidency's roles are pinned exactly as the President, the State President and the Deputy President
    (`institutional_office`); the exact presidency holder list is extended by the twelve Deputy President holders;
    response pins, the source list, hosts (adding `documents.un.org`) and access dates (2026-09-25 for this packet) and
    the undated claim count (7 + 31 + 13) are extended exactly.
  - `test_south_africa_heads_of_state_c01_09.py`: the exact role list of `za_presidency` and the exact source list are
    extended; the "State President role removed" mutation now removes that role by position (`pop(1)`), because the
    Deputy President role is last.
  - `test_south_africa_anc_presidents_c01_16.py`: the presidency role list and holder count (23), the entry and role
    counts (53, 7), the source order (its 54 sources, then this packet's 59) and the index totals (7, 342) are pinned.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and
  the sparse checkout was not widened.
- New extract fields beyond CLAUDE-C01-16's: none; rows reuse `printed_range` and `review_observation`. The extract
  `bounded_scope` of the four re-imported records names the CLAUDE-C01-09 extract that derives the other rows.
- New event kinds: `appointment_effective` (the one stated start), `appointment_noted_by_resolution`,
  `in_office_continuation_attestation` and the never-holder kinds listed in the test's `EVENTS` map.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; the
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
