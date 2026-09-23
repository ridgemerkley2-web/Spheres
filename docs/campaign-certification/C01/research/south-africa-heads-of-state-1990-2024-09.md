# South African heads of state 09: holders and transitions, 1990-2024

Packet: **CLAUDE-C01-09**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-za-09`, based on `codex/campaign-certification`
at `ffe54b02`; claim commit `0e6bbf94`. Research access: 22 September 2026 (local). The historical cutoff stays
**7 September 2026**.

This packet reviews ten observations about the existing `za_presidency` institution in
[south-africa.json](south-africa.json), from 1 January 1990 to the National Assembly election of 14 June 2024 that
the packet already holds (`za_ramaphosa_president_elect_20240614`): the State President when the period opens, the
1994 transition, each later election by the National Assembly and the oath or inauguration that followed, the 2008
and 2018 resignations, and the 2024 oath. It adds 50 sources and 78 claims, one role (`za_state_president`, the
State President under the 1983 Constitution) with one holder observation, nine holder observations on the existing
`za_president_election` role (the existing 14 June 2024 holder is unchanged and keeps its place in date order), two
role scope notes and coverage notes on `za_presidency` and the packet. It adds no organization, institution, game
mapping, lifespan, portrait or avatar. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1989-1999; B: 2004-2009; C: 2014-2024), and each part was checked
independently before this packet was written. Every checker defect is applied or explained below (see
[Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| ZA-HOS-01 | Who was head of state when the period opens (State President, 1983 Constitution)? | **Accepted in part:** F. W. de Klerk, observed in office on 2 Feb 1990 (Gazette); acting from 15 Aug 1989 and substantive by 28 Sep 1989 (Gazettes); his 1989 election and oath not found; no start or end |
| ZA-HOS-02 | 1994: the National Assembly's election, the oath and inauguration, and the State President's end | **Accepted in part:** nomination in the Assembly on 9 May 1994 at the sitting the Chief Justice fixed for the election (vote record not found); oath and inauguration 10 May 1994; Mandela observed 10 May 1994; de Klerk's end not stated (last signature 9 May 1994) |
| ZA-HOS-03 | 1999: election, inauguration and the outgoing President's end | **Accepted in part:** elected 14 Jun 1999; inaugurated 16 Jun 1999; Mandela's end 16 Jun 1999 stated by Mandela; Mbeki observed 16 Jun 1999, no oath or assumption record, so no start |
| ZA-HOS-04 | 2004: re-election, oath and inauguration | **Accepted:** elected unopposed 23 Apr 2004 (Hansard); oath 27 Apr 2004 (his own letter); inauguration 27 Apr 2004; observation dated 27 Apr 2004 |
| ZA-HOS-05 | 2008: resignation (announcement, tender, effect), the successor's election and oath | **Accepted in part:** intention 20 Sep, address and tender 21 Sep, letter read 22 Sep, resolution 23 Sep fixing effect on 25 Sep 2008 (Mbeki's end); Motlanthe elected 269-50 and sworn in 25 Sep 2008; time of effect, acting service and oath officiant unresolved |
| ZA-HOS-06 | 2009: election, oath and inauguration; Motlanthe's end | **Accepted in part:** elected 277-47 on 6 May 2009; oath 9 May 2009; Motlanthe's end stated only by retrospective span sources, not used |
| ZA-HOS-07 | 2014: re-election and assumption of the second term | **Accepted:** elected unopposed 21 May 2014; assumed the second term 24 May 2014 (the one stated start); inauguration 24 May 2014 |
| ZA-HOS-08 | 2018: resignation, acting service, the successor's election and oath | **Accepted:** President Act No. 24, resignation with immediate effect, 14 Feb 2018 (Zuma's end); receipt 15 Feb; Ramaphosa Acting President 15 Feb (claim only); elected unopposed and sworn in 15 Feb 2018 |
| ZA-HOS-09 | 2019: election and oath | **Accepted:** elected unopposed 22 May 2019; oath 25 May 2019; no end of the 2018 observation |
| ZA-HOS-10 | 2024: oath after the existing 14 June 2024 election claim | **Accepted:** oath text and inauguration address dated 19 Jun 2024; the existing election claim and holder unchanged |

The resulting holder observations, in date order:

| Role | Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|---|
| `za_state_president` | F. W. de Klerk | 1990-02-02 | null | null | Gazette 12286: signs as State President |
| `za_president_election` | Nelson Mandela | 1994-05-10 | null | 1999-06-16 | oath (archival copy; 2009 Presidency address); his own statement of 16 June 1999 |
| `za_president_election` | Thabo Mbeki | 1999-06-16 | null | null | styled President as host of the farewell banquet |
| `za_president_election` | Thabo Mbeki | 2004-04-27 | null | 2008-09-25 | oath stated in his 2008 letter; Assembly resolution fixing the effect of his resignation |
| `za_president_election` | Kgalema Motlanthe | 2008-09-25 | null | null | sworn in (Presidency profile); Hansard recess; Cabinet list |
| `za_president_election` | Jacob Zuma | 2009-05-09 | null | null | oath stated in his inauguration address |
| `za_president_election` | Jacob Zuma | null | 2014-05-24 | 2018-02-14 | "assumed his second term" (Presidency); President Act No. 24 |
| `za_president_election` | Cyril Ramaphosa | 2018-02-15 | null | null | sworn in that afternoon (Table Division; Presidency) |
| `za_president_election` | Cyril Ramaphosa | 2019-05-25 | null | null | "having just taken the oath" (Presidency address) |
| `za_president_election` | Cyril Ramaphosa (existing) | 2024-06-14 | null | null | unchanged: election as President-elect |
| `za_president_election` | Cyril Ramaphosa | 2024-06-19 | null | null | oath text dated 19 June 2024 (Presidency) |

### How a start and an end are decided

All three checks raised the same question: is the day of an oath or swearing-in a start? This packet applies one
rule to all three parts. A holder has `from` only where a source states the day office was assumed or took effect,
and `until` only where a source states the day a resignation took effect or a term ended. An oath, swearing-in or
inauguration statement dates a holder observation (`attested_on`) but is never a start, as CLAUDE-C01-08 recorded an
oath as "not a start"; relying on the constitutional sequence (oath, then office) would take a date from procedure.
So only Zuma's 2014 term has a start ("assumed his second term in office ... on 24 May 2014"), and the three ends are
Mandela's own statement of 16 June 1999, the Assembly's resolution that Mbeki's resignation takes effect on
25 September 2008, and President Act No. 24 of 14 February 2018 ("with immediate effect"). The existing
`test_south_africa_research_s10h.py` rule that no South African holder has an interval is replaced by an exact
pinned list of every presidency holder, and the organization holders keep the null-only rule.

Retrospective span lists are recorded as claims, with no structured date, and are never boundaries. The Presidency's
history list starts de Klerk on the day he became Acting State President (15 August 1989) and ends Mbeki on
24 September 2008, a day before the effect the Assembly resolved. The government's contact directory dates Mbeki's
presidency "from 14 June 1999", his election day, two days before Mandela says his own status changed (read live on
22 September 2026; see [Leads not imported](#leads-not-imported)). Both give 9 May 2009 for Motlanthe's end, which is
therefore not set: no event record states it, and Zuma's oath that day is not used as it.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Event | Claim or field |
|---|---|---|
| 15 Aug 1989 | Botha vacates the office; de Klerk designated, sworn in and assumes office as Acting State President | `za_botha_vacated_state_presidency_19890815`, `za_de_klerk_acting_state_president_designated_19890815` (claims, not holders) |
| 13 Sep 1989 | De Klerk signs as Acting State President | `za_de_klerk_acting_state_president_proclamation_167_19890913` |
| 20 Sep 1989 | Foundation transcript of his address "following his inauguration" | `za_de_klerk_post_inauguration_address_19890920` (context) |
| 28 Sep 1989 | De Klerk signs as State President | `za_de_klerk_state_president_proclamation_177_19890928` |
| 2 Feb 1990 | De Klerk signs as State President | `za_de_klerk_state_president_proclamation_r16_19900202`; de Klerk `attested_on` |
| 4 May 1994 | Chief Justice fixes 9 May 1994, 11:00 for the nomination and election of a President | `za_cj_fixes_president_election_sitting_19940504` |
| 9 May 1994 | De Klerk signs as State President; Mandela nominated in the National Assembly | `za_de_klerk_state_president_proclamation_98_19940509`, `za_mandela_na_nomination_19940509` |
| 10 May 1994 | Mandela's oath; inauguration address | `za_mandela_oath_of_office_19940510`, `za_mandela_oath_recalled_zuma_address_20090509`, `za_mandela_inauguration_ceremony_19940510`; Mandela `attested_on` |
| 8 Jun 1999 | Constitutional Court President fixes 14 June 1999, 14:30 for the election | `za_ccpresident_fixes_president_election_sitting_19990608` |
| 14 Jun 1999 | Mbeki elected (acceptance speech, index, later reference) | `za_mbeki_election_acceptance_speech_19990614`, `za_dfa_index_mbeki_election_speech_19990614`, `za_mbeki_na_convened_elected_president_19990614` |
| Jun 1999 (month) | Mbeki takes the oath "In June 1999" | `za_mbeki_oath_june_1999_zuma_address_20090509` (no structured date) |
| 16 Jun 1999 | Inauguration; Mandela's new status "since this morning"; Mbeki hosts the banquet as President | `za_mbeki_inauguration_ceremony_19990616`, `za_dfa_index_mbeki_inauguration_speech_19990616`, `za_mandela_presidency_ended_morning_19990616` (Mandela `until`), `za_mbeki_styled_president_farewell_19990616` (Mbeki `attested_on`) |
| 23 Apr 2004 | Mbeki re-elected unopposed; statement; inauguration announced for 27 April | `za_mbeki_na_elected_president_20040423`, `za_mbeki_election_acceptance_statement_20040423`, `za_mbeki_inauguration_announced_20040423` |
| 27 Apr 2004 | Oath; inauguration address | `za_mbeki_oath_of_office_20040427` (Mbeki 2004 `attested_on`), `za_mbeki_inauguration_ceremony_20040427` |
| 20 Sep 2008 | Presidency: the President "will step down" | `za_mbeki_resignation_intention_announced_20080920` |
| 21 Sep 2008 | Address to the nation; letter handed to the Speaker | `za_mbeki_resignation_announcement_20080921`, `za_mbeki_resignation_submitted_20080921` |
| 22 Sep 2008 | Letter read in the House; motion deferred; Mbeki still acting as President | `za_mbeki_resignation_letter_read_20080922`, `za_mbeki_resignation_effective_motion_deferred_20080922`, `za_mbeki_in_office_after_tender_20080922` |
| 23 Sep 2008 | Assembly resolves that the resignation takes effect on 25 September; Chief Justice convenes the vacancy election | `za_na_resolves_mbeki_resignation_effective_20080923`, `za_cj_vacancy_election_convened_20080923` |
| 24 Sep 2008 | Swearing-in advertised for 14h30 at Tuynhuys | `za_new_president_swearing_in_scheduled_20080924` |
| 25 Sep 2008 | Mbeki's resignation takes effect; Motlanthe elected 269-50; sworn in; Cabinet list | Mbeki 2004 `until`; `za_motlanthe_na_elected_president_20080925`, `za_motlanthe_swearing_in_recess_20080925`, `za_motlanthe_sworn_in_president_20080925`, `za_motlanthe_president_cabinet_list_20080925` (Motlanthe `attested_on`), `za_mbeki_outgoing_president_20080925`, `za_presidency_context_motlanthe_succeeded_mbeki_20080925` |
| 6 May 2009 | Zuma elected 277-47; Motlanthe still the outgoing President | `za_zuma_na_elected_president_20090506`, `za_zuma_na_elected_president_minutes_20090506`, `za_motlanthe_outgoing_president_20090506` |
| 9 May 2009 | Zuma's oath and inauguration; Motlanthe thanked | `za_zuma_oath_of_office_20090509` (Zuma 2009 `attested_on`), `za_zuma_inauguration_ceremony_20090509`, `za_zuma_inaugurated_first_term_20090509`, `za_presidency_context_zuma_inaugurated_20090509`, `za_motlanthe_thanked_by_successor_20090509` |
| 21 May 2014 | Zuma re-elected unopposed | `za_zuma_na_elected_president_20140521` |
| 24 May 2014 | Second term assumed; inauguration address | `za_zuma_second_term_assumed_20140524` (Zuma 2014 `from`), `za_zuma_inauguration_ceremony_20140524` |
| 14 Feb 2018 | President Act No. 24 (immediate effect); statement; televised announcement; tender | `za_zuma_resignation_effective_20180214` (Zuma `until`), `za_pd25_zuma_resignation_effective_20180214`, `za_gcis_zuma_resignation_vacancy_20180214`, `za_zuma_resignation_announcement_20180214`, `za_pd25_zuma_resignation_announced_20180214`, `za_zuma_resignation_tendered_20180214` |
| 15 Feb 2018 | Letter received and tabled; Ramaphosa Acting President; elected unopposed; sworn in that afternoon | `za_zuma_resignation_letter_received_20180215`, `za_zuma_resignation_letter_tabled_20180215`, `za_zuma_resignation_announced_to_na_20180215`, `za_ramaphosa_acting_president_20180215` (claim, not holder), `za_ramaphosa_na_elected_president_20180215`, `za_parliament_ramaphosa_first_elected_20180215`, `za_ramaphosa_oath_of_office_20180215`, `za_presidency_ramaphosa_sworn_in_20180215` (Ramaphosa 2018 `attested_on`) |
| 22 May 2019 | Ramaphosa elected unopposed | `za_ramaphosa_na_elected_president_20190522`, `za_parliament_ramaphosa_elected_president_20190522` |
| 25 May 2019 | Oath; inauguration address | `za_ramaphosa_oath_of_office_20190525` (Ramaphosa 2019 `attested_on`), `za_ramaphosa_inauguration_ceremony_20190525` |
| 14 Jun 2024 | Election as President-elect (existing) | `za_ramaphosa_president_elect_20240614`, unchanged |
| 19 Jun 2024 | Oath text; inauguration address | `za_ramaphosa_oath_of_office_20240619` (Ramaphosa 2024 `attested_on`), `za_ramaphosa_inauguration_ceremony_20240619` |
| undated | Presidency history list rows; gov.za directory spans | four `za_presidency_list_*` claims, `za_govza_directory_motlanthe_term_span`, `za_govza_directory_zuma_term_span` (no structured date) |

Date conventions follow CLAUDE-C01-07 and C01-08: `attested_on` is the day of the observed event as the source
dates it, and a page's issue date is `published_date`. A prospective notice is dated by the day it was made, not the
day it fixes. A day and month printed without a year take the year of the document (Procedural Developments issue
25 covers January to December 2018).

## Observations

### ZA-HOS-01 — The State President when the period opens

Evidence: Government Notice No. 1820 in Gazette No. 12059 of 16 August 1989 states that P. W. Botha vacated the
office of State President with effect from 15 August 1989 (`za_botha_vacated_state_presidency_19890815`) and that
the Cabinet designated F. W. de Klerk Acting State President, who took the acting oath and assumed the acting office
that day (`za_de_klerk_acting_state_president_designated_19890815`). He signed Proclamation No. 167 as Acting State
President on 13 September 1989 (`za_de_klerk_acting_state_president_proclamation_167_19890913`) and Proclamation
No. 177 as State President on 28 September 1989 (`za_de_klerk_state_president_proclamation_177_19890928`). Gazette
No. 12286 of 2 February 1990 prints a commission and two proclamations he signed as State President that day
(`za_de_klerk_state_president_proclamation_r16_19900202`). The FW de Klerk Foundation's transcript of his address
"following his inauguration", dated 20 September 1989, is an archival copy
(`za_de_klerk_post_inauguration_address_19890920`). The Presidency's history list gives "15 August 1989 - 10 May
1994" (`za_presidency_list_state_president_de_klerk_19890815_19940510`).

Decision: accepted in part. A new role, `za_state_president`, holds the State President under the 1983
Constitution; it is never a holder of `za_president_election`. De Klerk's holder is dated 2 February 1990, the
first primary in-office attestation in the period, with no start or end. The acting service is two claims on the
role, not a holder. The Foundation transcript is context: it implies but does not state an inauguration on or before
20 September 1989 (check defect A1).

Limits: no Gazette, Parliament or court record of the 1989 electoral-college election or the substantive oath was
found; the change from acting to substantive service lies between 13 and 28 September 1989 but is not dated. The
list's start is the acting date and is not used (defect A2). Four large September 1989 Gazettes (12093, 12101, 12111,
12114) were not checked because of disk limits.

### ZA-HOS-02 — The 1994 transition

Evidence: by a notice dated 4 May 1994, Chief Justice M. M. Corbett fixed Monday 9 May 1994 at 11:00 for the
nomination and election of a President (`za_cj_fixes_president_election_sitting_19940504`). Parliament's page for
the 2018 Students' Parliament says that in the National Assembly on 9 May 1994 Ms Sisulu nominated Nelson Mandela as
the first President (`za_mandela_na_nomination_19940509`). De Klerk signed Proclamation No. 98 as State President at
Cape Town on 9 May 1994 (`za_de_klerk_state_president_proclamation_98_19940509`). The government memorial site's
page gives the oath of office sworn by Mandela at his inauguration in Pretoria, dated 10 May 1994, with the Nelson
Mandela Foundation as its source (`za_mandela_oath_of_office_19940510`), and Zuma's Presidency-issued inauguration
address of 2009 says Mandela "took the oath of office on the 10th of May 1994"
(`za_mandela_oath_recalled_zuma_address_20090509`). The same memorial site dates Mandela's inauguration address to
10 May 1994 (`za_mandela_inauguration_ceremony_19940510`).

Decision: accepted in part. Mandela's holder is dated 10 May 1994, the day of his oath; the oath is not a start. The
election is placed at the 9 May sitting by the nomination and the Chief Justice's notice, but no record of the vote
itself was found. De Klerk's end is not stated: the last signature is 9 May 1994, the list's 10 May 1994 is not used,
and Mandela's oath is not used as his end.

Limits: Hansard and minutes of 9 May 1994 are not online. The oath record is an archival copy whose day comes from
the page's dateline; the 2009 address independently dates the oath. Parliament's 22 July 2018 release, which says
Mandela spoke from the City Hall "straight after Parliament elected him", is a dynamic page with no capture and is
not imported (defect A5).

### ZA-HOS-03 — The 1999 transition

Evidence: the President of the Constitutional Court fixed Monday 14 June 1999 at 14:30 for the election of the
President (Notice 1330, superseding Notice 1151; `za_ccpresident_fixes_president_election_sitting_19990608`). The
speech on accepting election, dated 14 June 1999 in the National Assembly, is attributed to Mbeki by DFA's index
(`za_mbeki_election_acceptance_speech_19990614`, `za_dfa_index_mbeki_election_speech_19990614`), and on 30 June he
referred to the Assembly that "elected the President of the Republic" on 14 June
(`za_mbeki_na_convened_elected_president_19990614`). The Office of the President issued his inauguration speech of
16 June 1999 at the Union Buildings (`za_mbeki_inauguration_ceremony_19990616`,
`za_dfa_index_mbeki_inauguration_speech_19990616`). At the farewell banquet that evening Mandela welcomed "the new
status I have occupied since this morning" (`za_mandela_presidency_ended_morning_19990616`), in a release headed as
hosted by President Thabo Mbeki (`za_mbeki_styled_president_farewell_19990616`). The 2009 address says Mbeki took
the oath "In June 1999" (`za_mbeki_oath_june_1999_zuma_address_20090509`).

Decision: accepted in part. Mandela's `until` is 16 June 1999, stated by Mandela himself; the release is a prepared
text marked check against delivery (defect A10). Mbeki's 1999 holder is dated 16 June 1999 with no start: no oath or
assumption record for that day was found, and Mandela's end is not used as Mbeki's start (defect A9).

Limits: the acceptance speech's body does not name the speaker; the attribution rests on DFA's index and the file
name. No oath record of 16 June 1999 was found in the June 1999 government releases that were captured.

### ZA-HOS-04 — 2004: re-election, oath and inauguration

Evidence: the Hansard of 23 April 2004 records that the Chief Justice declared Mbeki, the only nominee, properly
elected (`za_mbeki_na_elected_president_20040423`), and the Deputy President said the inauguration would fall on
27 April (`za_mbeki_inauguration_announced_20040423`). Mbeki's Presidency statement thanks members for electing him
(`za_mbeki_election_acceptance_statement_20040423`, now labelled an acceptance statement, not a second election
record; defect B4). His letter read to the Assembly on 22 September 2008 says that on 27 April 2004 he took the oath
of office (`za_mbeki_oath_of_office_20040427`), and the Presidency's inauguration address is dated that day
(`za_mbeki_inauguration_ceremony_20040427`).

Decision: accepted. The 2004 holder observation is dated 27 April 2004, the stated oath day, with no start.

Limits: the oath's officiant and time are not in the primary texts (news names the Chief Justice; a lead only).

### ZA-HOS-05 — 2008: resignation, election and oath

Evidence: on 20 September 2008 the Presidency said the President "will step down after all constitutional
requirements have been met" (`za_mbeki_resignation_intention_announced_20080920`; defect B9). On 21 September Mbeki
told the nation he had handed the Speaker a letter of resignation effective on a day the National Assembly would
determine (`za_mbeki_resignation_announcement_20080921`); a member later called it his "message to the nation on
Sunday evening". The resolution of 23 September dates the tender to 21 September
(`za_mbeki_resignation_submitted_20080921`). On 22 September the letter, signed "Thabo Mbeki", was read to the House
(`za_mbeki_resignation_letter_read_20080922`), a motion fixing 25 September was deferred on objection
(`za_mbeki_resignation_effective_motion_deferred_20080922`), and the Presidency said he met the security services
that day as President (`za_mbeki_in_office_after_tender_20080922`). On 23 September the House agreed on division
(Ayes 298, Noes 10, Abstain 1) that "the resignation of the President ... will take effect on 25 September 2008"
(`za_na_resolves_mbeki_resignation_effective_20080923`); members named Mbeki in the debate (defect B3). The Chief
Justice's letter convened the vacancy election for 25 September at 11:00 (`za_cj_vacancy_election_convened_20080923`,
a role-level claim that names no one). A Presidency advisory scheduled the swearing-in for 14h30 at Tuynhuys
(`za_new_president_swearing_in_scheduled_20080924`). On 25 September Motlanthe was elected 269-50
(`za_motlanthe_na_elected_president_20080925`); the sitting was suspended from 14:56 to 17:02 for the swearing-in at
Tuynhuys, after which he spoke as President (`za_motlanthe_swearing_in_recess_20080925`); the Presidency's profile
says he was sworn in that day (`za_motlanthe_sworn_in_president_20080925`); and the Presidency's Cabinet list of that
day names him President (`za_motlanthe_president_cabinet_list_20080925`; defect B5). Before the swearing-in, members
already spoke of "the previous President" and "the former President" (`za_mbeki_outgoing_president_20080925`;
defect B11). A retrospective Presidency page says Motlanthe succeeded Mbeki on 25 September 2008
(`za_presidency_context_motlanthe_succeeded_mbeki_20080925`).

Decision: accepted in part. Mbeki's 2004 holder ends on 25 September 2008, the effective day the Assembly resolved.
Motlanthe's holder is dated 25 September 2008 with no start. The recall by the ANC is recorded only as context inside
office-event claims.

Limits: no time of day is stated for the resignation's effect or for the oath; the oath's officiant is not named;
whether anyone acted as President between the two is unresolved (the Hansard of 25 September does not use the word
"Acting"). No Minutes of Proceedings were listed for 22-25 September 2008.

### ZA-HOS-06 — 2009: election, oath and inauguration; Motlanthe's end

Evidence: the Hansard and the reprinted minutes of 6 May 2009 record Zuma elected 277-47 over Dandala
(`za_zuma_na_elected_president_20090506`, `za_zuma_na_elected_president_minutes_20090506`), with Motlanthe addressed
as the outgoing President (`za_motlanthe_outgoing_president_20090506`). In his Presidency-issued inauguration
address of 9 May 2009 Zuma says "Today, as I take this solemn Oath of Office" (`za_zuma_oath_of_office_20090509`,
`za_zuma_inauguration_ceremony_20090509`) and thanks President Motlanthe (`za_motlanthe_thanked_by_successor_20090509`).
The Presidency's profile and its "Political Context" page date the inauguration to 9 May 2009
(`za_zuma_inaugurated_first_term_20090509`, `za_presidency_context_zuma_inaugurated_20090509`). The government's
contact directory gives Motlanthe "from 25 September 2008 to 9 May 2009" and Zuma "from 9 May 2009 until 14 February
2018" (`za_govza_directory_motlanthe_term_span`, `za_govza_directory_zuma_term_span`; defect B2), and the history list
gives Motlanthe "25 September 2008 - 9 May 2009" (`za_presidency_list_president_motlanthe_20080925_20090509`).

Decision: accepted in part. Zuma's 2009 holder is dated 9 May 2009, the stated oath day, with no start. Motlanthe has
no `until`: the only statements of his end are retrospective spans, which this packet does not use as boundaries (see
[How a start and an end are decided](#how-a-start-and-an-end-are-decided)).

Limits: the oath's officiant and time are not in the primary texts.

### ZA-HOS-07 — 2014: re-election and the second term

Evidence: the minutes of 21 May 2014 record Zuma as the only nominee, declared duly elected by Chief Justice
Mogoeng (`za_zuma_na_elected_president_20140521`). The Presidency's own copy of his inauguration address is dated
24 May 2014 (`za_zuma_inauguration_ceremony_20140524`; defect C6). The Presidency's profile states that he "assumed
his second term in office as President of the Republic on 24 May 2014" (`za_zuma_second_term_assumed_20140524`).

Decision: accepted. The 2014 holder starts on 24 May 2014, the only stated assumption in the packet. No end of his
first term is inferred from it.

Limits: the profile is retrospective and undated, not an instrument; no primary text reproduces the 2014 oath or
names its officiant.

### ZA-HOS-08 — 2018: resignation, acting service, election and oath

Evidence: President Act No. 24, reproduced in Parliament's paper of 15 February 2018, reads "I, Jacob Gedleyihlekisa
Zuma, hereby resign as President of the Republic of South Africa with immediate effect", given at Pretoria on the
14th day of February 2018 (`za_zuma_resignation_effective_20180214`); GCIS and the Table Division also give
14 February (`za_gcis_zuma_resignation_vacancy_20180214`, `za_pd25_zuma_resignation_effective_20180214`). The
Presidency's statement of 14 February announces the decision (`za_zuma_resignation_announcement_20180214`); the Table
Division records a televised announcement that day (`za_pd25_zuma_resignation_announced_20180214`); the Presidency
profile says he tendered his resignation that day (`za_zuma_resignation_tendered_20180214`, context only; defect C2).
Parliament received, announced and tabled the letter on 15 February (`za_zuma_resignation_letter_received_20180215`,
`za_zuma_resignation_announced_to_na_20180215`, `za_zuma_resignation_letter_tabled_20180215`). GCIS records Deputy
President Ramaphosa as Acting President until the election that afternoon (`za_ramaphosa_acting_president_20180215`).
The minutes record Ramaphosa as the only nominee, elected at the sitting that began nominations at 14:24
(`za_ramaphosa_na_elected_president_20180215`); a 2019 release calls it unanimous
(`za_parliament_ramaphosa_first_elected_20180215`). The Table Division records that the President-elect "was sworn in
later that afternoon at a separate ceremony at Tuynhuys" (`za_ramaphosa_oath_of_office_20180215`), and the Presidency
profile gives the day (`za_presidency_ramaphosa_sworn_in_20180215`; defect C7).

Decision: accepted. Zuma's 2014 holder ends on 14 February 2018, the day of the instrument with immediate effect.
Ramaphosa's 2018 holder is dated 15 February 2018 with no start. The acting service is a claim on the role, with the
role title "Acting President" in its row (defect C11), never a holder. The ANC recall is context only.

Limits: no clock time of the resignation, of the acting service or of the oath; the officiant is not named in a
primary text.

### ZA-HOS-09 — 2019: election and oath

Evidence: the reprinted minutes of 22 May 2019 record Ramaphosa as the only nominee, declared duly elected
(`za_ramaphosa_na_elected_president_20190522`), and Parliament's release of that day says he would be inaugurated on
the Saturday (`za_parliament_ramaphosa_elected_president_20190522`; the release itself does not say unopposed;
defect C8). In the Presidency's address of 25 May 2019 he says "I stand before you having just taken the oath to be
President" (`za_ramaphosa_oath_of_office_20190525`, `za_ramaphosa_inauguration_ceremony_20190525`).

Decision: accepted. The 2019 holder is dated 25 May 2019 with no start; the 2018 holder has no end.

Limits: the venue and officiant are not in the Presidency text (news and an uncaptured Presidency notice name Loftus
Versfeld; leads only).

### ZA-HOS-10 — 2024: the oath after the existing election claim

Evidence: the Presidency's page for the 2024 inauguration at the Union Buildings, dated Wednesday 19 June 2024 and
submitted on 23 June, gives the text of the President's oath in the name of Matamela Cyril Ramaphosa
(`za_ramaphosa_oath_of_office_20240619`; defect C10). His inauguration address of the same day says "On this day, we
assert by solemn oath the will of the people of this land" (`za_ramaphosa_inauguration_ceremony_20240619`; defect C9).

Decision: accepted. A new holder observation dated 19 June 2024 follows the existing one (14 June 2024), which is
unchanged, as is its claim and extract. The existing scope note's "assumption of office ... unresolved" wording is
replaced by the role's new scope note, which keeps its statement that the 14 June claim is an election as
President-elect only and not evidence for the ANC presidency or a caucus role.

Limits: no clock time or officiant in the primary text (a prospective Presidency notice names Chief Justice Zondo;
a lead only).

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `za_gazette_12059_19890816` | Gazette 12059, GN 1820: Botha vacates; de Klerk Acting State President (15 Aug 1989) | 389,884 bytes, `13e0e473…01e310`; gazettes.africa PDF; PDF page 1 viewed; located by the check |
| `za_gazette_12108_19890919` | Gazette 12108, Proclamation 167: signed as Acting State President (13 Sep 1989) | 414,469 bytes, `42c1a2f4…d8b2ef`; gazettes.africa PDF; PDF page 1 viewed |
| `za_gazette_12128_19890929` | Gazette 12128, Proclamation 177: signed as State President (28 Sep 1989) | 354,362 bytes, `e1ce9e84…76c7fc`; gazettes.africa PDF; PDF page 1 viewed; located by the check |
| `za_gazette_12286_19900202` | Gazette 12286, Harms commission and Proclamations R. 16-17 (2 Feb 1990) | 4,750,042 bytes, `7e86500b…d14813`; gazettes.africa PDF; PDF pages 1, 2 viewed |
| `za_fwdk_foundation_inauguration_speech_19890920` | FW de Klerk Foundation transcript, post-inauguration address (archival copy) | 109,220 bytes, `f997290b…a0a126`; capture 2026-05-21 |
| `za_presidency_history_list` | Presidency history page: lists of State Presidents and Presidents | 64,973 bytes, `bf8b63b0…269bac`; capture 2026-04-15 |
| `za_gazette_15725_19940505` | Gazette 15725, Notice 444: Chief Justice fixes 9 May 1994 sitting | 335,547 bytes, `7d80cf46…2c1101`; gazettes.africa PDF; PDF page 1 viewed; located by the check |
| `za_gazette_15740_19940509` | Gazette 15740, Proclamation 98: signed as State President (9 May 1994) | 381,753 bytes, `dbea3c74…b3e830`; gazettes.africa PDF; PDF page 1 viewed; located by the check |
| `za_parliament_students_parliament_event_2018` | Parliament event page: 9 May 1994 nomination | 43,212 bytes, `da3505ad…d8a423`; capture 2021-03-02 |
| `za_mandela_gov_oath_19940510` | mandela.gov.za: oath of office, 10 May 1994 (archival copy) | 4,187 bytes, `503f9e60…3aeeec`; capture 2013-12-10 |
| `za_mandela_gov_inauguration_address_19940510` | mandela.gov.za: inauguration address, 10 May 1994 | 9,284 bytes, `3358e6e3…44bca4`; capture 2013-12-10 |
| `za_gazette_20215_19990614` | Gazette 20215, Notice 1330: 14 June 1999 sitting fixed | 537,340 bytes, `b3c1e0ac…fe438b`; gazettes.africa PDF; PDF page 1 viewed; located by the check |
| `za_dfa_mbeki_acceptance_election_19990614` | DFA: speech accepting election, 14 June 1999 | 30,799 bytes, `df5d8133…5ef02f`; capture 2004-10-28 |
| `za_dfa_mbeki_speech_index_2004` | DFA index of the President's speeches | 120,310 bytes, `3f705eaa…05f6cf`; capture 2004-10-24 |
| `za_govza_mbeki_closing_debate_19990630` | gov.za: closing debate, 30 June 1999 | 61,446 bytes, `30da9323…5b9c9d`; capture 2026-06-22 |
| `za_infogov_mbeki_inauguration_19990616` | Office of the President: inauguration speech, 16 June 1999 | 15,897 bytes, `f8bfd381…5aba6a`; capture 2010-12-04 |
| `za_infogov_mandela_farewell_19990616` | Office of the President: Mandela's farewell banquet speech, 16 June 1999 | 13,066 bytes, `56876526…f2f5d7`; capture 2005-06-21 |
| `za_parliament_hansard_na_20040423` | Hansard, National Assembly, 23 April 2004 | 126,976 bytes, `f81fe250…398e05`; Parliament file, Last-Modified 2 Mar 2026 |
| `za_presidency_mbeki_post_election_statement_20040423` | Presidency: statement after election, 23 April 2004 | 22,784 bytes, `fd4b7287…bd97b2`; capture 2004-06-03 |
| `za_presidency_mbeki_inauguration_address_20040427` | Presidency: inauguration address, 27 April 2004 | 37,674 bytes, `3617e47e…9ba968`; capture 2004-05-20 |
| `za_presidency_statement_step_down_20080920` | Presidency: 'will step down' statement, 20 Sep 2008 | 27,701 bytes, `d05a8ed4…cfda48`; capture 2008-09-25; located by the check |
| `za_presidency_mbeki_address_to_nation_20080921` | Presidency: address to the nation, 21 Sep 2008 | 18,021 bytes, `6b09cbf2…46f86c`; capture 2008-09-25 |
| `za_presidency_statement_security_services_20080922` | Presidency: security-services statement, 22 Sep 2008 | 28,188 bytes, `f495dcfa…8eec1f`; capture 2008-09-25; located by the check |
| `za_parliament_hansard_na_20080922` | Hansard, National Assembly, 22 Sep 2008 | 623,104 bytes, `b6ab9c44…0badf5`; Parliament file, Last-Modified 2 Mar 2026 |
| `za_parliament_hansard_na_20080923` | Hansard, National Assembly, 23 Sep 2008 | 662,016 bytes, `38a1080e…169e91`; Parliament file, Last-Modified 2 Mar 2026 |
| `za_presidency_swearing_in_advisory_20080924` | Presidency: swearing-in media advisory, 24 Sep 2008 | 7,014 bytes, `b6eeadd6…c51da9`; capture 2008-09-26 |
| `za_parliament_hansard_na_20080925` | Hansard, National Assembly, 25 Sep 2008 | 178,176 bytes, `ad14dad0…2a179f`; Parliament file, Last-Modified 2 Mar 2026 |
| `za_presidency_cabinet_list_20080925` | Presidency: Cabinet list, 25 Sep 2008 | 33,333 bytes, `14e0bd21…015a9f`; capture 2008-10-02; located by the check |
| `za_presidency_profile_motlanthe` | Presidency: Motlanthe profile | 65,736 bytes, `e0c27b1b…90edde`; capture 2025-08-14 |
| `za_presidency_political_context` | Presidency: 'Political Context' page | 66,159 bytes, `412ffa5a…0670df`; capture 2026-05-10; located by the check |
| `za_govza_directory_motlanthe` | gov.za contact directory: Motlanthe | 35,579 bytes, `88436ae8…c3780e`; capture 2026-06-24; located by the check |
| `za_govza_directory_zuma` | gov.za contact directory: Zuma | 37,531 bytes, `92051c59…ec15df`; capture 2026-06-22; located by the check |
| `za_parliament_hansard_na_20090506` | Hansard, National Assembly, 6 May 2009 | 174,080 bytes, `336921b4…1f2c4c`; Parliament file, Last-Modified 2 Mar 2026 |
| `za_parliament_minutes_na_20090506` | Minutes of Proceedings No 1-2009, 6 May 2009 (reprint) | 173,801 bytes, `53eef917…97f4f2`; Parliament file, Last-Modified 2 Mar 2026; PDF pages 1, 5, 6 viewed |
| `za_presidency_zuma_inauguration_address_20090509` | Presidency: inauguration address, 9 May 2009 | 16,099 bytes, `4311d0a1…82e7d0`; capture 2010-02-07 |
| `za_parliament_na_minutes_20140521` | Minutes of Proceedings No 1-2014, 21 May 2014 | 172,594 bytes, `bafafbf0…237190`; Parliament file, Last-Modified 2 Mar 2026; PDF pages 1, 5 viewed |
| `za_presidency_zuma_inauguration_address_20140524` | Presidency: inauguration address, 24 May 2014 | 53,974 bytes, `6db6a529…405d10`; capture 2019-05-11; located by the check |
| `za_presidency_profile_zuma` | Presidency: Zuma profile | 84,076 bytes, `ca732f4e…8ea43f`; capture 2024-04-10 |
| `za_govza_zuma_resignation_statement_20180214` | Presidency statement on gov.za: resignation, 14 Feb 2018 | 52,991 bytes, `c3fbc550…261779`; capture 2018-02-15 |
| `za_parliament_atc14_20180215` | ATC No 14-2018 with President Act No. 24, 15 Feb 2018 | 4,678,819 bytes, `e12e46e1…f81894`; Parliament file, Last-Modified 2 Mar 2026; PDF pages 3, 4, 6 viewed |
| `za_parliament_speaker_receives_resignation_20180215` | Parliament release: Speaker receives letter, 15 Feb 2018 | 52,572 bytes, `10e486a9…4b7969`; capture 2022-03-02; located by the check |
| `za_gcis_acting_president_statement_20180215` | GCIS: Acting President statement, 15 Feb 2018 | 44,484 bytes, `8e18b2cd…f6c8bc`; capture 2018-02-15 |
| `za_parliament_na_minutes_20180215` | Minutes of Proceedings No 1-2018, 15 Feb 2018 | 98,236 bytes, `70fc3ce1…cfcee9`; Parliament file, Last-Modified 2 Mar 2026; PDF pages 1, 2 viewed |
| `za_parliament_procedural_developments_25_2018` | Procedural Developments in the NA, Issue 25 (2018) | 439,100 bytes, `5aa35699…bdb92f`; Parliament file, Last-Modified 2 Mar 2026; PDF page 4 viewed |
| `za_presidency_ramaphosa_profile` | Presidency: Ramaphosa profile | 68,987 bytes, `eb3fcd85…4cb1c5`; capture 2024-06-14 |
| `za_parliament_na_minutes_20190522` | Minutes of Proceedings No 1-2019, 22 May 2019 (reprint) | 134,623 bytes, `2de55070…64243d`; Parliament file, Last-Modified 2 Mar 2026; PDF pages 1, 6 viewed |
| `za_parliament_na_elects_president_20190522` | Parliament release: NA elects President, 22 May 2019 | 53,984 bytes, `b2684e60…768f00`; capture 2021-06-17; located by the check |
| `za_presidency_inauguration_address_20190525` | Presidency: inauguration address, 25 May 2019 | 59,461 bytes, `507e9cd9…62cbe2`; capture 2019-07-19 |
| `za_presidency_oath_of_office_20240619` | Presidency: President's oath of office, 19 June 2024 | 60,579 bytes, `022df477…b568ac`; capture 2024-06-24 |
| `za_presidency_inauguration_address_20240619` | Presidency: inauguration address, 19 June 2024 | 74,794 bytes, `aa6f1b35…c67cc7`; capture 2024-06-19 |

Thirty-two sources are raw Internet Archive captures (`id_` form) made before the cutoff; each records the capture
URL as `url`, the original address as `original_url` (without `:80`) and the capture time in its extract. Seven are
scanned Government Gazettes from the gazettes.africa archive host, whose document landing pages sit behind a
Cloudflare challenge that was not bypassed; the public PDFs are static files. Eleven are official Parliament files
(Hansard `.doc` files, minutes, an announcements paper and a procedural digest), static files with Last-Modified
2 March 2026 that must be fetched with GET because the server rejects HEAD. Each response was downloaded by the
researcher or the independent check on 22 September 2026, downloaded again the same day with the same byte count
and SHA-256, and re-hashed from the kept copy for this packet. No new source has a dynamic or unreproducible
response: the one dynamic page in the dossiers was dropped (defect A5).

Each new source has a derived factual extract under [sources/](sources/) in the packet's existing format: one row
per claim, keyed by `claim_id`, with `observation_id` `za_presidency`, `role_id`, `holder_name`, `role_title`,
`event_kind` and `attested_on`, and also the claim's text and locator, the review observation and, for list and
directory rows, the printed range as text. The extract's checksum is in the packet, separate from the response hash.
Original pages, PDFs, Word files and renders are not checked in, and no seal, coat of arms, signature or photograph
is republished.

Two sources are archival copies whose limitation is stated in their scope notes and claims: the FW de Klerk
Foundation transcript and the memorial-site oath page credited to the Nelson Mandela Foundation. Retrospective
official pages (the Presidency history list, profiles and "Political Context" page, and the gov.za directory) are
labelled as such and are used for no boundary except Zuma's stated assumption of 24 May 2014.

Source types: `primary_government_gazette_archive_copy`, `archival_copy_of_official_speech`,
`primary_government_reference_list_archived`, `government_hosted_speech_archived`,
`primary_presidency_release_archived`, `primary_presidency_profile_archived`, `primary_presidency_page_archived`,
`primary_government_statement_archived`, `government_contact_directory_archived`,
`primary_legislature_page_archived`, `primary_legislature_release_archived`, `primary_legislature_hansard`,
`primary_legislature_minutes`, `primary_legislature_announcements_paper` and `primary_legislature_procedural_digest`.

## Leads not imported

- Parliament, "Parliament This Week", 22 July 2018 (https://www.parliament.gov.za/press-releases/parliament-week-07-22):
  says Mandela spoke from the City Hall "straight after Parliament elected him". A dynamic page with no Internet Archive
  capture; three downloads gave different bytes (62,739 bytes, `365aa6dd…e4b578`; 62,541 bytes, `40ce9bd4…fb459d`;
  62,692 bytes, `01d11cae…81705f`) while the article text was the same. Not reproducible, so not imported (defect A5).
- mandela.gov.za, Cape Town address of 9 May 1994 (capture 20131210203515 of `940509_inauguration.htm`, 10,310 bytes,
  `5fc81dc7…4f5e79`): its archive heading calls it an "inauguration as State President", but it mentions no National
  Assembly vote, oath or State Presidency; without the dropped release it bounds nothing (defect A6).
- info.gov.za, Mandela's luncheon speech of 13 June 1999 (capture 20110301112939 of `990614123p1003.htm`, 12,170 bytes,
  `d4ba07ef…6fca35`): Mandela styled President three days before the inauguration, in a check-against-delivery
  text; it bounds nothing (defect A4).
- gov.za contact directory, Thabo Mbeki (https://www.gov.za/about-government/contact-directory/thabo-mvuyelwa-mbeki-mr,
  read live on 22 September 2026, 37,443 bytes, `bd53ff0e…d911ec`; a dynamic page, and the Internet Archive was
  offline for a capture lookup): "from 14 June 1999 to September 2008". Used only as the reason directory spans are
  not boundaries.
- The Presidency's Mbeki profile at `/former-president-thabo-mvuyelwa-mbeki` (capture 20251114031652, 68,368 bytes,
  `55478200…b00799`): "(2004, second term)" at year precision; adds no day (defect B10).
- The Presidency's own copy of the 21 September 2008 address (`sp0921210`, capture 20080925143904): not hashed; the
  GCIS copy is used.
- gov.za, Zuma's 2014 inauguration address as republished in June 2026 (capture 20260624042110, 72,236 bytes,
  `d239699b…5b5570`): replaced by the Presidency's own 2019 capture (defect C6).
- The Presidency's prospective notices: "Presidential Inauguration 2024" of 18 June 2024
  (https://www.thepresidency.gov.za/presidential-inauguration-2024, capture 20240619020100 listed, not downloaded)
  names Chief Justice Zondo and the Union Buildings amphitheatre; "Presidential Inauguration 2019" (no capture) names
  Loftus Versfeld. Officiant and venue leads only.
- Gazette No. 20191 of 11 June 1999 (Notice 1151; 538,332 bytes, `186da48c…552cfa`): superseded by Notice 1330, which
  is imported. Gazette No. 15757 of 17 May 1994 (Proclamation No. 101, signed "N. R. MANDELA, President" at Umtata on
  16 May 1994): read by the check, not hashed.
- Captures replaced by the check: Parliament's 2018 Speaker release (20190514093223, 20,238 bytes) and 2019 election
  release (20190612201135, 21,766 bytes); part B's capture of the Zuma profile (20250912062415, 84,150 bytes), merged
  into the April 2024 capture with identical text.
- Parliament documents not downloaded: the Hansards of 21 May 2014, 15 February 2018 and 22 May 2019, and Procedural
  Developments issues 21 and 27; the minutes cover the same events.
- Parliamentary Monitoring Group mirrors of the 2008 Hansards (https://pmg.org.za/hansard/17961/,
  https://pmg.org.za/hansard/17959/): civil-society copies; the official files are used.
- gov.za copy of Motlanthe's acceptance speech of 25 September 2008: its salutation addresses an unnamed "Acting
  President" that the Hansard text lacks; no capture found. A lead on the acting-president question only.
- Presidency releases out of scope: Motlanthe's address of 28 September 2008, the statement of 23 September 2008 on
  the Deputy President's and ministers' resignations, the advisory of 26 September 2008 on ministers' oaths, and
  GCIS's 27 April 2004 arrivals list.
- The O'Malley archive transcript (https://omalley.nelsonmandela.org/) and the DISA scan of the 2 February 1990
  opening address: archival copies not needed, since the Gazette of that day is primary.
- History sites, encyclopaedias and news: https://sahistory.org.za/ entries on the 1989 inauguration and election and
  the 1999 and 2004 speeches, Wikipedia's 1989 and 2008 election articles, SAnews, News24, Mail & Guardian, VOA,
  Politicsweb and Brand South Africa items (the 41 spoilt ballots of 2008, the Tuynhuys oath time, officiants and
  venues). ANC party copies (anc.org.za, anc1912.org.za) and the Polity mirror. Leads only.
- Catalogue records: https://catalog.hathitrust.org/Record/101829305 (National Assembly Debates from 1994) and the
  Wits index of debates; no full text of 9 May 1994 or 14 June 1999.
- The Western Cape government biography of de Klerk (404) and FW de Klerk Foundation biography: secondary.

## Sources attempted

- gazettes.africa document landing pages: HTTP 403 behind a Cloudflare JavaScript and cookie challenge, not bypassed;
  the public archive PDF host served the files directly.
- Internet Archive: intermittent connection failures, HTTP 504, 503 and 429 and "Temporarily Offline" pages for all
  three researchers and during this packet's writing; every capture used was eventually retrieved. The CDX lookup
  for the gov.za Mbeki directory entry failed twice on 22 September 2026.
- Parliament's 22 July 2018 release: no Internet Archive capture (CDX empty).
- September 1989 Gazettes checked for a notice of de Klerk's election or oath: 12058, 12067, 12068, 12092, 12094-12100,
  12102-12105, 12107, 12109, 12115, 12116 and 12122-12130 hold none; 12093 (86 MB), 12101 (88 MB), 12111 and 12114
  were not checked because of disk limits. No 1999 Gazette notice of the result or oath was found in 20190-20233.
- Hansard or minutes of 9 May 1994 and 14 June 1999: catalogue records only. No info.gov.za oath release of June 1999
  is captured (`990617935a1001` has no capture).
- Parliament's document index listed no Minutes of Proceedings for April 2004 or 22-25 September 2008; the check's
  repeat query met a technical-issues page.
- Parliament's storage host returns HTTP 500 to HEAD requests; GET works.
- The Presidency's own URL for Zuma's 2018 statement has no capture; the 2019 Presidency inauguration notice has no
  capture; the Presidency's 2019 address URL now returns 404.
- The Presidency's Mbeki profile was first tried at a wrong address (404); the correct one is listed under leads.
- judiciary.org.za and the web were searched for a Chief Justice or Constitutional Court record of the 2014, 2018,
  2019 and 2024 oaths; none was found.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | De Klerk's 20 Sep 1989 transcript labelled an inauguration ceremony; locator off by one | **Applied**: renamed `za_de_klerk_post_inauguration_address_19890920`, event kind `post_inauguration_address`, context only; the day of the inauguration is stated as implied; locator names the paragraphs by their opening words |
| A2 | "The list's start is the acting start" was unsourced | **Applied**: Gazette 12059 (GN 1820) added with two claims (Botha's vacancy; de Klerk's acting designation, oath and assumption on 15 Aug 1989); the list claim cites it |
| A3 | Mbeki 1999 inauguration locator wrong | **Applied**: body paragraphs 1, 5 and 6 by opening words, plus the closing line |
| A4 | Luncheon locator wrong; check-against-delivery not stated | **Resolved by removal**: the source bounds nothing (13 June precedes the election) and is listed under leads with the limitation |
| A5 | Parliament's 2018 release not reproducible; it states the nomination day, not the election day; "static" wrongly said | **Resolved by removal**: the source is not imported (its three observed identities are under leads); Gazette 15725 (Notice 444) added as a prospective claim; the byte-stable event page keeps the nomination day; the dossier's election claim is withdrawn and ZA-HOS-02 says the vote record was not found |
| A6 | Cape Town address said to mention no election | **Resolved by removal**: not imported once the release was dropped; listed under leads with the corrected wording ("no National Assembly vote, oath or State Presidency") |
| A7 | mandela.gov.za pages carried the 1994 event date as `published_date` | **Applied**: `published_date` null; the scope notes give the December 2013 publication and treat 10 May 1994 as the dateline |
| A8 | History list `original_url` had "www" | **Applied**: `https://thepresidency.gov.za/history`, with the live www host noted |
| A9 | Mbeki `from` 1999-06-16 rested on no oath or assumption record | **Applied**: downgraded to `attested_on` 1999-06-16 on the farewell styling; the farewell sentences are quoted in `za_mandela_presidency_ended_morning_19990616`; Mandela's end is not used as Mbeki's start |
| A10 | Mandela `until` claim did not state check against delivery | **Applied**: stated in the claim's uncertainty and the holder's uncertainty |
| A11 | De Klerk holder omitted primary records that bound his service | **Applied**: Gazettes 12059, 12128 and 15740 added as claims on `za_state_president`; still no start or end |
| A12 | Proposed holders and role lacked `sources`; `context_claim_ids` not a packet field; scope note would be wrong | **Applied**: every holder and both roles carry `sources`; context claims are role claims only; the `za_president_election` scope note is rewritten |
| B1 | `from` on oath days (Mbeki 2004, Motlanthe, Zuma 2009); the no-interval test | **Applied**: `attested_on` on the oath day for all three, and the same ruling for parts A and C; the no-interval assertion is replaced by an exact pinned holder list |
| B2 | A government page (gov.za directory) states Motlanthe's end | **Applied**: added as `za_govza_directory_motlanthe_term_span` with its limitation; `until` stays null (the checker's second option) because the directory's spans are not reliable boundaries; unresolved note reworded |
| B3 | Three 23 Sep 2008 claims did not name the holder; the Chief Justice's letter cited for the end | **Applied**: the naming basis (the signed letter; Mulder and Swart in the debate) added to both resolution claims; the letter is `za_cj_vacancy_election_convened_20080923` with `holder_name` null, kept off every holder |
| B4 | 2004 post-election statement labelled an election | **Applied**: event kind `election_acceptance_statement` (and the same for the 1999 acceptance speech, renamed `za_mbeki_election_acceptance_speech_19990614`) |
| B5 | Motlanthe holder cited the prospective advisory | **Applied**: removed; the Presidency's Cabinet list of 25 Sep 2008 added and cited |
| B6 | Holders without `sources`; claims without `observation_id` and `role_title` | **Applied**: holders carry `sources`; every extract row carries both fields; no telephone numbers copied |
| B7 | Hansard line numbers reproducible only with antiword `-w 0` | **Applied**: access method, scope notes and the `antiword_w0_text_lines` locator key state it |
| B8 | Existing pinned tests would fail (byte floor, hosts, access date, counts) | **Applied**: exact pins only; the 40,000-byte floor and host and date pins kept for the original five sources by id; each new response pinned by bytes and SHA-256 |
| B9 | 20 Sep 2008 Presidency statement missed; "Sunday evening" not cited | **Applied**: `za_mbeki_resignation_intention_announced_20080920` added; the Hansard reference cited in the 21 Sep claim |
| B10 | Mbeki profile 404 was a wrong address | **Applied**: corrected in Sources attempted; the profile is year-precision only and listed under leads |
| B11 | Pre-oath references on 25 Sep 2008 not cited | **Applied**: added to `za_mbeki_outgoing_president_20080925` and the unresolved notes; no time or acting holder derived |
| C1 | Ramaphosa `from` on oath days (2018, 2019, 2024) | **Applied**: `attested_on` on each oath day |
| C2 | "Tendered his resignation" listed as support for Zuma's end | **Applied**: context only, kept off the holder |
| C3 | Two Parliament captures under the 40,000-byte floor | **Applied**: replaced by the later captures the check verified (52,572 and 53,984 bytes) |
| C4 | Existing tests would fail (no-interval rule, hosts, access date) | **Applied**: exact pins, as B8 |
| C5 | Holder shape (`from_claim_ids` and similar; no uncertainty) | **Applied**: packet holder shape with `uncertainty` on every new holder |
| C6 | 2014 locator pointed at a search form; "fifth President" wording; Presidency copy missed | **Applied**: the Presidency's own capture of 2019 is the source; "fifth President" quoted as the title only; locator is the date line under the title |
| C7 | ZA-HOS-08 summary credited the Presidency with Tuynhuys | **Applied**: Tuynhuys and "later that afternoon" attributed to the Table Division only |
| C8 | ZA-HOS-09 summary said the release records an unopposed election | **Applied**: "no further nominations" attributed to the minutes only |
| C9 | 2024 address "affirmed" | **Applied**: "assert by solemn oath", quoted |
| C10 | 2024 oath page said to narrate the swearing | **Applied**: "the text of the President's oath of office in the name of" Ramaphosa; the posting date in the uncertainty and `published_date` |
| C11 | Acting service row titled as the office | **Applied**: row `role_title` "Acting President of the Republic of South Africa" (and "Acting State President" for de Klerk's acting rows) |

Missing primary records found by the checks: Gazettes 12059, 12128, 15725, 15740 and 20215 (part A); the Presidency
statements of 20 and 22 September 2008, the Cabinet list of 25 September 2008, the "Political Context" page and the
two directory entries, and the 2009 address's references to the 1994 and 1999 oaths (part B); and the Presidency's
2014 address (part C) are all imported. Gazette 20191 (superseded), Gazette 15757 (not hashed), the Mbeki profile
(year only), the Presidency's copy of the 21 September address (not hashed) and the prospective 2019 and 2024
inauguration notices are leads, for the reasons given above.

Other changes made to fit the packet's rules rather than a numbered defect:

- Claim IDs that encoded a collapsed event kind or a dropped source are renamed or withdrawn; none of the old IDs
  remains in the packet.
- List and directory claims carry no structured date or period, so no printed span can be read as a boundary; the
  printed range is kept as text in the extract rows.
- A prospective notice is dated by the day it was made (4 May 1994; 8 June 1999), not the day it fixes.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-SouthAfrica-HOS-002`: 1989 — a record of de Klerk's electoral-college election and substantive oath (the four
  unchecked September 1989 Gazettes, House of Assembly debates or a Supreme Court record).
- `C01-SouthAfrica-HOS-003`: 1994 and 1999 — the National Assembly Debates or minutes of 9 May 1994 and 14 June 1999
  (library holdings), and any record of de Klerk's end and Mbeki's 1999 oath.
- `C01-SouthAfrica-HOS-004`: oath officiants and times, 1994-2024 — a Constitutional Court, Chief Justice or
  Presidency record for each oath.
- `C01-SouthAfrica-HOS-005`: 25 September 2008 — the time the resignation took effect and whether anyone acted as
  President before Motlanthe's oath.
- `C01-SouthAfrica-HOS-006`: deputy presidents and acting presidents during absences, 1990-2026 (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Base and claim:** based on `ffe54b02` (`codex/campaign-certification`); claim commit `0e6bbf94` holds only the
  handoff. No other pending packet touches `south-africa.json`.
- `research-index.json` is regenerated in a **separate commit**. New totals: 211 sources and 1,840 claims (previously
  161 and 1,762). South Africa role observations rise from 4 to 5; organization, institution, packet and batch counts
  are unchanged, and South Africa keeps six open batches.
- Pinned tests, none loosened and no assertion removed, in `test_south_africa_research_s10h.py`:
  - counts (entries, sources, claims, roles) are now (53, 55, 187, 5), from (53, 5, 109, 4);
  - the presidency's role kinds are now exactly two `head_of_state` roles, with their ids pinned;
  - the rule that no holder has `from` or `until` still holds for every organization holder; presidency holders are
    pinned exactly as (role, name, `attested_on`, `from`, `until`);
  - the 40,000-byte floor, the four-host set and the 2026-09-14 access date still apply to the original five sources
    by id; the 50 new sources are pinned to three hosts, the 2026-09-22 access date and exact response bytes and
    SHA-256; the claims without a structured date are an exact set of seven.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here,
  and the sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows only "Observed on" when a holder has both `attested_on`
  and `until` (Mandela; Mbeki 2004); their notes, which the atlas displays, state the end. No UI code changed.
- New fields: `source_type` and `original_url` on South Africa sources (used by the Tonga packets); `review_observation`,
  `printed_range`, `text` and `locator` in extract rows; the `antiword_w0_text_lines` locator key.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator;
  this handoff is self-proposed and not registered there.

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
