# Tonga prime ministers 08: holders and transitions, 1990-2019

Packet: **CLAUDE-C01-08**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-tonga-08`, stacked on
`claude/c01-tonga-07` at `e6f9fa41` (CLAUDE-C01-07, ready for review), which itself sits on
CLAUDE-C01-04 (ready for review). Research access: 22 September 2026 (local). The historical
cutoff stays **7 September 2026**.

This packet reviews eight observations about the existing prime-minister office
(`to_prime_minister`, role `to_pm`) in [tonga.json](tonga.json), from 1 January 1990 to the
8 October 2019 appointment already in the packet: the holder when the period opens, each later
holder's appointment, acting service, resignation or death, and, for 2010, 2014 and 2017, the
separate Assembly selection, royal appointment and effective date. It adds 39 sources and 47
claims, seven `to_pm` holder observations (placed before the existing 2019 and 2021 holders),
a `to_pm` scope note and coverage notes on `to_prime_minister` and the packet. It adds no
organization, institution, role, game mapping, lifespan, portrait or avatar. The parent scope
(C01, C06, S23, WC1 and CP1) remains open.

The research dossier and an independent check were prepared before this packet was written.
Every checker defect is applied below (see [Checker defects](#checker-defects)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| TO-PM90-01 | Who held the office when the period opens? | **Accepted at year precision:** Fatafehi Tu'ipelehake; three government lists give 1965-1991, IPU corroborates; holder observed in 1990 only as a year |
| TO-PM90-02 | When did Tu'ipelehake's premiership end and Baron Vaea's begin (1991)? | **Unresolved:** the succession is supported, its day is not; IPU alone gives August 1991; Vaea holder at year precision |
| TO-PM90-03 | When did Prince 'Ulukalala Lavaka Ata take office, and when did Vaea's premiership end? | **Accepted:** stated commencement 3 Jan 2000; Vaea's end unresolved; IPU's 1999 statement a lead only |
| TO-PM90-04 | 2006: resignation, acting service, substantive appointment | **Accepted in part:** resignation accepted and Sevele appointed acting on 11 Feb 2006; the substantive appointment day is unresolved (holder attested 7 Apr 2006) |
| TO-PM90-05 | 2010: Assembly selection, royal appointment, effective date | **Accepted:** ballot 20 or 21 Dec, result presented 21 Dec; appointment audience 22 Dec 2010 at noon; no effective date stated |
| TO-PM90-06 | 2014: Assembly selection, royal appointment, effective date | **Accepted:** selection 29 Dec (15-11); appointment audience 30 Dec 2014 at 10:00; no effective date stated |
| TO-PM90-07 | 2017: status after the dissolution, re-selection, appointment, effective date | **Accepted:** re-selection 18 Dec 2017 (14-12); appointed with effect from 2 Jan 2018; oath 18 Jan 2018; status from 24 Aug 2017 unresolved |
| TO-PM90-08 | September 2019: Pohiva's death and any acting Prime Minister | **Accepted in part:** death recorded by 14 and 16 Sep 2019 and on or before 12 Sep (Auckland date); day unresolved; Sika's acting service attested as claims only |

The resulting `to_pm` holders, after the two existing string attestations (2025 and 2026), which
stay first:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| Fatafehi Tu'ipelehake | null (`attested_period` 1990) | null | null | year-range lists (1965-1991); IPU |
| Baron Vaea | null (`attested_period` 1992-1998) | null | null | year-range lists (1991-1999/2000); PMO tribute; IPU month |
| Prince 'Ulukalala Lavaka Ata | null | 2000-01-03 | 2006-02-11 | PMO "Commencement Date"; resignation accepted by the Prince Regent |
| Feleti Sevele | 2006-04-07 | null | null | first contemporaneous PMO styling as Prime Minister |
| Lord Tu'ivakano | 2010-12-22 | null | null | Palace Office: appointment audience, Letter of Appointment |
| Samuela 'Akilisi Pohiva | 2014-12-30 | null | null | PMO: appointment audience |
| Samuela 'Akilisi Pohiva | null | 2018-01-02 | null | PMO: appointed under Clause 50A "with effect from 2 January, 2018" |
| Pohiva Tu'i'onetoa (existing) | 2019-10-08 | null | null | unchanged |
| Siaosi 'Ofakivahafolau Sovaleni (existing) | null | 2021-12-27 | null | unchanged |

Only one holder has an end: Lavaka Ata, whose resignation the PMO says was accepted on 11 February
2006. No end comes from a successor's start, a leave-taking audience, a description as "former
Prime Minister", a printed year span or a death whose day is not stated. No acting Prime Minister
is a holder: acting service (Sevele in 2006, Tangi in 2009, Sika in 2019) is recorded as claims on
the role, whether it arose from a vacancy or an absence, as CLAUDE-C01-07 did for regencies.

### Date ledger

Each row is a separate dated fact with its own claim. Different dates for a selection, its
presentation to the King, an appointment audience, an effective date, an oath, an acting
appointment, a resignation, a death notice and a funeral are not inconsistencies, and none is
merged into another, even where two fall on the same day.

| Date | Event | Claim or field |
|---|---|---|
| 1965-1991 (years) | Tu'ipelehake in three government lists | `to_pmo_former_pms_2026_tuipelehake`, `to_pmo_former_pms_2015_list`, `to_mic_pm_list_2011_list` (no structured date for the range; the 2015 and 2011 list claims carry only their observation dates, 2015-04-23 and 2011-01-18, as `attested_on`) |
| after 16 Feb 1990 (reference date unknown) | IPU: "The Prime Minister is Prince Fatafefi Tu'ipelehake" | `to_ipu_1990_tuipelehake` (no structured date) |
| Aug 1991 (month, IPU only) | Tu'ipelehake retires; succeeded by Baron Vaea | `to_ipu_1993_retirement_succession_199108` |
| undated | Vaea appointed "by the Sovereign" | `to_pmo_vaea_tribute_appointment_undated` |
| 3 Jan 2000 | Lavaka Ata's stated commencement | `to_pmo_lavaka_ata_commencement_20000103`; Lavaka Ata `from` |
| Jan 2000 (month) | Timeline: Lavaka Ata appointed | `to_pmo_timeline_lavaka_ata_appointed_200001`, `..._to` |
| 11 Feb 2006 | Prince Regent accepts Lavaka Ata's resignation | `to_lavaka_ata_resignation_accepted_20060211`, `..._to`; Lavaka Ata `until` |
| 11 Feb 2006 | Prince Regent appoints Sevele Acting Prime Minister | `to_sevele_acting_pm_appointed_20060211` (claim, not holder) |
| 23 Mar 2006 | Sevele still styled Acting Prime Minister | `to_sevele_acting_pm_20060323` |
| 30 Mar 2006 (retrospective timeline) | Sevele "appointed" | `to_sevele_appointed_20060330` (claim only) |
| 7 Apr 2006 | Sevele styled Prime Minister | `to_sevele_pm_20060407`; Sevele `attested_on` |
| 13 Apr 2006 | "Acting Prime Minister, and now Prime Minister" | `to_sevele_acting_then_pm_20060413` |
| 13 Jun 2009 | Tangi named Acting Prime Minister at a funeral | `to_pmo_vaea_tribute_acting_pm_20090613` (claim, not holder) |
| 20 Dec 2010 | Special Meeting; three nominations read | `to_pm_2010_special_meeting_nominees_20101220` |
| 20 or 21 Dec 2010 | Assembly ballot, 14-12 | `to_tuivakano_assembly_result_presented_20101221` (text; no ballot date) |
| 21 Dec 2010 | Result presented to the King; appointment announced for 22 Dec | `to_tuivakano_assembly_result_presented_20101221`, `to_tuivakano_appointment_announced_20101221` |
| 22 Dec 2010, 11:00 | Sevele's leave-taking audience | `to_sevele_leave_audience_20101222` (not an end) |
| 22 Dec 2010, 12:00 | Tu'ivakano's appointment audience; Letter of Appointment | `to_tuivakano_royal_appointment_20101222`; Tu'ivakano `attested_on` |
| 9 Dec 2014 | Writ returned; procedure announced | `to_pm_2014_writ_returned_timetable_20141209` |
| 29 Dec 2014 | Two nominations; ballot; report to the King; Pohiva elected 15-11 | `to_pm_2014_two_nominations_20141229`, `to_pm_2014_result_reported_to_king_20141229`, `to_pohiva_assembly_selection_20141229`, `to_pohiva_declared_pm_elect_pmo_20141229` |
| 30 Dec 2014, 10:00 | Pohiva's appointment audience | `to_pohiva_royal_appointment_20141230`; Pohiva (2014) `attested_on` |
| 24 Aug 2017, 17:00 | Assembly dissolved | `to_dissolution_instrument_20170824` (entry level) |
| 5 Sep 2017 | Pohiva, as Prime Minister, conveys the King's consent to the Deputy PM's removal | `to_pohiva_conveys_deputy_pm_removal_20170905` |
| 12 Sep 2017 | Speaker styles Pohiva Prime Minister | `to_pohiva_styled_pm_20170912` |
| 17 Nov 2017 | Lord Tangi Interim Speaker | `to_interim_speaker_tangi_20171117` |
| 14 Dec 2017, 16:30 | Nominations close (two) | `to_pm_2017_nominations_closed_20171214` |
| 15 Dec 2017 | Meeting programme for 18 Dec | `to_pm_2017_meeting_scheduled_20171218` |
| 18 Dec 2017 | Assembly re-selects Pohiva, 14-12 | `to_pohiva_assembly_reselection_20171218` |
| 2 Jan 2018 | Appointment effective (Clause 50A) | `to_pohiva_royal_appointment_20180102`, `to_pohiva_appointment_repeated_20180122`; Pohiva (2018) `from` |
| 18 Jan 2018 | Pohiva's oaths in the Assembly | `to_pohiva_oath_assembly_20180118` (not a start) |
| 12 Sep 2019 (sitting) | Assembly prayer for the Prime Minister; indefinite adjournment | `to_la_20190912_pm_prayers_adjourned` (not a death date) |
| on or before 12 Sep 2019 (Auckland date) | Pohiva's death, implied by the funeral programme's first item | `to_pohiva_late_pm_state_funeral_20190914` (no death day stored) |
| 14 Sep 2019 | Programme for "the late" Prime Minister; Sika listed as Acting Prime Minister | `to_pohiva_late_pm_state_funeral_20190914`, `to_sika_acting_pm_20190914_programme` |
| 16 Sep 2019 | Gazette death notice, signed by Sika as Acting Prime Minister; Cabinet's mourning decisions | `to_gazette_pohiva_death_notice_20190916`, `to_sika_acting_pm_20190916_gazette`, `to_pohiva_cabinet_mourning_decisions_20190916` |
| 27 Sep and 8 Oct 2019 | Tu'i'onetoa's selection and appointment (existing) | unchanged |

Date conventions follow CLAUDE-C01-07. `attested_on` is the date of the observed event or state
as the source dates it; a page's issue date is `published_date`. "Today" in a dated document is
read as that date, and a day and month printed without a year ("5 'o Sepitema") take the year of
the document's own dateline. A year range becomes no structured date on a claim; on a holder it
supports only an `attested_period` of whole calendar years inside every cited range.

## Observations

### TO-PM90-01 — The holder when the period opens

Evidence: the current PMO list of former Prime Ministers ("K.B.E Fatafehi Tu'ipelehake ( 1965 –
1991 )"; `to_pmo_former_pms_2026_tuipelehake`), the PMO's own list as archived in April 2015
("Fatafehi Tu'ipelehake K.B.E. 1965-1991"; `to_pmo_former_pms_2015_list`), and the Ministry of
Information's list, last updated 18 January 2011 ("HRH Prince Tu'i Pelehake (Fatafehi) CBE
(1965-1991)"; `to_mic_pm_list_2011_list`). IPU's summary of the 16 February 1990 elections ends
"The Prime Minister is Prince Fatafefi Tu'ipelehake, brother of the King" (`to_ipu_1990_tuipelehake`).

Decision: accepted at year precision. The holder carries `attested_period` 1990-01-01 to
1990-12-31, the calendar year that lies inside every list's 1965-1991 range when that range is
read as one continuous tenure; `from`, `until` and `attested_on` stay null. The atlas shows it as
"Observed between 1990-01-01 and 1990-12-31; office term not established".

Limits: all three lists are retrospective and give years only. No 1990 gazette, instrument or
contemporaneous Tongan record naming him was found: the AGO gazette index lists nothing for 1990,
the AGO law reports start in 2001, and PacLII returned a bot challenge that was not bypassed.
IPU's sentence is written in the present tense after it reports events of "the next month", so it
dates from March 1990 or later; it carries no structured date (defect D6).

### TO-PM90-02 — The 1991 succession

Evidence: the three lists give Baron Vaea from 1991 (`to_pmo_former_pms_2026_vaea_lavaka`,
`to_pmo_former_pms_2015_list`, `to_mic_pm_list_2011_list`). The PMO's 2009 tribute calls him the
12th Prime Minister, appointed "by the Sovereign", without a date
(`to_pmo_vaea_tribute_appointment_undated`). IPU's 1993 summary says Tu'ipelehake "retired from
office in August 1991and was succeeded by Baron Vaea" (`to_ipu_1993_retirement_succession_199108`,
month period; spacing as printed).

Decision: unresolved. The succession is supported; its day is not. Neither Tu'ipelehake's end nor
Vaea's start is inferred from the other. The Vaea holder carries `attested_period` 1992-01-01 to
1998-12-31 (the years inside every list's range), with no start or end.

Limits: only IPU gives the month. The AGO gazette index lists no 1991 gazettes, and no 1991 PMO or
Palace pages are archived. The "12th" ordinal is recorded as printed and not used to number a
succession.

### TO-PM90-03 — Lavaka Ata's start and Vaea's end

Evidence: the PMO's page on the office, captured in April 2002, profiles Lavaka Ata with
"Commencement Date : 3 rd January, 2000" (`to_pmo_lavaka_ata_commencement_20000103`). The
PMO-hosted "Our Constitution" timeline has "2000 January - Prince 'Ulukalala Lavaka Ata, appointed
prime minister" and its Tongan entry "2000 Sanuali" (`to_pmo_timeline_lavaka_ata_appointed_200001`,
`..._to`).

Decision: accepted. The Lavaka Ata holder starts on 2000-01-03, the stated commencement. Vaea's end
is unresolved: the PMO lists disagree (1999 in the 2015 capture, 2000 now), and no retirement
record was found. The start of one premiership does not supply the end of the other.

Limits: the profile is not an instrument; the appointment or signature date is not given
separately. The same man later reigned as Tupou VI (recorded on `to_king` by CLAUDE-C01-07); that
identity is not re-derived here. IPU's 1999 statement that Langi Kavaliku "was named as Prime
Minister" after the March 1999 poll contradicts every government record and is a lead only (see
[Leads not imported](#leads-not-imported)); whether Kavaliku acted temporarily was not researched.

### TO-PM90-04 — 2006: resignation, acting service and substantive appointment

Evidence: the PMO's English and Tongan releases of 13 February 2006 state that on Saturday
11 February 2006 the Prince Regent, Crown Prince Tupouto'a, accepted Lavaka Ata's resignation with
all his portfolios (`to_lavaka_ata_resignation_accepted_20060211`), which the Tongan text calls
retirement ("malolo" as printed; `to_lavaka_ata_resignation_accepted_20060211_to`), and on the same
day appointed Sevele Acting Prime Minister ("Palemia Le'ole'o" as printed, without the macron;
`to_sevele_acting_pm_appointed_20060211`). The PMO styles Sevele "THE ACTING PRIME MINISTER" in a
speech dated 23 March 2006 (`to_sevele_acting_pm_20060323`) and "PRIME MINISTER OF THE KINGDOM OF
TONGA" in an address dated 7 April 2006 (`to_sevele_pm_20060407`), and on 13 April refers to "his
tenure as the country's Acting Prime Minister, and now Prime Minister" (`to_sevele_acting_then_pm_20060413`).
The PMO-hosted timeline dates his appointment to 30 March 2006 (`to_sevele_appointed_20060330`);
its Tongan entry gives the year only (`to_sevele_appointed_2006_to`).

Decision: accepted in part. Lavaka Ata's `until` is 2006-02-11, the stated acceptance of his
resignation. The acting appointment is a claim on the role, not a holder. The substantive holder
is event-dated to 7 April 2006, the earliest contemporaneous PMO styling as Prime Minister
(defect D3). The 30 March date is kept as a claim labelled a retrospective timeline date.

Limits: no instrument or contemporaneous release of the substantive appointment was found, and
the checker confirmed that PMO articles 103-106 (30 March to 7 April 2006) contain none. The end
of the acting service is not separately stated. The date of the resignation letter is not given.

### TO-PM90-05 — 2010: selection, appointment and effective date

Evidence: MIC reports the Special Meeting of the 26 newly elected members on 20 December 2010 at
which three nominations were read (`to_pm_2010_special_meeting_nominees_20101220`). MIC's release
of 21 December reports a 14-12 secret-ballot win over Pohiva, presented to King George Tupou V
"today at the Villa" before the announcement in the House
(`to_tuivakano_assembly_result_presented_20101221`), and the Interim Speaker's statement that the
King "will formalize" the appointment on 22 December (`to_tuivakano_appointment_announced_20101221`).
The Palace Office release of 22 December records that the Prime Minister-designate was received in
audience at 12.00 noon at the Villa "for his appointment to the office of Prime Minister" ("The
Kissing of Hands") and received his Letter of Appointment ("Tohi Fakanofo")
(`to_tuivakano_royal_appointment_20101222`), after the outgoing Sevele's leave-taking audience at
11:00 (`to_sevele_leave_audience_20101222`).

Decision: accepted. The Tu'ivakano holder is event-dated to the appointment audience, 2010-12-22,
as the packet already does for the 2019 appointment. The selection claim is dated 21 December only
for the presentation and announcement; the ballot's own day (20 or 21 December) is not stated
(defect D8). Sevele's leave audience is not an `until`.

Limits: no "with effect from" date and no clause is given; the releases do not mention Clause 50A,
whose commencement is covered by the existing `to_reform_procedure_2010`. The same MIC sentence
says "unanimously", which contradicts its own 14-12 count; the count is kept as printed.

### TO-PM90-06 — 2014: selection, appointment and effective date

Evidence: the writ was returned to the King on 9 December 2014 and the procedure announced
(`to_pm_2014_writ_returned_timetable_20141209`). On 29 December the Interim Speaker announced two
nominations, lodged on 22 and 23 December (`to_pm_2014_two_nominations_20141229`), reported the
ballot's winner to the King in an audience before announcing it
(`to_pm_2014_result_reported_to_king_20141229`), and the Assembly elected Pohiva 15-11
(`to_pohiva_assembly_selection_20141229`; PMO corroboration `to_pohiva_declared_pm_elect_pmo_20141229`).
The PMO release of 30 December states that the Prime Minister-Designate "was received-in audience
today, Tuesday 30th December 2014 at 10.00 a.m. at the Royal Palace, for his appointment to the
office of Prime Minister" (`to_pohiva_royal_appointment_20141230`).

Decision: accepted. The 2014 Pohiva holder is event-dated to 2014-12-30. This gives a day-precision
primary date for IPU's "the following day" endorsement; the existing IPU claims are unchanged and
still carry no derived date.

Limits: no effective date, instrument or clause is cited. Tu'ivakano's end is not stated; "former
Prime Minister" on 29 December is only a description.

### TO-PM90-07 — 2017: status after the dissolution, re-selection and appointment

Evidence: the Instrument of Dissolution dissolved the Assembly "with effect from Thursday 24 August
2017 at 1700 hours" and says nothing about the Prime Minister (`to_dissolution_instrument_20170824`).
A MEIDECC release of 7 September 2017 reports former Deputy Prime Minister Siaosi Sovaleni as
saying that on 5 September a letter from the "Palemia 'o Tonga Hon. 'Akilisi Pohiva" conveyed the
King's consent to his removal as Deputy Prime Minister and Minister
(`to_pohiva_conveys_deputy_pm_removal_20170905`; located by the checker). The Speaker's release of
12 September still calls him "the Hon. Prime Minister" (`to_pohiva_styled_pm_20170912`). Lord
Tangi became Interim Speaker from 17 November (`to_interim_speaker_tangi_20171117`); nominations
closed at 4:30pm on 14 December with two received (`to_pm_2017_nominations_closed_20171214`); the
meeting was scheduled for 18 December at 10:00 (`to_pm_2017_meeting_scheduled_20171218`). On
18 December Pohiva, "ka ko e Palēmia lolotonga" (and the current Prime Minister), won 14-12
against Siaosi Sovaleni (`to_pohiva_assembly_reselection_20171218`). The PMO release of 4 January
2018 (dateline misprinted "2017") states that under Clause 50A the King appointed him "with effect
from 2 January, 2018" (`to_pohiva_royal_appointment_20180102`), repeated on 22 January
(`to_pohiva_appointment_repeated_20180122`). He took his two oaths in the Assembly on 18 January
2018 (`to_pohiva_oath_assembly_20180118`).

Decision: accepted. The 2018 holder starts on 2018-01-02, the stated effective date, with no end.
The selection, the oath and the release date are separate claims and not starts. His status from
24 August 2017 to 2 January 2018 (full or caretaker) stays unresolved, and no end of his 2014
appointment is inferred.

Limits: the 5 September letter and the Speaker's styling show him acting as, and called, Prime
Minister after the dissolution; neither states his legal status. The PMO's own January 2018 wording
that he "served as the 16th Prime Minister from 2015-2017" and will serve "his second term ...
from 2018-2021" is recorded as printed and used as no boundary (defect D4). IPU's statement that he
was "dismissed by the King in August" is not supported by any Tongan record found; it is a lead,
not a claim. No caretaker instrument, 18 December minutes or warrant was found.

### TO-PM90-08 — September 2019: death and acting service

Evidence: Gazette Extraordinary No. 28 of Monday 16 September 2019 is headed "DEATH OF THE PRIME
MINISTER HONOURABLE SAMUELA 'AKILISI POHIVA" and directs half-masting on the funeral day, 19
September (`to_gazette_pohiva_death_notice_20190916`). The PMO programme of 14 September is for
"the late" Prime Minister; its first item, headed "AUCKLAND, NEW ZEALAND – 12 SEPTEMBER 2019", has
the Prime Minister resting at an Auckland funeral parlour (`to_pohiva_late_pm_state_funeral_20190914`).
The PMO notice of 16 September sets out Cabinet's mourning decisions
(`to_pohiva_cabinet_mourning_decisions_20190916`). The Assembly minutes of the 12 September sitting
record prayer for the Prime Minister and an indefinite adjournment (`to_la_20190912_pm_prayers_adjourned`).
Semisi K.L. Sika signs the gazette as "Acting Prime Minister of Tonga" (`to_sika_acting_pm_20190916_gazette`)
and is listed in advance as "Acting Prime Minister (Hon. Semisi Sika)" in the 14 September
programme (`to_sika_acting_pm_20190914_programme`).

Decision: accepted in part. The death is recorded, and the programme bounds it to on or before
12 September 2019 (Auckland date); no record states the date, time or place of death, so no day is
stored and the 2018 holder has no `until` (defect D5). The existing month claim
`to_pohiva_death_month_2019` stands. Sika's acting service is recorded as two claims, not a holder
(defect D2). The minutes are not used to date the death.

Limits: the start date and legal basis of Sika's acting service were not found. The Assembly
recommendation of 27 September and the royal appointment of 8 October 2019 are already in the
packet and unchanged.

## Sources added

| Source ID | What | Retained provenance |
|---|---|---|
| `to_pmo_former_pms_2026` | PMO, "Former Prime Ministers" list (live page) | 75,947 bytes, `bb1231c3…9c2442`; browser fetch, checker matched |
| `to_pmo_former_pms_2015` | PMO, "Former PMs" list (archived) | 53,745 bytes, `1f2f361a…c5d9f7`; capture 2015-04-23 |
| `to_mic_pm_list_2011` | MIC, "Tongan Prime Ministers" list (archived) | 30,099 bytes, `91c22bf2…64989a`; capture 2011-01-31 |
| `to_ipu_1990` | IPU PARLINE, 1990 election summary | 3,669 bytes; no raw hash (normalised `94904e4b…fbad07`) |
| `to_ipu_1993` | IPU PARLINE, 1993 election summary | 4,401 bytes; no raw hash (normalised `c4caf4ce…d5d78b`) |
| `to_pmo_vaea_tribute_20090613` | PMO tribute to Baron Vaea, 13 Jun 2009 (archived) | 28,846 bytes, `3c5f1b3b…b92ad1`; capture 2010-08-25 |
| `to_pmo_office_pm_2002` | PMO page on the Office of the Prime Minister (archived) | 7,015 bytes, `05bd7234…8bbda8`; capture 2002-04-20 |
| `to_pmo_timeline_2000_en` | "Our Constitution" timeline, January 2000 entry (archived) | 19,908 bytes, `16f6e16d…9ea6f0`; capture 2010-08-24 |
| `to_pmo_timeline_2000_to` | "Our Constitution" timeline, Tongan January 2000 entry (archived) | 28,854 bytes, `6155930a…118e02`; capture 2010-08-24 |
| `to_pmo_20060213_resignation` | PMO release on Lavaka Ata's resignation, 13 Feb 2006 (archived) | 11,375 bytes, `bb2d758a…788877`; capture 2006-05-01 |
| `to_pmo_20060213_resignation_to` | PMO Tongan release, 13 Feb 2006 (archived) | 12,521 bytes, `f58d6cd4…125614`; capture 2006-05-01 |
| `to_pmo_20060330_farewell_speech` | PMO item with the Acting PM's speech of 23 Mar 2006 (archived) | 24,857 bytes, `a0267a66…1679a7`; capture 2006-04-26 |
| `to_pmo_20060410_marist_remarks` | PMO item with the PM's address of 7 Apr 2006 (archived) | 18,700 bytes, `b1c0dce7…2c01f1`; capture 2006-05-01 |
| `to_pmo_20060413_psc` | PMO release on Public Service Commissioners, 13 Apr 2006 (archived) | 12,476 bytes, `50504b1b…522eac`; capture 2006-05-01 |
| `to_pmo_timeline_2006_en` | "Our Constitution" timeline, 2006 entry (archived) | 28,570 bytes, `96a9b335…24dd63`; capture 2010-08-24 |
| `to_pmo_timeline_2006_to` | "Our Constitution" timeline, Tongan 2006 entry (archived) | 28,746 bytes, `eb86f334…11b250`; capture 2010-08-24 |
| `to_mic_20101220_nominees` | MIC release on the two nominees, 20 Dec 2010 (archived) | 31,679 bytes, `6614d97f…c32e02`; capture 2011-11-30 |
| `to_mic_20101221_designate` | MIC release on the Prime Minister Designate, 21 Dec 2010 (archived) | 35,956 bytes, `deb4413c…74e9e3`; capture 2012-05-25 |
| `to_palace_20101222_appointment` | Palace Office release on the appointment, 22 Dec 2010 (archived) | 34,100 bytes, `dd2d472b…44684d`; capture 2011-11-30 |
| `to_la_20141209_special_sitting` | Assembly news, writ returned and timetable, 9 Dec 2014 (archived) | 46,657 bytes, `2b34e308…deb1fa`; capture 2016-10-27 |
| `to_la_20141229_nominations` | Assembly news, two nominations, 29 Dec 2014 (archived) | 32,613 bytes, `9ef62e7f…8bd24a`; capture 2016-10-27 |
| `to_la_20141229_report_to_king` | Assembly news, result reported to the King, 29 Dec 2014 (archived) | 32,805 bytes, `4e10fc8c…9b18ea`; capture 2016-10-27 |
| `to_la_20141229_pohiva_elected` | Assembly news, Pohiva elected, 29 Dec 2014 (archived) | 32,247 bytes, `a018c79a…e50495`; capture 2016-10-27 |
| `to_pmo_20141229_pm_elect` | PMO release, Prime Minister-elect, 29 Dec 2014 (archived) | 48,086 bytes, `68baba1b…db6608`; capture 2015-07-06 |
| `to_pmo_20141230_appointment` | PMO release, "King appoints", 30 Dec 2014 (archived) | 47,640 bytes, `a72e7de9…b9d350`; capture 2015-07-07 |
| `to_gazette_supp_14_2017` | [Gazette Supplement Extraordinary No. 14 of 2017](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2017/2017-0041/GazetteSupplementExtraordinaryNo.14of2017.pdf) (Instrument of Dissolution) | 204,017 bytes, `5124737e…3907ae`; PDF page 1 viewed |
| `to_la_20170912_speaker_release` | Speaker's press release, 12 Sep 2017 (archived) | 33,527 bytes, `a6908cbe…d0921b`; capture 2017-09-14 |
| `to_mic_20170907_sovaleni_letter` | MEIDECC Tongan release, 7 Sep 2017 (archived; located by the checker) | 35,165 bytes, `365cbeac…239165`; capture 2017-09-10 |
| `to_mic_20171120_interim_speaker` | Interim Speaker release, 20 Nov 2017 (archived) | 32,141 bytes, `ba0ac146…a885e6`; capture 2018-10-31 |
| `to_mic_20171215_nominations` | Assembly release, close of nominations, 15 Dec 2017 (archived) | 29,425 bytes, `94a74a45…be2f85`; capture 2018-01-15 |
| `to_la_20171215_meeting_programme` | Assembly programme for 18 Dec 2017 (archived) | 39,505 bytes, `f4449649…1b34ef`; capture 2018-03-02 |
| `to_mic_20171218_selection_to` | MEIDECC Tongan release on the selection, 18 Dec 2017 (archived) | 27,861 bytes, `7aae9e76…c511be`; capture 2018-01-15 |
| `to_pmo_20180104_appointment` | PMO release, "King appoints", 4 Jan 2018 (archived) | 31,574 bytes, `a6021ca9…5e97e3`; capture 2018-01-13 |
| `to_pmo_20180122_cabinet` | PMO release on the Prime Minister and Cabinet, 22 Jan 2018 (archived) | 34,016 bytes, `f47dc0dc…f1c9e6`; capture 2018-10-30 |
| `to_la_minutes_20180118` | [Assembly minutes No. 01, 18 Jan 2018](https://parliament.gov.to/en/parliament-business/hansards-debates/13-2018/18-jan-may/309-miniti-fika-01-aho-18-o-sanuali-2018) | 311,902 bytes, `69b70a03…e8f181`; PDF pages 2, 7 viewed |
| `to_la_minutes_20190912` | [Assembly minutes No. 19A, Sep 2019](https://parliament.gov.to/en/parliament-business/hansards-debates/12-2019/17-jun-dec/305-miniti-fika-19a-aho-12-o-sepitema-2019) | 832,241 bytes, `6eee316f…58268d`; PDF pages 2, 7 viewed |
| `to_pmo_20190914_funeral_programme` | [PMO State Funeral programme, 14 Sep 2019](https://pmo.gov.to/wp-content/uploads/2019/09/Media-Release_14_09_2019_State-funeral-Programme-for-the-late-Hon.-Samuela-Akilisi-Pohiva-Prime-Minister-of-Tonga.pdf) | 502,596 bytes, `020d23f8…bb35cf`; PDF pages 1, 2 viewed |
| `to_pmo_20190916_public_notice` | [PMO public notice, 16 Sep 2019](https://pmo.gov.to/wp-content/uploads/2019/09/PUBLIC_NOTICE_16_09_2019-1.pdf) | 162,294 bytes, `4250dc7e…e96c3b`; PDF page 1 viewed |
| `to_gazette_ext_28_2019` | [Gazette Extraordinary No. 28 of 2019](https://ago.gov.to/cms/images/LEGISLATION/GAZETTES/2019/2019-0048/GazetteExtraordinaryNo.282019.pdf) | 64,374 bytes, `eb1f70f2…db75a4`; PDF page 1 viewed |

Thirty of the sources are raw Internet Archive captures (`id_` form) of original `pmo.gov.to`,
`mic.gov.to` and `parliament.gov.to` pages; each records the capture URL as `url` and the original
address as `original_url`, and each extract records the capture time. The researcher downloaded
every original except the checker-located MEIDECC capture on 22 September 2026. The independent
check re-downloaded 35 of them the same day and matched the current PMO list in its own browser
tab: every asserted hash matched, and the two IPU pages matched in byte count, differing only in
the Cloudflare token. The two Assembly minutes are served only through a Phoca Download form that
requires accepting a site download licence; the checker did not submit it and verified the kept
copies against their recorded hashes instead. For this packet every kept copy was re-hashed and
matched its recorded value, and the MEIDECC capture (7 September 2017) was downloaded twice with
identical results. "Viewed" PDF pages were rendered and visually checked in the dossier; HTML pages
were read as text.

The two IPU pages embed a per-request Cloudflare script, so no raw hash is asserted. Removing the
single injected `<script>(function(){function c(){` element (938 bytes), the rule already recorded
for IPU 2014, leaves 2,731 bytes with SHA-256
`94904e4ba80231f4462c86fec2dfd7edf2a00de4038cbba18b8d8a2723fbad07` (1990) and 3,463 bytes with
`c4caf4cea6514b6f1ee1decb0da41c5533e0b1dc9c8589b4d8194130afd5d78b` (1993). These were computed
from the researcher's copies; the checker's fresh copies differed only inside that element. A
re-fetch for this packet returned HTTP 403 (a Cloudflare challenge), so no third copy was used.

Pages read after the cutoff say so in their scope notes. The current PMO list has no revision date
and names a 2025 holder; only its entries for holders before 2020 are used.

Each new source has a checked-in derived factual extract under [sources/](sources/) (39 in all).
Every extract repeats the packet's claims exactly; its own checksum is in the packet, separate from
the original-response hash. Original pages, PDFs and renders are not checked in; no seal, coat of
arms, signature or photograph is republished.

Source types: `primary_government_reference_list` and `primary_government_reference_list_archived`
(PMO and MIC lists), `interparliamentary_election_record` (IPU), `primary_government_release_archived`,
`primary_government_office_profile_archived`, `primary_government_appointment_announcement_archived`
and `primary_palace_release_archived` (PMO, MIC/MEIDECC, Palace Office),
`government_hosted_retrospective_timeline_archived` (the "Our Constitution" timeline),
`primary_legislature_news_notice_archived`, `primary_legislature_release_archived` and
`primary_legislature_minutes` (Assembly), `primary_government_gazette` (AGO gazettes), and
`primary_government_release` and `primary_government_public_notice` (PMO 2019 PDFs).

## Leads not imported

- IPU PARLINE, 1999 election summary ([2317_99.htm](https://data.ipu.org/election-summary/HTML/2317_99.htm),
  4,733 bytes, per-request Cloudflare token): "Mr. Langi Kavaliku was named as Prime Minister to
  replace the long-serving Baron Vaea, who had retired." It contradicts all three government lists
  and the PMO's 3 January 2000 commencement date. A possible acting role for Kavaliku in 1999 is a
  lead only (TO-PM90-03).
- IPU Parline, November 2017 election ([TO-LC01-E20171116](https://data.ipu.org/parliament/TO/TO-LC01/election/TO-LC01-E20171116/),
  the existing `to_ipu_2017`): calls Pohiva "the former Prime Minister 'Akilisi Pohiva,
  dismissed by the King in August". Not supported by any Tongan record found: the dissolution instrument is
  silent on the Prime Minister, and the 5 September letter, the Speaker's 12 September styling and
  MEIDECC's 18 December "Palēmia lolotonga" treat him as Prime Minister, though none states his
  legal status. The same page's "On 18 December, the Legislative Assembly re-elected Mr. Pohiva as
  Prime Minister" corroborates the MEIDECC release; it was not added to `to_ipu_2017`, whose exact
  claim set and extract are pinned by CLAUDE-C01-01's test, so the selection rests on the Tongan
  primary release.
- US Department of State, Country Reports on Human Rights Practices for 2011 (researcher's copy,
  96,075 bytes, `1cf0b652…901912`): "in December 2010 Parliament elected a nobles' representative,
  Lord Tu'ivakano, as prime minister". A foreign-government secondary report; not used.
- MIC, 21 December 2010: the Interim Speaker's statement that the King "will formalize" the
  appointment on 22 December is kept only as a prospective claim; the appointment itself rests on
  the Palace Office release.
- Legislative Assembly, 9 December 2014: the expected sitting dates and the expected oath on about
  20 January 2015; the 2015 oath was not verified.
- Legislative Assembly news item `290-second-nomination-for-pm` of 23 December 2014 (archived
  capture in the researcher's folder, 32,187 bytes, `52fd6128…d72180`): Tu'i'onetoa lodged the second
  2014 nomination at about 9:30am. Referred to in `to_pm_2014_two_nominations_20141229` but not
  imported.
- Legislative Assembly news of 18 and 22 January 2018 (oaths; Foreign Affairs portfolio): corroborate
  the minutes and the PMO release; not imported separately.
- Legislative Assembly news of 14 October 2019
  ([king-appoints-new-pm-and-cabinet-ministers-effective-last-week](https://parliament.gov.to/en/media-centre/latest-news/king-appoints-new-pm-and-cabinet-ministers-effective-last-week))
  and 28 October 2019 (oath): they concern the 2019 appointment already in the packet; the October
  item's "sudden death of former late PM ... last month" duplicates the existing September claim.
- Legislative Assembly minutes No. 20A: the cover reads "Mōnite, 28 Sepitema 2019" while the
  session header reads 28 'Okatopa 2019. They record Tu'i'onetoa's oath, outside this packet.
- PMO ordinals: "12th" (Vaea, 2009), "15th" (Tu'ivakano, MIC), "16th" (Pohiva, 2014 and 2018) and
  "17th" (Pohiva, 2018, and Tu'i'onetoa, 2019) are inconsistent and are not used to number a
  succession.
- The current PMO list's ranges for Pohiva (2015-2019) and Tu'i'onetoa (2020-2021) conflict with
  dated releases; they are recorded in `to_pmo_former_pms_2026_later_years` only as the reason they
  are not used.
- Temporary acting Prime Ministers during absences: the PMO's 2010 site index lists an "Australia
  Day 2009" speech by Tangi and an address by "Acting Prime Minister Lord Tuita" on the 2009
  tsunami (titles only, not read). CLAUDE-C01-07 also noted "The Acting Prime Minister, Hon. Dr.
  Feleti Sevele" in the PMO report of 13 March 2006. Not imported.
- Characterisations ("first non-noble Prime Minister"; "for the first time in history, the members
  of Tonga's parliament has elected its own prime minister"): not imported.
- News and tertiary accounts of the day of Pohiva's death: not used; no primary record states it.

## Sources attempted

- The researcher's `cdx_pmo_2006.txt` and `cdx_pmo_2010.txt` were Internet Archive "Temporarily
  Offline" pages, not CDX data; the CDX API was queried again successfully on 22 September 2026.
  The earlier `vaea.html` and `oath704.html` are parliament.gov.to 404 pages.
- `pmo.gov.to` captures from December 2017 to December 2018: the only page capture (2 February 2018)
  is a hosting placeholder; the 2018 releases survive on the government portal instead.
- PMO WordPress REST API (posts and media before 2020): the earliest posts date from December 2018,
  and there is no 2014 or 2017 appointment post and no September 2019 death announcement.
- `parliament.gov.to` site search: `com_search` returns 404; `com_finder` found nothing for
  Tu'ivakano; the current news archive has no items between September 2011 and October 2019, and the
  current Hansard index begins in 2018. The old site's 2013-2017 minutes are archived as listing
  pages only; the minutes of 20-21 December 2010, 29 December 2014 and 18 December 2017 were not found.
- AGO gazettes by year: no items for 1990, 1991, 2000 or 2006; one unrelated item for 2010; no Prime
  Minister appointment notice in the 2014, 2015 or 2017 lists. Gazette No. 4 of 31 January 2018
  (civil-service appointments), Gazette Supplementary No. 60 of 2014 (a list of Acts) and Gazette
  Supplement Extraordinary No. 19 of 2019 (the Tongatapu 1 by-election writ of 20 September 2019)
  were read and hold no Prime Minister appointment or date of death.
- AGO Tonga Law Reports begin in 2001. PacLII's Tonga search returned an HTTP 403 bot challenge,
  which was not bypassed.
- IPU Parline 2021 page: "the late Prime Minister 'Akilisi Pohiva", with no date of death.
- Palace Office captures exist only for 2010-2014 and hold no other appointment item.
- The "Our Constitution" timeline has no 1991 entry.
- IPU 1990 and 1993 re-fetch for this packet: HTTP 403 Cloudflare challenge (see Sources added).

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| D1 | Tu'ipelehake and Vaea holders attested no date, though TO-PM90-01 was "accepted" | **Applied**: `attested_period` 1990 for Tu'ipelehake (three government lists) and 1992-1998 for Vaea, each with a year-precision and continuity statement; TO-PM90-01 relabelled "Accepted at year precision" |
| D2 | New `capacity` key: unvalidated, unrendered, inconsistent with CLAUDE-C01-07 and with the Tangi treatment | **Applied** (first option): no `capacity` key; Sevele's 2006 and Sika's 2019 acting service are role claims, not holders; the criterion (no acting service, from vacancy or absence, is a holder) is stated in the role scope note and coverage |
| D3 | Sevele's substantive holder dated from a retrospective timeline | **Applied**: holder event-dated to 7 April 2006 (`to_sevele_pm_20060407`); 30 March kept as a claim labelled a retrospective timeline date |
| D4 | IPU "dismissed" called "contradicted"; a primary release of 7 September 2017 missed; PMO "2015-2017" wording not discussed | **Applied**: reworded to "not supported by any Tongan record found" and kept as a lead; MEIDECC release added (`to_pohiva_conveys_deputy_pm_removal_20170905`, attested 5 September 2017, English as this packet's reading); the "2015-2017" and "2018-2021" wording recorded in `to_pohiva_royal_appointment_20180102`, and the "2015-2017" wording in the 2014 Pohiva holder's uncertainty; status 24 Aug 2017 - 2 Jan 2018 unresolved |
| D5 | Death bound stated as "by 14 September" and a 12-14 September period that reads as a death window | **Applied**: "on or before 12 September 2019 (Auckland date); day not stated"; the period replaced by `attested_on` 2019-09-14 (publication); Pohiva's `until` null |
| D6 | IPU 1990 claim dated to the election day | **Applied**: `attested_on` removed; undated IPU corroboration |
| D7 | `to_sika_acting_pm_20190917_programme` encoded the ceremony date | **Applied**: renamed `to_sika_acting_pm_20190914_programme` and described as a prospective programme listing |
| D8 | 2010 selection dated to 21 December though the ballot may have been on 20 December | **Applied**: renamed `to_tuivakano_assembly_result_presented_20101221`; 21 December dates only the presentation and announcement; the ballot day is stated as unknown |
| D9 | Tongan forms normalised in quotation | **Applied, corrected**: the capture's bytes 0x91/0x92 render as glottal stops, so only macrons are missing; quoted as printed ("malolo", "Palemia Le'ole'o", "Tama Tu'i Fakale'o", "Tupouto'a") with the macron spelling in brackets |
| D10 | Locator of `to_sevele_appointed_2006_to` did not match the quote | **Applied**: locator is now the timeline list entry, and the page title is quoted as printed |
| D11 | Live pages read after the cutoff without a note | **Applied**: scope notes on the live PMO list, both IPU pages, both gazettes, both minutes and both PMO PDFs; the list's 2021-2024 and 2025 entries are not imported |
| D12 | Two claims had no home | **Applied**: `to_pmo_former_pms_2026_later_years` and `to_pmo_vaea_tribute_acting_pm_20090613` are on the `to_pm` role |
| D13 | IPU 1993 "Last-Modified" statement not reproducible | **Applied**: removed |

Other changes made to fit the packet's rules rather than a numbered defect:

- The dossier's year-only claim periods ("1965"-"1991"; "2006"-"2006") are invalid for the validator,
  which requires ISO days, and year bounds would overstate precision; those claims now carry no
  structured period.
- The IPU 1999 statement and the IPU 2017 "dismissed" statement were in the dossier both as claims
  and as leads not imported. Both contradict the primary record, so they stay out of `tonga.json`
  and are listed as leads; `to_ipu_1999` is not a source here.
- The dossier's proposed claims on the existing `to_ipu_2017` were not added (see Leads).
- Two phrases are written so as not to trip CLAUDE-C01-04's and CLAUDE-C01-02's lead guards without
  weakening them: the funeral programme's first item is described as "dated the 12th of September
  2019" and the Assembly sitting as "12 Sepitema 2019" (as printed on the minutes), so no claim reads
  as the news-reported day of death; and the 2018-2021 span in the MEIDECC selection release is
  described as "the coming term printed as '(2018 – 2021)'", not as a term length.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-Tonga-PM-002`: 1990-1991 — a Gazette, Privy Council or Tonga Chronicle record naming
  Tu'ipelehake in 1990 and dating his retirement and Vaea's appointment (National Archives or a
  library holding).
- `C01-Tonga-PM-003`: 1999-2000 — Vaea's retirement and the instrument appointing Lavaka Ata; whether
  Langi Kavaliku acted in 1999.
- `C01-Tonga-PM-004`: 2006 and 2010-2014 — the Prince Regent's acting instrument, Sevele's
  substantive appointment, the 2010 Letter of Appointment, the 2010 ballot day and the 29 December
  2014 minutes.
- `C01-Tonga-PM-005`: 2017 — any caretaker ruling or Palace statement after the dissolution, the
  18 December 2017 minutes and the 2018 warrant.
- `C01-Tonga-PM-006`: 2019 — a primary record of the date, time and place of Pohiva's death (a PMO
  announcement or an Assembly condolence record), the 11 September 2019 minutes, and the basis and
  start of Sika's acting service.
- `C01-Tonga-PM-007`: a survey of temporary acting Prime Ministers during absences, 1990-2026.

## Integration notes (outside this packet's file boundary)

- **Stacked:** this branch is based on `claude/c01-tonga-07` at `e6f9fa41` (CLAUDE-C01-07), which sits
  on CLAUDE-C01-04. Merge CLAUDE-C01-04 and CLAUDE-C01-07 first; against them, this branch is
  CLAUDE-C01-08 only.
- **Possible conflict with CLAUDE-C01-03** (`claude/c01-tonga-03`, ready for review, not on this
  branch): it sets Sovaleni's `until` and appends an Eke holder after Sovaleni in the same `to_pm`
  `holder_claims` list, and edits the same coverage and pinned tests. This packet inserts its seven
  holders before Tu'i'onetoa, so the chronological holder order after both merge would be the seven
  1990-2018 holders, Tu'i'onetoa, Sovaleni, Eke. Textual hunks are separate but adjacent, and each
  packet's tests pin the exact holder list, so whichever merges second needs its pins updated to the
  combined list. Claude will rebase on request after either one merges.
- `research-index.json` is regenerated in a **separate commit**. New totals: 145 sources and 1,735
  claims (previously 106 and 1,688). Tonga role observations stay at 15; organization, institution,
  packet and batch counts are unchanged, and Tonga keeps one open batch, `C01-Tonga-DISC-B001`.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the
  integrator. This handoff is self-proposed and not registered there.
- Pinned tests, none loosened and no assertion removed:
  - `test_tonga_research_s10g.py`: totals 103 sources and 157 claims (from 64 and 110); the exact
    `to_pm` holder list now has nine names, and the 2019 holder is found as the one before Sovaleni.
  - `test_tonga_transition_c01_02.py`: the exact `to_pm` holder list now has nine names.
  - `test_tonga_dpfi_c01_04.py`: the exact holder list; "no Pohiva holder" became "exactly two Pohiva
    holders, (2014-12-30, no start, no end) and (from 2018-01-02, no end)"; the exact sets of stated
    starts and ends gain Lavaka Ata (2000-01-03, 2006-02-11) and Pohiva (2018-01-02). Its
    `assertNotIn('2014-12-30', raw)` guarded IPU's "following day" from being converted; the PMO
    release now states that day, so the assertion became: the string occurs exactly three times
    (the release's `published_date`, its claim and the one holder citing it), and the IPU claim still
    has no date.
  - `test_tonga_crown_c01_07.py`: the "no regent as a holder" name check excluded "Lavaka" from every
    holder name; it now exempts only the `to_pm` holder "Prince 'Ulukalala Lavaka Ata" (still checked
    for the other names) and asserts that exactly one holder anywhere carries that name, with the
    stated 2000-2006 boundaries.
  - `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was
    not run here, and the sparse checkout was not widened.
- Existing text edited: CLAUDE-C01-04's TO-DPFI-05 coverage entry on `to_prime_minister` now reads "no
  to_pm holder, start or end is created for Pohiva from the IPU record (CLAUDE-C01-08 later adds his
  holders from the PMO releases of 30 December 2014 and 4 January 2018)"; it stays the last
  `unresolved` item, so the C01-04 check on its wording still passes.
- New fields: none. `attested_period` on a prime-minister holder reuses the field CLAUDE-C01-01 put on
  the People's Party holders, which the atlas already renders. The role `scope_note` follows the
  `to_king` precedent. No UI code changed and no browser review was run.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_tonga_*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check
```

Results are recorded in the handoff.
