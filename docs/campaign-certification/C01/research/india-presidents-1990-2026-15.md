# Indian presidents 15: holders and transitions, 1990-2026

Packet: **CLAUDE-C01-15**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-in-15`, **stacked on CLAUDE-C01-11** (`claude/c01-in-11` at
`538920f1`, ready for review and not yet integrated), which is itself based on `codex/campaign-certification` at
`ffe54b02`; claim commit `f755794c`. Research access: 24 September 2026 (UTC), with this packet's re-downloads running into
the first minutes of 25 September. The historical cutoff stays **7 September 2026**.

This packet adds the presidency to [india.json](india.json), which after CLAUDE-C01-11 holds 82 party-recognition
observations and one institution, the prime-ministership. It reviews ten observations from 1 January 1990 to the cutoff:
the President when the period opens and the end of his term, the elections and assumptions of office of 1992, 1997, 2002,
2007, 2012, 2017 and 2022, the stated ends of terms, and an official attestation of the holder in office before the cutoff.
It adds 56 sources and 84 claims, a second institution (`in_presidency`, an `executive_institution` with lifecycle
`unknown`), appended after `in_prime_minister`, with one role (`in_president`, "President of India", kind
`head_of_state`), eight holder observations, a role scope note, institution coverage and a bounded note in the packet
coverage. It adds no organization, game mapping, lifespan, portrait or avatar, and it changes none of the 82 organization
observations, the two original sources or anything that CLAUDE-C01-11 added: the thirteen prime-minister holders are
unchanged. The parent scope (C01, C06, S23, WC1 and CP1) remains open.

The research was done in three parts (A: 1990-1997; B: 2002-2012; C: 2017-2026 and the stated ends), and each part was
checked independently before this packet was written. Every checker defect is applied or its outcome explained (see
[Checker defects](#checker-defects)), and every missing primary record the checks confirmed is imported, already recorded
or explained.

## Outcome

| ID | Question | Decision |
|---|---|---|
| IN-PRES-01 | The holder when the period opens (R. Venkataraman) and the end of his term only if a source states it | **Accepted in part:** observed on 13 Jan 1990 by his own Order; the Election Commission's notice of 10 Jun 1992 that his term was due to expire on 24 Jul 1992 is prospective; no primary record states the day his office ended |
| IN-PRES-02 | 1992: the election result and Shankar Dayal Sharma's oath and assumption of office | **Accepted:** poll dates appointed on 10 Jun; count and Returning Officer's declaration on 16 Jul (published 17 Jul); ceremony programme of 24 Jul; oath administered by Chief Justice M.H. Kania on 25 Jul (Lok Sabha Secretariat's journal, mirror-only); resolution and proclamation of 25 Jul 1992 (his start) |
| IN-PRES-03 | 1997: the election result and K. R. Narayanan's oath and assumption of office | **Accepted in part:** poll dates appointed on 9 Jun; declaration on 17 Jul (published 22 Jul); programme of 24 Jul; resolution and proclamation of 25 Jul 1997 (his start); a member's statement in the Lok Sabha that day refers to his assumption; no record of the oath itself or of the administering Chief Justice |
| IN-PRES-04 | 2002: the election result and A. P. J. Abdul Kalam's oath and assumption of office | **Accepted in part:** declaration and publication on 18 Jul (reported to the Rajya Sabha that day with 89.58 per cent of the votes); his own address on assumption of office of 25 Jul 2002 (his start); the Secretariat's release that he had taken the oath; the text of S.O. 788(E), the Chief Justice and the hour are not found |
| IN-PRES-05 | 2007: the election result and Pratibha Devisingh Patil's oath and assumption of office | **Accepted in part:** declaration and publication on 21 Jul; programme of 24 Jul; resolution and proclamation of 25 Jul 2007 (her start) and her address that day; the oath is attested only by a caption published in February 2008, which does not name the Chief Justice |
| IN-PRES-06 | 2012: the election result and Pranab Mukherjee's oath and assumption of office | **Accepted:** declaration and publication on 22 Jul; programme of 24 Jul; resolution and proclamation of 25 Jul 2012 (his start), his address and the Secretariat's caption of his swearing-in by the Chief Justice that day; the Lok Sabha seat notice is corroboration only |
| IN-PRES-07 | 2017: the election result and Ram Nath Kovind's oath and assumption of office | **Accepted:** declaration and publication on 20 Jul; programme of 24 Jul; resolution and proclamation of 25 Jul 2017 (his start); PIB's caption of the oath administered by Chief Justice J.S. Khehar that day; the name corrigendum of 4 Aug 2017 changes no date |
| IN-PRES-08 | 2022: the election result and Droupadi Murmu's oath and assumption of office | **Accepted:** declaration on 21 Jul (published 22 Jul); programme of 24 Jul; resolution and proclamation of 25 Jul 2022 (her start); the Secretariat's caption of the oath administered by Chief Justice N.V. Ramana that day; the PMO's report and a 29 Jul recollection are claims only |
| IN-PRES-09 | The stated ends of terms, where a source states them | **Unresolved:** no primary source states the day any President's office ended; seven notices that a term was due to expire on 24 July, three farewells, the eve of demitting office and three same-day stylings of the predecessor as former President are claims only |
| IN-PRES-10 | An official attestation of the holder in office before the cutoff | **Accepted:** Droupadi Murmu signs the Income-tax (Amendment) Ordinance, 2026 on 5 Jun 2026 and delivers the address on the eve of Independence Day on 14 Aug 2026; neither extends or bounds her term |

The resulting holder observations of `in_president`, in date order:

| Holder | `attested_on` | `from` | `until` | Basis |
|---|---|---|---|---|
| R. Venkataraman | 1990-01-13 | null | null | his Order of 13 Jan 1990 under the Delhi Administration Act ("Whereas I, R. Venkataraman, President of India"), Gazette Extraordinary No. 19 |
| Shankar Dayal Sharma | null | 1992-07-25 | null | MHA Resolution S.O. 548(E) of 25 Jul 1992 (takes his seat as President) and its Proclamation (has entered upon the office); his address on assumption and the oath by M.H. Kania (journal, mirror-only) |
| K. R. Narayanan | null | 1997-07-25 | null | MHA Resolution S.O. 525(E) of 25 Jul 1997 and its Proclamation |
| A. P. J. Abdul Kalam | null | 2002-07-25 | null | his speech on assumption of office, 25 Jul 2002 ("While I assume the office of the President"); the Secretariat's release that he had taken the oath and his first day's engagements |
| Pratibha Devisingh Patil | null | 2007-07-25 | null | MHA Resolution S.O. 1207(E) of 25 Jul 2007 and its Proclamation; her address on assumption of office |
| Pranab Mukherjee | null | 2012-07-25 | null | MHA Resolution S.O. 1687(E) of 25 Jul 2012 and its Proclamation; his address; the Secretariat's caption of his swearing-in by the Chief Justice |
| Ram Nath Kovind | null | 2017-07-25 | null | MHA Resolution S.O. 2318(E) of 25 Jul 2017 and its Proclamation; his address; the Secretariat's profile (assumed charge that day); PIB's captions of the oath by J.S. Khehar and of his signing the register |
| Droupadi Murmu | null | 2022-07-25 | null | MHA Resolution S.O. 3373(E) of 25 Jul 2022 and its Proclamation; her address; the Secretariat's caption of the oath by N.V. Ramana; in office on 29 Jul 2022, 5 Jun 2026 and 14 Aug 2026 |

### How a start and an end are decided

One rule covers all three parts. A holder has `from` only where a source states the day office was assumed, and `until`
only where a source states the day the office ended; otherwise the holder is dated by `attested_on`. From 1992 to 2022 the
Ministry of Home Affairs published, in the Gazette of India Extraordinary, Part II Section 3(ii), a resolution dated the
day of the ceremony saying that the President-elect "takes his [or her] seat as President of India" under a salute of 21
guns, with a proclamation that he or she "has entered upon the said Office"; those two acts are kept as separate claims
(`assumption_statement` and `assumption_proclaimed`) and give six of the seven starts. The 2002 notification (S.O. 788(E))
is known only from a corrigendum of 13 August 2002 that recites it as "regarding assumption of office by Dr. A P J. Abdul
Kalam as the President of India on 25-7-2002"; following the Part B check, Kalam's start rests on his own same-day speech
"on his assumption of office as President of India", and the corrigendum's recital is kept off the holder as corroboration
(check B6). A same-day address or profile that states the assumption of office is also an `assumption_statement` and
agrees with the resolution's day in every year. No clock time is stored.

The election result is never a start. The Election Commission's notice fixing the poll, the count, the Returning Officer's
declaration under section 11 of the Presidential and Vice-Presidential Elections Act, 1952, its publication by the
Legislative Department under section 12, a House's report of the result and the outgoing President's congratulations are
each their own dated claim. The Ministry of Home Affairs programme for the ceremony, published the day before, is
prospective (`assumption_ceremony_scheduled`) and never shows that the oath was taken. The oath administered by the Chief
Justice is its own claim (`oath_of_office`) where a same-day or near-contemporary record shows it: the Lok Sabha
Secretariat's journal (1992), the President's Secretariat's release (2002), the Secretariat's caption (2012), PIB's caption
(2017) and the Secretariat's caption (2022). A caption published months later (2007) is a recollection, and the
Prime Minister's Office's report that Murmu "took oath" and Mukherjee's reference in his address to the oath he had taken
are claims only. Oaths support a holder's observation and never make a start.

Proposed ends that are refused:

- **A term "due to expire".** The Election Commission's notices of 1992, 1997, 2002, 2007, 2012, 2017 and 2022 recite that
  the sitting President's term is due to expire on 24 July and fix the election so that the President elected can enter
  upon office on 25 July. They are prospective; the constitutional rule that a President continues until the successor
  enters upon office is procedure, not a date. Each is a claim that also attests the sitting President on its own day.
- **A farewell or the eve of demitting office.** The members' farewell to R. Venkataraman (21 July 1992), Pranab
  Mukherjee's farewell address and the report of the function on the eve of demitting office (24 July 2017), and Ram Nath
  Kovind's farewell address (24 July 2022) state no day on which the office ended, and are kept off the holders so that a
  last act before an unstated end cannot be read as that end.
- **A same-day styling of the predecessor as former President.** The President's Secretariat's release of 25 July 2002
  ("ex-President" K. R. Narayanan), PIB's caption of 25 July 2017 (Pranab Mukherjee, "former President of India") and the
  Secretariat's caption of 25 July 2022 ("former President Shri Ram Nath Kovind") show that the predecessor was out of
  office by then but do not state the day the office ended.
- **A successor's assumption of office.** None is used as an end.
- **Retrospective spans.** The President's website's former-president pages (captured 14 April 2026) and the Lok Sabha
  Secretariat's 1992 journal article give spans such as "25 July 1987 to 25 July 1992"; they are claims with no structured
  date, never boundaries, and their "25 July" ends differ from the Election Commission's "24 July" expiry dates.

A Vice-President acting as President would be claims only, never a holder; no such service was found in the sources
reviewed.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims. "Claim" marks a claim that
never feeds a holder; "support" marks a holder's supporting claim that does not set its date.

| Date | Event | Claim or field |
|---|---|---|
| 13 Jan 1990 | Venkataraman: in office | `in_venkataraman_president_order_delhi_19900113` (Venkataraman `attested_on`) |
| 10 Jun 1992 | Venkataraman: term due to expire on 24 July (prospective); the election: election dates appointed | `in_eci_venkataraman_term_due_to_expire_19920610` (claim), `in_eci_appoints_presidential_election_dates_19920610` (claim) |
| 16 Jul 1992 | Sharma: count reported; Sharma: Returning Officer's declaration | `in_jpi_presidential_count_19920716` (claim), `in_ro_declares_shanker_dayal_sharma_elected_19920716` (claim) |
| 17 Jul 1992 | Sharma: declaration published in the Gazette | `in_gazette_publishes_sharma_declaration_19920717` (claim) |
| 21 Jul 1992 | Venkataraman: farewell | `in_mps_farewell_to_president_venkataraman_19920721` (claim) |
| 24 Jul 1992 | Sharma: ceremony programme for the next day | `in_sharma_assumption_ceremony_notified_19920724` (claim) |
| 25 Jul 1992 | Sharma: oath; Sharma: stated assumption of office; Sharma: proclamation of entry upon the office | `in_cji_kania_administers_oath_to_sharma_19920725` (support), `in_sharma_address_on_entering_office_19920725` (Sharma `from`), `in_sharma_takes_seat_as_president_19920725` (Sharma `from`), `in_proclamation_sharma_entered_upon_office_19920725` (Sharma `from`) |
| 9 Jun 1997 | Sharma: term due to expire on 24 July (prospective); the election: election dates appointed | `in_eci_sharma_term_due_to_expire_19970609` (claim), `in_eci_appoints_presidential_election_dates_19970609` (claim) |
| 17 Jul 1997 | Narayanan: Returning Officer's declaration | `in_ro_declares_narayanan_elected_19970717` (claim) |
| 22 Jul 1997 | Narayanan: declaration published in the Gazette | `in_gazette_publishes_narayanan_declaration_19970722` (claim) |
| 24 Jul 1997 | Narayanan: ceremony programme for the next day | `in_narayanan_assumption_ceremony_notified_19970724` (claim) |
| 25 Jul 1997 | Narayanan: stated assumption of office; Narayanan: proclamation of entry upon the office; Narayanan: member's statement in the Lok Sabha | `in_narayanan_takes_seat_as_president_19970725` (Narayanan `from`), `in_proclamation_narayanan_entered_upon_office_19970725` (Narayanan `from`), `in_narayanan_assumption_recalled_in_lok_sabha_19970725` (claim) |
| 11 Jun 2002 | Narayanan: term due to expire on 24 July (prospective) | `in_eci_narayanan_term_due_to_expire_20020611` (claim) |
| 18 Jul 2002 | Kalam: Returning Officer's declaration; Kalam: declaration published in the Gazette; Kalam: result reported to the Rajya Sabha; Kalam: figures from the Chair; Kalam: outgoing President's congratulations | `in_ro_tripathi_declares_kalam_elected_20020718` (claim), `in_law_ministry_publishes_kalam_declaration_20020718` (claim), `in_rs_informed_ro_declared_kalam_20020718` (claim), `in_rs_kalam_votes_reported_20020718` (claim), `in_narayanan_congratulates_kalam_on_election_20020718` (claim) |
| 25 Jul 2002 | Kalam: stated assumption of office; Kalam: oath; Kalam: in office; Narayanan: predecessor styled former President; Kalam: recited in a corrigendum of 13 Aug 2002 | `in_kalam_assumption_speech_20020725` (Kalam `from`), `in_kalam_oath_taken_reported_20020725` (support), `in_kalam_president_first_day_engagements_20020725` (support), `in_narayanan_styled_ex_president_20020725` (claim), `in_kalam_assumption_recited_in_corrigendum_20020725` (claim) |
| 13 Aug 2002 | Kalam: name corrigendum | `in_mha_corrects_kalam_hindi_name_20020813` (claim) |
| 16 Jun 2007 | Kalam: term due to expire on 24 July (prospective) | `in_eci_kalam_term_due_to_expire_20070616` (claim) |
| 21 Jul 2007 | Patil: Returning Officer's declaration; Patil: declaration published in the Gazette | `in_ro_achary_declares_patil_elected_20070721` (claim), `in_law_ministry_publishes_patil_declaration_20070721` (claim) |
| 24 Jul 2007 | Patil: ceremony programme for the next day | `in_mha_schedules_patil_assumption_ceremony_20070724` (claim) |
| 25 Jul 2007 | Patil: stated assumption of office; Patil: proclamation of entry upon the office; Patil: later caption recalls the swearing-in; Patil: retrospective recollection | `in_patil_takes_seat_as_president_20070725` (Patil `from`), `in_mha_proclaims_patil_entered_upon_office_20070725` (Patil `from`), `in_patil_assumption_speech_20070725` (Patil `from`), `in_patil_swearing_in_recalled_caption_20070725` (claim), `in_pp_profile_patil_assumed_office_recalled_20070725` (claim) |
| 16 Jun 2012 | Patil: term due to expire on 24 July (prospective) | `in_eci_patil_term_due_to_expire_20120616` (claim) |
| 22 Jul 2012 | Mukherjee: Returning Officer's declaration; Mukherjee: declaration published in the Gazette | `in_ro_agnihotri_declares_pranab_elected_20120722` (claim), `in_law_ministry_publishes_pranab_declaration_20120722` (claim) |
| 24 Jul 2012 | Mukherjee: ceremony programme for the next day | `in_mha_schedules_pranab_assumption_ceremony_20120724` (claim) |
| 25 Jul 2012 | Mukherjee: stated assumption of office; Mukherjee: proclamation of entry upon the office; Mukherjee: Lok Sabha seat vacated with effect from this day (notice of 30 Jul); Mukherjee: oath referred to in the address; Mukherjee: oath | `in_pranab_takes_seat_as_president_20120725` (Mukherjee `from`), `in_mha_proclaims_pranab_entered_upon_office_20120725` (Mukherjee `from`), `in_pranab_ls_membership_ceased_wef_20120725` (claim), `in_pranab_assumption_speech_20120725` (Mukherjee `from`), `in_pranab_refers_to_oath_taken_20120725` (claim), `in_pranab_sworn_in_by_chief_justice_caption_20120725` (support) |
| 14 Jun 2017 | Mukherjee: term due to expire on 24 July (prospective) | `in_eci_pranab_term_due_to_expire_20170614` (claim) |
| 20 Jul 2017 | Kovind: Returning Officer's declaration; Kovind: declaration published in the Gazette | `in_ro_declares_kovind_elected_20170720` (claim), `in_kovind_declaration_published_so2282_20170720` (claim) |
| 24 Jul 2017 | Kovind: ceremony programme for the next day; Mukherjee: eve of demitting office; Mukherjee: farewell | `in_kovind_assumption_ceremony_scheduled_20170724` (claim), `in_pranab_on_eve_of_demitting_office_20170724` (claim), `in_pranab_farewell_address_20170724` (claim) |
| 25 Jul 2017 | Kovind: stated assumption of office; Kovind: proclamation of entry upon the office; Kovind: oath; Kovind: in office; Mukherjee: predecessor styled former President | `in_kovind_takes_seat_as_president_20170725` (Kovind `from`), `in_kovind_entered_upon_office_proclaimed_20170725` (Kovind `from`), `in_kovind_assumption_address_20170725` (Kovind `from`), `in_kovind_assumed_charge_stated_20170725` (Kovind `from`), `in_kovind_oath_administered_by_cji_khehar_20170725` (support), `in_kovind_signs_register_after_swearing_in_20170725` (support), `in_pranab_styled_former_president_20170725` (claim) |
| 4 Aug 2017 | Kovind: name corrigendum | `in_kovind_name_corrigendum_20170804` (claim) |
| 15 Jun 2022 | Kovind: term due to expire on 24 July (prospective) | `in_eci_kovind_term_due_to_expire_20220615` (claim) |
| 21 Jul 2022 | Murmu: Returning Officer's declaration | `in_ro_declares_murmu_elected_20220721` (claim) |
| 22 Jul 2022 | Murmu: declaration published in the Gazette | `in_murmu_declaration_published_so3325_20220722` (claim) |
| 24 Jul 2022 | Murmu: ceremony programme for the next day; Kovind: farewell | `in_murmu_assumption_ceremony_scheduled_20220724` (claim), `in_kovind_farewell_address_20220724` (claim) |
| 25 Jul 2022 | Murmu: stated assumption of office; Murmu: proclamation of entry upon the office; Murmu: oath; Kovind: predecessor styled former President; Murmu: PMO report that she took oath; Murmu: retrospective recollection | `in_murmu_takes_seat_as_president_20220725` (Murmu `from`), `in_murmu_entered_upon_office_proclaimed_20220725` (Murmu `from`), `in_murmu_assumption_address_20220725` (Murmu `from`), `in_murmu_oath_administered_by_cji_ramana_20220725` (support), `in_kovind_styled_former_president_20220725` (claim), `in_pmo_reports_murmu_took_oath_20220725` (claim), `in_rb_murmu_assumption_recalled_20220725` (claim) |
| 29 Jul 2022 | Murmu: in office | `in_murmu_receives_mozambique_delegation_20220729` (support) |
| 5 Jun 2026 | Murmu: in office | `in_murmu_promulgates_income_tax_ordinance_20260605` (support) |
| 14 Aug 2026 | Murmu: in office | `in_murmu_independence_day_eve_address_20260814` (support) |
| undated | Retrospective spans (JPI; the President's website) | `in_jpi_span_venkataraman_19870725_19920725`, `in_rb_span_venkataraman_19870725_19920725`, `in_rb_span_sharma_19920725_19970725`, `in_rb_span_narayanan_19970725_20020725` (no structured date) |

Date conventions follow CLAUDE-C01-11: `attested_on` is the day of the observed event as the source dates it (a
notification by its date, a resolution by its date, a caption by the day it states), and a page's issue date is
`published_date`. A retrospective statement is dated by the day it recalls (the Patil caption and profile and the
29 July 2022 release); the four retrospective spans carry no structured date. The Lok Sabha Secretariat's seat notice is
dated by the stated effective day (25 July 2012), not by its own date (30 July), and never feeds the holder.

## Observations

### IN-PRES-01 — The holder when the period opens

Evidence: the Ministry of Home Affairs published in the Gazette of India Extraordinary of 13 January 1990 an Order of the
President under section 31 of the Delhi Administration Act, 1966, which begins "Whereas I, R. Venkataraman, President of
India" and is signed R. Venkataraman, President of India (`in_venkataraman_president_order_delhi_19900113`). The Election
Commission's notification O.N. 49(E) of 10 June 1992 recites that his term "is due to expire on the 24th day of July,
1992" (`in_eci_venkataraman_term_due_to_expire_19920610`). The Lok Sabha Secretariat's Journal of Parliamentary
Information of September 1992 reports the members' farewell to the outgoing President on 21 July 1992
(`in_mps_farewell_to_president_venkataraman_19920721`) and says he remained in office "till 25 July 1992"
(`in_jpi_span_venkataraman_19870725_19920725`); the President's website gives his term as 25 July 1987 to 25 July 1992
(`in_rb_span_venkataraman_19870725_19920725`).

Decision: accepted in part. His holder is observed on 13 January 1990, the earliest 1990 attestation pinned to a
byte-stable response, with no start or end.

Limits: his assumption of office on 25 July 1987 lies before the period; the 1987 Gazette resolution and proclamation are
recorded as a lead (check A10). The President's Act No. 1 of 1990 that he signed, published on 10 January 1990, is a
statute text and is not imported (check A1). No primary record states the day his office ended: the Lok Sabha Debates of
16, 21, 24, 27 and 28 July 1992, the Rajya Sabha index for 23-29 July 1992 and the President's website speech pages were
searched by the Part A check, and the PIB archive holds no July 1992 release of the President's Secretariat.

### IN-PRES-02 — 1992: Shankar Dayal Sharma

Evidence: O.N. 49(E) of 10 June 1992 fixes the poll for 13 July 1992
(`in_eci_appoints_presidential_election_dates_19920610`). Sudarshan Agarwal, Returning Officer, declares on 16 July 1992
that Dr. Shanker Dayal Sharma has been duly elected (`in_ro_declares_shanker_dayal_sharma_elected_19920716`), published by
the Ministry of Law on 17 July (S.O. 531(E); `in_gazette_publishes_sharma_declaration_19920717`); the journal reports the
count of 16 July, 2,865 votes valued at 675,864, with Table II on the next page agreeing
(`in_jpi_presidential_count_19920716`; check A5). The Ministry of Home Affairs notifies on 24 July the ceremony for 11.15
A.M. the next day (S.O. 543(E); `in_sharma_assumption_ceremony_notified_19920724`). On 25 July 1992 Resolution S.O. 548(E)
says he takes his seat as President of India (`in_sharma_takes_seat_as_president_19920725`), and its Proclamation that he
has entered upon the office (`in_proclamation_sharma_entered_upon_office_19920725`). The journal records the oath
administered that day by the Chief Justice of India, Mr. Justice M.H. Kania, in the Central Hall
(`in_cji_kania_administers_oath_to_sharma_19920725`) and reproduces his address on assumption of office
(`in_sharma_address_on_entering_office_19920725`). The President's website gives his term as 25 July 1992 to 25 July
1997 (`in_rb_span_sharma_19920725_19970725`).

Decision: accepted. His start, 25 July 1992, rests on the resolution and proclamation; the declaration, its publication,
the programme and the oath stay separate claims.

Limits: the oath rests on the journal, held only as a third-party Internet Archive copy (the eparlib hosts could not be
reached). The 1992 records print "Shanker"; the 1997 notice and the President's website print "Shankar", which is the
holder name here (check A3).

### IN-PRES-03 — 1997: K. R. Narayanan

Evidence: O.N. 65(E) of 9 June 1997 fixes the poll for 14 July 1997
(`in_eci_appoints_presidential_election_dates_19970609`). S. Gopalan, Returning Officer and Secretary General of the Lok
Sabha, declares on 17 July 1997 that Shri Kocheril Raman Narayanan has been duly elected
(`in_ro_declares_narayanan_elected_19970717`), published on 22 July (S.O. 508(E);
`in_gazette_publishes_narayanan_declaration_19970722`). The programme of 24 July fixes the ceremony for 10.15 A.M. on
25 July (S.O. 517(E); `in_narayanan_assumption_ceremony_notified_19970724`). Resolution S.O. 525(E) of 25 July 1997 says he
takes his seat as President of India (`in_narayanan_takes_seat_as_president_19970725`), with the Proclamation
(`in_proclamation_narayanan_entered_upon_office_19970725`). In the Lok Sabha that day Shri Sriballav Panigrahi refers to
"our new President, Shri K.R. Narayanan, after assumption of his Office as President of India"
(`in_narayanan_assumption_recalled_in_lok_sabha_19970725`; check A2). The President's website gives his term as 25 July
1997 to 25 July 2002 (`in_rb_span_narayanan_19970725_20020725`).

Decision: accepted in part. His start, 25 July 1997, rests on the resolution and proclamation; the member's statement is a
claim only.

Limits: no accessible record shows the oath actually administered in 1997 or names the Chief Justice. A member's line at
column 266 of the same debates that the President was taking oath that day is translated from Hindi and is not used. No
1997 vote totals and no 1997 issue of the journal were found.

### IN-PRES-04 — 2002: A. P. J. Abdul Kalam

Evidence: R. C. Tripathi, Returning Officer, declares on 18 July 2002 that Shri Abdul Kalam has been duly elected
(`in_ro_tripathi_declares_kalam_elected_20020718`), published the same day (S.O. 763(E);
`in_law_ministry_publishes_kalam_declaration_20020718`). The Deputy Chairman informs the Rajya Sabha that day
(`in_rs_informed_ro_declared_kalam_20020718`), saying he got 89.58 per cent of the votes and, asked by a member, 9,22,884
votes against 1,07,366 (`in_rs_kalam_votes_reported_20020718`; check B2), and the President's Secretariat reports K. R.
Narayanan's congratulations (`in_narayanan_congratulates_kalam_on_election_20020718`). The Secretariat publishes his
speech "on his assumption of office as President of India", dated 25 July 2002 ("While I assume the office of the
President"; `in_kalam_assumption_speech_20020725`), and a release of the same day says that after taking oath as
President and seeing off the ex-President he returned to Rashtrapati Bhavan (`in_kalam_oath_taken_reported_20020725`),
with his first day's engagements (`in_kalam_president_first_day_engagements_20020725`). The Ministry of Home Affairs'
corrigendum S.O. 849(E) of 13 August 2002 recites its notification S.O. 788(E) of 25 July 2002 as "regarding assumption of
office by Dr. A P J. Abdul Kalam as the President of India on 25-7-2002"
(`in_kalam_assumption_recited_in_corrigendum_20020725`) and corrects the Hindi rendering of his name
(`in_mha_corrects_kalam_hindi_name_20020813`).

Decision: accepted in part. His start, 25 July 2002, rests on his own same-day statement of assumption of office; the
corrigendum's recital agrees and is corroboration only (check B6).

Limits: the text of S.O. 788(E) was not found (eGazette IDs 115340 and 115341 are consecutive around it); no record names
the Chief Justice who administered the oath (the speech's salutation greets the Chief Justice of India, unnamed) or gives
the hour; no 2002 ceremony programme was found in PIB's day indexes of 22-24 July 2002.

### IN-PRES-05 — 2007: Pratibha Devisingh Patil

Evidence: P. D. T. Achary, Returning Officer, declares on 21 July 2007 that Smt. Pratibha Devisingh Patil has been duly
elected (`in_ro_achary_declares_patil_elected_20070721`), published the same day (S.O. 1193(E);
`in_law_ministry_publishes_patil_declaration_20070721`). The programme of 24 July fixes the ceremony for 2.30 PM on 25 July
(S.O. 1205(E); `in_mha_schedules_patil_assumption_ceremony_20070724`). Resolution S.O. 1207(E) of 25 July 2007 says she
takes her seat as President of India (`in_patil_takes_seat_as_president_20070725`), with the Proclamation
(`in_mha_proclaims_patil_entered_upon_office_20070725`); the Secretariat publishes her speech on assumption of office of
that day (`in_patil_assumption_speech_20070725`). A caption on the Secretariat's page last modified in February 2008 recalls
her swearing-in by the Chief Justice of India on 25 July 2007 (`in_patil_swearing_in_recalled_caption_20070725`), and the
former President's office profile recalls that she assumed office that day
(`in_pp_profile_patil_assumed_office_recalled_20070725`; check B7 corrects its publisher).

Decision: accepted in part. Her start, 25 July 2007, rests on the resolution and proclamation, with her same-day address.

Limits: the oath is attested only by the later caption, which does not name the Chief Justice; the hour (2.30 PM) is only
scheduled; no vote figures were found (the Rajya Sabha did not sit on the result day).

### IN-PRES-06 — 2012: Pranab Mukherjee

Evidence: V. K. Agnihotri, Returning Officer, declares on 22 July 2012 that Shri Pranab Mukherjee has been duly elected
(`in_ro_agnihotri_declares_pranab_elected_20120722`), published the same day (S.O. 1657(E);
`in_law_ministry_publishes_pranab_declaration_20120722`). The programme of 24 July fixes the ceremony for 11.30 AM on
25 July (S.O. 1672(E); `in_mha_schedules_pranab_assumption_ceremony_20120724`). Resolution S.O. 1687(E) of 25 July 2012
says he takes his seat as President of India (`in_pranab_takes_seat_as_president_20120725`), with the Proclamation
(`in_mha_proclaims_pranab_entered_upon_office_20120725`). His address on assumption of office that day
(`in_pranab_assumption_speech_20120725`) refers to the oath he had taken (`in_pranab_refers_to_oath_taken_20120725`), and
the Secretariat's page, last modified on 3 August 2012, captions his swearing-in by the Chief Justice of India on 25 July
2012 (`in_pranab_sworn_in_by_chief_justice_caption_20120725`). The Lok Sabha Secretariat's notice S.O. 1724(E) of 30 July
2012 says he ceased to be a member with effect from 25 July 2012, consequent upon assuming the office of President
(`in_pranab_ls_membership_ceased_wef_20120725`).

Decision: accepted. His start, 25 July 2012, rests on the resolution and proclamation, with his address and the caption.
The seat notice is corroboration only: reading its effective day as his start would rest on Article 59(1), a
constitution text (check B1).

Limits: the caption does not name the Chief Justice (the address greets Justice S. H. Kapadia); the hour (11.30 AM) is
only scheduled; no vote figures were found.

### IN-PRES-07 — 2017: Ram Nath Kovind

Evidence: Anoop Mishra, Returning Officer, declares on 20 July 2017 that Shri Ramnath Kovind has been duly elected
(`in_ro_declares_kovind_elected_20170720`), published the same day (S.O. 2282(E);
`in_kovind_declaration_published_so2282_20170720`). The programme of 24 July fixes the ceremony for 12.15 PM on 25 July
(S.O. 2309(E); `in_kovind_assumption_ceremony_scheduled_20170724`). Resolution S.O. 2318(E) of 25 July 2017 says he takes
his seat as President of India (`in_kovind_takes_seat_as_president_20170725`), with the Proclamation
(`in_kovind_entered_upon_office_proclaimed_20170725`). His address on assumption of office
(`in_kovind_assumption_address_20170725`) and the Secretariat's profile released that day ("before assuming charge ... on
July 25, 2017"; `in_kovind_assumed_charge_stated_20170725`) are of the same day. PIB's photo gallery, captured the next
morning, captions the Chief Justice of India, Shri Justice J.S. Khehar, administering the oath to him on July 25, 2017
(`in_kovind_oath_administered_by_cji_khehar_20170725`; check C2) and shows him signing the register at the President's
office after the ceremony (`in_kovind_signs_register_after_swearing_in_20170725`). The Ministry of Home Affairs corrigendum
S.O. 2487(E) of 4 August 2017 sets the President's name for all official purposes as "Shri Ram Nath Kovind"
(`in_kovind_name_corrigendum_20170804`; check C4).

Decision: accepted. His start, 25 July 2017, rests on the resolution and proclamation; the corrigendum corrects only the
name.

Limits: the hour (12.15 PM) is only scheduled; no vote figures are printed in the Gazette.

### IN-PRES-08 — 2022: Droupadi Murmu

Evidence: P. C. Mody, Returning Officer, declares on 21 July 2022 that Shrimati Droupadi Murmu has been duly elected
(`in_ro_declares_murmu_elected_20220721`), published on 22 July (S.O. 3325(E);
`in_murmu_declaration_published_so3325_20220722`). The programme of 24 July fixes the ceremony for 10:15 AM on 25 July
(S.O. 3365(E); `in_murmu_assumption_ceremony_scheduled_20220724`). Resolution S.O. 3373(E) of 25 July 2022 says she takes
her seat as President of India (`in_murmu_takes_seat_as_president_20220725`), with the Proclamation
(`in_murmu_entered_upon_office_proclaimed_20220725`), and the Secretariat publishes her address on assumption of office
(`in_murmu_assumption_address_20220725`). The Secretariat's photo gallery, captured at 13:32Z that day, captions the Chief
Justice of India, Shri Justice N.V. Ramana, administering the oath to her on July 25, 2022
(`in_murmu_oath_administered_by_cji_ramana_20220725`; check C3). The Prime Minister's Office's release says she took oath
(`in_pmo_reports_murmu_took_oath_20220725`). The Secretariat's release of 29 July 2022 records a delegation calling on her
(`in_murmu_receives_mozambique_delegation_20220729`) and recalls her assumption of office on 25 July 2022
(`in_rb_murmu_assumption_recalled_20220725`; check C1).

Decision: accepted. Her start, 25 July 2022, rests on the resolution and proclamation; the PMO's report and the
recollection are claims only.

Limits: the hour (10:15 AM) is only scheduled; no vote figures are printed in the Gazette.

### IN-PRES-09 — The stated ends of terms

Evidence: seven Election Commission notices recite that the sitting President's term is due to expire on 24 July: R.
Venkataraman's (10 June 1992, counted under IN-PRES-01), Shankar Dayal Sharma's (9 June 1997;
`in_eci_sharma_term_due_to_expire_19970609`), K. R. Narayanan's (11 June 2002;
`in_eci_narayanan_term_due_to_expire_20020611`), A. P. J. Abdul Kalam's (16 June 2007;
`in_eci_kalam_term_due_to_expire_20070616`), Pratibha Devisingh Patil's (16 June 2012;
`in_eci_patil_term_due_to_expire_20120616`), Pranab Mukherjee's (14 June 2017; `in_eci_pranab_term_due_to_expire_20170614`)
and Ram Nath Kovind's (15 June 2022; `in_eci_kovind_term_due_to_expire_20220615`). Mukherjee's farewell address and the
report of the function on the eve of demitting office are of 24 July 2017 (`in_pranab_farewell_address_20170724`,
`in_pranab_on_eve_of_demitting_office_20170724`), and Kovind's farewell address of 24 July 2022 is recorded from PIB's
same-day release (`in_kovind_farewell_address_20220724`; check C6). On 25 July 2002, 2017 and 2022 the predecessor is
styled ex-President or former President (`in_narayanan_styled_ex_president_20020725`,
`in_pranab_styled_former_president_20170725`, `in_kovind_styled_former_president_20220725`).

Decision: unresolved. No primary source reviewed states the day any President's office ended, so no holder has `until`,
and no end is inferred from a successor's assumption of office.

Limits: the Part C check's full-text search of the Gazette found no text saying that a President "relinquished", "ceased
to hold" or "demitted" office. Venkataraman's 1992 notice is counted under IN-PRES-01, whose question includes his end.

### IN-PRES-10 — In office before the cutoff

Evidence: the Income-tax (Amendment) Ordinance, 2026 (No. 2 of 2026), dated 5 June 2026, is promulgated by the President
and signed "DROUPADI MURMU, President" (`in_murmu_promulgates_income_tax_ordinance_20260605`), and the President's website
publishes her address to the nation on the eve of the 80th Independence Day, dated 14 August 2026
(`in_murmu_independence_day_eve_address_20260814`).

Decision: accepted. Both attest Droupadi Murmu in office before the cutoff and support her holder; neither extends or
bounds her term.

Limits: the address is recorded from a static PDF attachment on presidentofindia.gov.in (fixed Last-Modified and ETag, no
Internet Archive capture), not from the site's per-request Drupal pages.

## Sources added

| Source ID | What | Provenance |
|---|---|---|
| `in_gazette_ext_p2s3ii_no19_19900113` | Gazette Extraordinary, 13 Jan 1990: the President's Order (Delhi Administration Act) | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_eci_on49_19920610` | Gazette Extraordinary, 10 Jun 1992: ECI O.N. 49(E), term due to expire; poll dates | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_ls_jpi_199209` | Journal of Parliamentary Information, Sep 1992: farewell, count, oath (M.H. Kania), address, span | Internet Archive item copy (mirror-only); PDF pages 1, 2, 3, 6, 9, 19, 22, 23, 45, 46, 50, 51, 76 viewed |
| `in_rb_former_president_venkataraman` | President's website, former President R. Venkataraman (retrospective span) | capture 2026-04-14 of presidentofindia.nic.in |
| `in_gazette_ext_p2s3ii_no463_19920717` | Gazette Extraordinary, 17 Jul 1992: S.O. 531(E), Returning Officer's declaration of 16 Jul | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_p2s3ii_no475_19920724` | Gazette Extraordinary, 24 Jul 1992: S.O. 543(E), ceremony programme | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_p2s3ii_no479_19920725` | Gazette Extraordinary, 25 Jul 1992: Resolution S.O. 548(E) and Proclamation | official file, egazette.gov.in; PDF pages 1, 2, 3, 4 viewed |
| `in_rb_former_president_sharma` | President's website, former President Shankar Dayal Sharma (retrospective span) | capture 2026-04-14 of presidentofindia.nic.in |
| `in_gazette_ext_eci_on65_19970609` | Gazette Extraordinary, 9 Jun 1997: ECI O.N. 65(E), term due to expire; poll dates | Internet Archive item copy (stored Gazette copy); PDF page 2 viewed |
| `in_gazette_ext_p2s3ii_no407_19970722` | Gazette Extraordinary, 22 Jul 1997: S.O. 508(E), Returning Officer's declaration of 17 Jul | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_p2s3ii_no414_19970724` | Gazette Extraordinary, 24 Jul 1997: S.O. 517(E), ceremony programme | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_p2s3ii_no416_19970725` | Gazette Extraordinary, 25 Jul 1997: Resolution S.O. 525(E) and Proclamation | official file, egazette.gov.in; PDF pages 1, 2, 3, 4 viewed |
| `in_ls_debates_19970725` | Lok Sabha Debates, 25 Jul 1997: a member refers to the new President's assumption of office | Internet Archive item copy (mirror-only); PDF pages 8, 140, 157 viewed |
| `in_rb_former_president_narayanan` | President's website, former President K. R. Narayanan (retrospective span) | capture 2026-04-14 of presidentofindia.nic.in |
| `in_gazette_ext_eci_on60_20020611` | Gazette Extraordinary, 11 Jun 2002: ECI O.N. 60(E), term due to expire | Internet Archive item copy (stored Gazette copy); PDF pages 1, 2 viewed |
| `in_gazette_ext_no648_so763e_20020718` | Gazette Extraordinary, 18 Jul 2002: S.O. 763(E), Returning Officer's declaration | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_rs_debates_20020718` | Rajya Sabha Debates, 18 Jul 2002: the result reported to the House, with figures | official file, cms.rajyasabha.nic.in; PDF pages 3, 256 viewed |
| `in_poi_narayanan_congratulates_kalam_20020718` | President's Secretariat release, 18 Jul 2002: congratulations on the election | capture 2002-08-23 of presidentofindia.nic.in |
| `in_poi_kalam_assumption_speech_20020725` | President's Secretariat: Kalam's speech on assumption of office, 25 Jul 2002 | capture 2002-08-16 of presidentofindia.nic.in |
| `in_poi_kalam_first_day_release_20020725` | President's Secretariat release, 25 Jul 2002: oath taken; first day; 'ex-President' | capture 2002-08-23 of presidentofindia.nic.in |
| `in_gazette_ext_no723_so849e_20020813` | Gazette Extraordinary, 13 Aug 2002: S.O. 849(E), corrigendum reciting S.O. 788(E) | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_eci_on85_20070616` | Gazette Extraordinary, 16 Jun 2007: ECI O.N. 85(E), term due to expire | Internet Archive item copy (stored Gazette copy); PDF page 1 viewed |
| `in_gazette_ext_no891_so1193e_20070721` | Gazette Extraordinary, 21 Jul 2007: S.O. 1193(E), Returning Officer's declaration | official file, egazette.gov.in; PDF page 1 viewed |
| `in_gazette_ext_no901_so1205e_20070724` | Gazette Extraordinary, 24 Jul 2007: S.O. 1205(E), ceremony programme | official file, egazette.gov.in; PDF page 1 viewed |
| `in_gazette_ext_no903_so1207e_20070725` | Gazette Extraordinary, 25 Jul 2007: Resolution S.O. 1207(E) and Proclamation | official file, egazette.gov.in; PDF pages 1, 2, 3, 4 viewed |
| `in_poi_patil_assumption_speech_20070725` | President's Secretariat: Patil's speech on assumption of office, 25 Jul 2007 | capture 2007-09-21 of www.presidentofindia.nic.in |
| `in_poi_swearing_in_page_patil` | President's Secretariat, 'Swearing in of The President' (caption of Feb 2008) | capture 2008-03-14 of presidentofindia.nic.in |
| `in_pp_profile_former_president_patil` | Office of the former President Patil: profile (retrospective) | capture 2013-01-01 of pratibhapatil.nic.in |
| `in_gazette_ext_eci_on45_20120616` | Gazette Extraordinary, 16 Jun 2012: ECI O.N. 45(E), term due to expire | Internet Archive item copy (stored Gazette copy); PDF pages 1, 2 viewed |
| `in_gazette_ext_no1364_so1657e_20120722` | Gazette Extraordinary, 22 Jul 2012: S.O. 1657(E), Returning Officer's declaration | official file, egazette.gov.in; PDF page 1 viewed |
| `in_gazette_ext_no1376_so1672e_20120724` | Gazette Extraordinary, 24 Jul 2012: S.O. 1672(E), ceremony programme | official file, egazette.gov.in; PDF page 1 viewed |
| `in_gazette_ext_no1389_so1687e_20120725` | Gazette Extraordinary, 25 Jul 2012: Resolution S.O. 1687(E) and Proclamation | official file, egazette.gov.in; PDF pages 1, 3, 4 viewed |
| `in_gazette_ext_no1425_so1724e_20120731` | Gazette Extraordinary, 31 Jul 2012: Lok Sabha S.O. 1724(E), seat vacated w.e.f. 25 Jul | official file, egazette.gov.in; PDF page 1 viewed |
| `in_poi_pranab_assumption_speech_20120725` | President's Secretariat: Mukherjee's speech on assumption of office, 25 Jul 2012 | capture 2012-08-09 of presidentofindia.nic.in |
| `in_poi_swearing_in_page_pranab` | President's Secretariat, 'Swearing in of The President' (caption of Aug 2012) | capture 2012-09-19 of presidentofindia.nic.in |
| `in_gazette_ext_eci_on38_20170614` | Gazette Extraordinary, 14 Jun 2017: ECI O.N. 38(E), term due to expire | official file, egazette.gov.in; PDF pages 1, 2, 3, 5, 6, 7, 10, 11, 12 viewed |
| `in_gazette_ext_so2282_declaration_20170720` | Gazette Extraordinary, 20 Jul 2017: S.O. 2282(E), Returning Officer's declaration | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_so2309_ceremony_20170724` | Gazette Extraordinary, 24 Jul 2017: S.O. 2309(E), ceremony programme | official file, egazette.gov.in; PDF page 1 viewed |
| `in_pib_pranab_eve_of_demitting_office_20170724` | PIB (President's Secretariat), 24 Jul 2017: eve of demitting office | capture 2017-07-28 of pib.nic.in |
| `in_rb_pranab_farewell_address_20170724` | President's website: Mukherjee's farewell address, 24 Jul 2017 | capture 2017-07-24 of presidentofindia.nic.in |
| `in_gazette_ext_so2318_assumption_20170725` | Gazette Extraordinary, 25 Jul 2017: Resolution S.O. 2318(E) and Proclamation | official file, egazette.gov.in; PDF pages 1, 4, 5, 6, 7 viewed |
| `in_rb_kovind_assumption_speech_20170725` | President's website: Kovind's speech on assumption of office, 25 Jul 2017 | capture 2017-07-25 of presidentofindia.nic.in |
| `in_pib_kovind_profile_20170725` | PIB (President's Secretariat), 25 Jul 2017: profile, assumed charge that day | capture 2017-07-28 of pib.nic.in |
| `in_pib_photo_20170725_oath` | PIB photo gallery: oath by J.S. Khehar; register; Mukherjee "former President" | capture 2017-07-26 of www.pib.nic.in |
| `in_gazette_ext_so2487_corrigendum_20170804` | Gazette Extraordinary, 4 Aug 2017: S.O. 2487(E), the President's official name | official file, egazette.gov.in; PDF page 1 viewed |
| `in_gazette_ext_eci_on40_20220615` | Gazette Extraordinary, 15 Jun 2022: ECI O.N. 40(E), term due to expire | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_so3325_declaration_20220722` | Gazette Extraordinary, 22 Jul 2022: S.O. 3325(E), Returning Officer's declaration of 21 Jul | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_gazette_ext_so3365_ceremony_20220724` | Gazette Extraordinary, 24 Jul 2022: S.O. 3365(E), ceremony programme | official file, egazette.gov.in; PDF pages 1, 2 viewed |
| `in_pib_kovind_farewell_address_20220724` | PIB (President's Secretariat), 24 Jul 2022: Kovind's farewell address | capture 2022-07-24 of pib.gov.in |
| `in_gazette_ext_so3373_assumption_20220725` | Gazette Extraordinary, 25 Jul 2022: Resolution S.O. 3373(E) and Proclamation | official file, egazette.gov.in; PDF pages 1, 2, 3, 4, 5, 6, 7 viewed |
| `in_rb_murmu_assumption_address_20220725` | President's website: Murmu's address on assumption of office, 25 Jul 2022 | capture 2022-07-25 of presidentofindia.nic.in |
| `in_rb_photo_gallery_20220725` | President's website photo gallery: oath by N.V. Ramana; Kovind "former President" | capture 2022-07-25 of presidentofindia.nic.in |
| `in_pib_pmo_murmu_took_oath_20220725` | PIB (Prime Minister's Office), 25 Jul 2022: she took oath | capture 2022-07-25 of pib.gov.in |
| `in_rb_murmu_mozambique_delegation_20220729` | President's website release, 29 Jul 2022: delegation; recollection of 25 Jul | capture 2022-08-15 of presidentofindia.nic.in |
| `in_gazette_ext_ordinance_2_2026_20260605` | Gazette Extraordinary, 5 Jun 2026: Income-tax (Amendment) Ordinance signed by Murmu | official file, egazette.gov.in; PDF pages 1, 2, 3 viewed |
| `in_rb_murmu_independence_day_address_20260814` | President's website (static PDF): address on the eve of Independence Day, 14 Aug 2026 | official file, www.presidentofindia.gov.in; PDF page 1 viewed |

## Response identities and stability checks

The reviewer re-downloads every recorded response and compares its byte count and SHA-256, so every identity here was
downloaded at least twice, at least 30 minutes apart or with a cache-busting query, with identical bytes, and no identity
is a page generated per request. How each was established:

- **Part A.** The researcher downloaded each response between 21:47Z and 22:07Z on 24 September and again at 22:44Z (the
  eGazette files and the journal with a cache-busting query), and the check downloaded all twelve recorded here again between 23:01Z
  and 23:03Z, 54-76 minutes after the first downloads. Every byte count and SHA-256 matched.
- **Part B.** The researcher downloaded each response twice at least 30 minutes apart (the Gazette and Rajya Sabha files
  once with a cache-busting query), and the check downloaded all eighteen again between 23:17Z and 23:20Z, the ten files
  twice (with a cache-busting query and plain) and the eight captures in the raw `id_` form with identity encoding.
- **Part C.** The researcher downloaded each response twice, 30 minutes or more apart (the eGazette files and the 2026 PDF
  with a cache-busting query), and the check downloaded all of them again between 23:08Z and 23:11Z.
- **Records added by the checks.** Each was downloaded by its check and again by this packet (six records), at least 30
  minutes after the check's first download; the times are in the table below. The large copies were deleted from scratch
  after hashing.

| Source | Bytes | SHA-256 | Downloads with identical bytes |
|---|---:|---|---|
| `in_gazette_ext_p2s3ii_no19_19900113` | 95,282 | `4e10fbbe05c55af36f827c134c5ff851a7bce1aa681cc301ebd895692a35bacf` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_eci_on49_19920610` | 477,596 | `39d6f0914fa658d0b85a2f13d77980f20dcf9d5b1a96e4e325eafe7108b55f1b` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_ls_jpi_199209` | 19,494,832 | `db63276d42ae2cf1b9670c317ec7fbd90fb4142fa9bb9bf7ef79c54d51e919f9` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_rb_former_president_venkataraman` | 56,372 | `c73d0fa3eb0c3d3fa971d2d6b0db6f64d0f92626d19de94d91add0f340be02a1` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_p2s3ii_no463_19920717` | 86,875 | `c38ac9dff034d98dfd0ba64aa7b2fef4414e99172b2451a7b1939fd1e63ed0e0` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_p2s3ii_no475_19920724` | 63,461 | `9ccccde383954b6e4333ee638d6f95cac5d25c4ebccc9cd67e7d9a22da0e3939` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_p2s3ii_no479_19920725` | 136,366 | `5d56bc3ac541eb2d9b263af206346e39d0e7e7ad5af2e018674887247ec60a46` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_rb_former_president_sharma` | 56,354 | `f835ea08d9f6fad1b3ad372f6ed0956dafb5ba3191e5c2b61db88f1d10269158` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_eci_on65_19970609` | 544,135 | `373b8d88fe5ce6dd0299468dbadea5ea385e10929e0280de36619e15e7dc4055` | check 23:34Z and 23:52Z; packet 00:03Z, 00:19Z, 00:21Z and 00:27Z on 25 Sep |
| `in_gazette_ext_p2s3ii_no407_19970722` | 60,255 | `06e047abc596a69b3083a3ab4a68e73d2e111a5f816db0f493102d094784b92b` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_p2s3ii_no414_19970724` | 60,134 | `77be3e9b8c0af5c616a55f435ffdbbb4482bdcacd6287ad5b07092a095c6d4bd` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_p2s3ii_no416_19970725` | 134,579 | `b7ca86bd390b232c16f1c4351a73a2f103f5fd71271057793ecf1fdaaf58165c` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_ls_debates_19970725` | 10,823,304 | `90481cf5c86643105a9b67df2df7146aa8438b98cc0ea47febaa71fc0180e70f` | check 23:09Z; packet 23:59Z (50 minutes later) |
| `in_rb_former_president_narayanan` | 56,405 | `8eb27596d6d95d150a1fc0280c3e8219ed54c4bfaf0147257b5044926139fca4` | research first download 21:47-22:07Z, again 22:44Z; check 23:01-23:03Z |
| `in_gazette_ext_eci_on60_20020611` | 488,037 | `8ae25a5c05fffd715ff6666843ce2cda8096ee524c5318c7ccb609d0388df87c` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_gazette_ext_no648_so763e_20020718` | 52,620 | `7414cccc8e3ecfb4cb319194e7a3c1a56d4e8bff2608ac00c5dc3e2d0e817c23` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:17Z |
| `in_rs_debates_20020718` | 1,486,333 | `c7f43556fc4c49d1a8f79752261ecfd2e3d5ff5d2e77099ceae80aefc59b3b6c` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:17Z |
| `in_poi_narayanan_congratulates_kalam_20020718` | 2,758 | `622a91d47ab4a3f36235585731c021a86de51084aa72c71549ca50f7da2601de` | research (30+ minutes apart, cache-busting for files); check 23:18Z |
| `in_poi_kalam_assumption_speech_20020725` | 13,348 | `0f7bed08a917f2f338ccbc55d066c6c77a6638f7d6387f999985cbac08492e5b` | research (30+ minutes apart, cache-busting for files); check 23:19Z |
| `in_poi_kalam_first_day_release_20020725` | 7,353 | `0cb19059457be5f6166b6f3405fc10c5063ccd1fb59857c367f3fb0ed7513a73` | research (30+ minutes apart, cache-busting for files); check 23:19Z |
| `in_gazette_ext_no723_so849e_20020813` | 39,931 | `86135c39e1597fbb6dbe7bc9ac53b0d4e069f75c429eef6ff2c908f6a127de74` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:17Z |
| `in_gazette_ext_eci_on85_20070616` | 373,904 | `cf0e3a33eebd0ae9b6c03c8314f5f50abaadaf2a0cdc67d728e0f29b6a091812` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_gazette_ext_no891_so1193e_20070721` | 41,704 | `e4c73073c7123d4c9d54c1376a7ead7fd3fd72825f613605daf6d053312f8f08` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_gazette_ext_no901_so1205e_20070724` | 38,987 | `c67af9716e5e1f68057ba5773c2f738b78c53f53c6676d013ced994a233f1366` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_gazette_ext_no903_so1207e_20070725` | 128,992 | `0ae199eefcdb35e5d6070638fa72e85db6d8cf6f27a4194b33ce0fcf9139d8dd` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_poi_patil_assumption_speech_20070725` | 10,882 | `685218e368e1f5fc32c965cada9c0a8b6f2a60c5cb2048b79d3f046448a4eea2` | research (30+ minutes apart, cache-busting for files); check 23:19Z |
| `in_poi_swearing_in_page_patil` | 2,904 | `e7f97344f841ad19ab2c36cbc000642f006da9d30a35680003877667852a410e` | research (30+ minutes apart, cache-busting for files); check 23:19Z |
| `in_pp_profile_former_president_patil` | 9,030 | `f2b67138e9048eec5369598b4e5e24b61f33b415927976a2cc2e6532c940e429` | research (30+ minutes apart, cache-busting for files); check 23:20Z |
| `in_gazette_ext_eci_on45_20120616` | 230,151 | `d30202f8c8f0f57a685b984292ec2d0e3bc8d566c740c29a91cf7d1cd0717d57` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_gazette_ext_no1364_so1657e_20120722` | 22,147 | `a77e15dc1fb4ea3e8dcd9e66b52d93c824f34351b46de907959756ebb0f38780` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_gazette_ext_no1376_so1672e_20120724` | 24,435 | `9718610dd0ea9fc32701e30a6c94a082f19f5541fa23cc2e24ad1268fb78f6c6` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_gazette_ext_no1389_so1687e_20120725` | 52,566 | `b233ae369a5c1bea3d7070e6d4c8f6ec3674cea9eff5e7b55e1ec0a5d2fb10b9` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_gazette_ext_no1425_so1724e_20120731` | 17,687 | `10ba73725c43b5c17366ab4a6a8e58906059b334f652426a2a71f90dfdab04df` | research (30+ minutes apart, cache-busting for files); check 23:17Z, 23:18Z |
| `in_poi_pranab_assumption_speech_20120725` | 12,598 | `0352665a7a57a3d7b66f6b8857b3af902f80f5e4db997484282299d40393d969` | research (30+ minutes apart, cache-busting for files); check 23:18Z |
| `in_poi_swearing_in_page_pranab` | 5,595 | `e8a2e6fe6c17a90d793b3de2ff6475b66c93fe19870b5d173a11d987c4fb0e1f` | research (30+ minutes apart, cache-busting for files); check 23:20Z |
| `in_gazette_ext_eci_on38_20170614` | 1,380,182 | `11fb2a53dd349f7f7101d38d1a90a9f92bdb969dc991857762b8955ed6bbd2e1` | research (30+ minutes apart, cache-busting for files); check 23:09Z (cache-busting) |
| `in_gazette_ext_so2282_declaration_20170720` | 1,266,561 | `bce8741ce227ea19643b9068765a4654f7c66d12e0575df07b8e939ff9d314c6` | research (30+ minutes apart, cache-busting for files); check 23:08Z (cache-busting) |
| `in_gazette_ext_so2309_ceremony_20170724` | 1,265,680 | `27e1ef83a9e7b118aec453620477c0ddeb9608a8dd2b327b874cddcd528ccb64` | research (30+ minutes apart, cache-busting for files); check 23:08Z (cache-busting) |
| `in_pib_pranab_eve_of_demitting_office_20170724` | 14,661 | `0b62ff975909eebb5d1fb3fa5ecfb1e0ca05605db917c5fed78b8264bf646233` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_rb_pranab_farewell_address_20170724` | 94,872 | `f6cb0331448eb3853382dec8b6f8264222fbc140dac25a1f1e7171b7a0809bfa` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_gazette_ext_so2318_assumption_20170725` | 1,356,195 | `7e0a5bbe67e01961938e41d4a820715b598399816c0d91cc47534a3e844f487e` | research (30+ minutes apart, cache-busting for files); check 23:09Z (cache-busting) |
| `in_rb_kovind_assumption_speech_20170725` | 90,535 | `6643fae1fc1db4d35926f3ffd85cf38c7037555d0e14465a62fc8835f54d488b` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_pib_kovind_profile_20170725` | 9,699 | `1670c76e15ce1fcbdf34a5145db19ae3e40e95339c893dce4a929f4ada75c425` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_pib_photo_20170725_oath` | 56,334 | `710516928416ec4934a592e268e65e80b0a8bab2c90e1dd03cc770b7da0eff8f` | check about 23:46Z and 23:49Z; packet 00:19Z, 00:21Z and 00:27Z on 25 Sep |
| `in_gazette_ext_so2487_corrigendum_20170804` | 1,275,649 | `9098b6d29b80bb58dc6be4c076d20a8a3daf8fe3d77ebe55c5dc643e300cb045` | check 23:17Z, 23:18Z and 23:52Z (cache-busting); packet 00:03Z on 25 Sep |
| `in_gazette_ext_eci_on40_20220615` | 1,095,857 | `3ed805e2f26c7ac33c9052c02fefc879dcc15987d6c6656b6ef9c3a6ae5f7f79` | research (30+ minutes apart, cache-busting for files); check 23:10Z (cache-busting) |
| `in_gazette_ext_so3325_declaration_20220722` | 1,069,378 | `728485253cece6c57f1b95d6ffdb1b9fc2d119b798cab5a7a16ef452bf546587` | research (30+ minutes apart, cache-busting for files); check 23:09Z (cache-busting) |
| `in_gazette_ext_so3365_ceremony_20220724` | 845,105 | `12303213746e18aee2de8c79007dab2f86ae048e387e813acd307af9272a59f1` | research (30+ minutes apart, cache-busting for files); check 23:09Z (cache-busting) |
| `in_pib_kovind_farewell_address_20220724` | 114,788 | `485dbf2399e2544f1c0d11ec200a466f86d3ba91c36bb209ba7525fccf31b166` | check about 23:43Z, 23:45Z and 23:52Z; packet 00:19Z, 00:21Z and 00:27Z on 25 Sep |
| `in_gazette_ext_so3373_assumption_20220725` | 1,151,852 | `20256cd28f8d5848b356a27806b352985e3f809f8c5d2f945feb89882386688c` | research (30+ minutes apart, cache-busting for files); check 23:09Z (cache-busting) |
| `in_rb_murmu_assumption_address_20220725` | 67,544 | `deaf87826586768663ec8956efcceab3781492768ccefe4b1c7af26f742ec22f` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_rb_photo_gallery_20220725` | 72,793 | `956acf2c17783230a2560bf713b5703294866659ef64f7dadf1736da067f118d` | check about 23:50Z and 23:51Z; packet 00:19Z, 00:21Z and 00:27Z on 25 Sep |
| `in_pib_pmo_murmu_took_oath_20220725` | 48,168 | `6d8b43fa00fd482010b5f8292d42c76dcaabbb5e5d1f31b907294038bdbb2dba` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_rb_murmu_mozambique_delegation_20220729` | 42,823 | `f281a1c5540a18c168bef6cb407f1da6652c4913e41cc7a23c973f460544eff1` | research (30+ minutes apart, cache-busting for files); check 23:10Z |
| `in_gazette_ext_ordinance_2_2026_20260605` | 329,484 | `d3cc622530042291a5b8dcd6ebc6bca2366da8aaab596cccf8e0d3e2e4aba18e` | research (30+ minutes apart, cache-busting for files); check 23:10Z (cache-busting) |
| `in_rb_murmu_independence_day_address_20260814` | 623,615 | `aed4c60f6792dc2ea391cd02be29fa6dfe0fd3b3d61da3883d07c82996146d19` | research (30+ minutes apart, cache-busting for files); check 23:10Z (cache-busting) |

Reproducibility traps met and avoided:

- The live presidentofindia.nic.in and presidentofindia.gov.in pages (Drupal) regenerate view tokens per request: a second
  live download with a cache-busting query changed both the byte count and the SHA-256. Every Secretariat page is
  therefore recorded from a raw Internet Archive capture made before the cutoff; the one direct presidentofindia.gov.in
  identity is a static PDF attachment.
- Live PIB release pages inject a per-request token (two requests for relid 85462 twelve seconds apart gave the same size
  and different SHA-256s), so PIB is recorded only from raw captures.
- The eGazette server now answers only the lower-case `writereaddata` path for the 2017-2026 files (the capitalised path
  given in the Internet Archive metadata returns 404), while the 1990-2012 files are served at `WriteReadData`; each URL is
  recorded exactly as it was downloaded.
- Web archive requests were intermittently refused or answered with 429 or 503 pages; those bodies were discarded, and
  no recorded identity is such a page. The capture of 31 July 2022 of the Secretariat's release 2191 redirects to a 503
  capture, so the capture of 15 August 2022 is recorded.

## Leads not imported

- President's Act No. 1 of 1990, Gazette Extraordinary of 10 January 1990
  (https://egazette.gov.in/WriteReadData/1990/E-0576-1990-0004-24040.pdf, 95,736 bytes, SHA-256 `7ef31d8a0030deb5...`):
  signed by R. Venkataraman, but a statute text, which gives procedure only and never a date (check A1).
- Gazette Extraordinary of 25 July 1987, Resolution S.O. 741(E) and Proclamation of R. Venkataraman's assumption of office
  (https://egazette.gov.in/WriteReadData/1987/E-0718-1987-0382-31802.pdf, 123,572 bytes, SHA-256 `2cc28038142851c2...`,
  stable across two downloads by the Part A check): before the period, like the pre-1990 starts CLAUDE-C01-11 left
  unimported (check A10).
- R. Venkataraman's memoir "My Presidential Years" (https://archive.org/details/mypresidentialye0000venk): a personal
  retrospective account.
- Rajya Sabha Debates of 25 July 1997, pp. 1-3 (the store file ID_181_25071997_01_p1-3_1.pdf, 138,349 bytes): members
  object that the new President's speech omitted a former President's name; the new President is not named.
- Gazette Extraordinary of 26 June 1997 (list of candidates) and Lok Sabha Debates of 17 and 27 July 1992
  (https://archive.org/details/eparlib.nic.in.3167): not needed for the result or the oath.
- The Election Commission's archived "Presidential Election 2002" page
  (https://web.archive.org/web/20101009122526id_/http://eci.nic.in/eci_main/miscellaneous_statistics/presidential_elec2002.asp,
  22,526 bytes, SHA-256 `39ea8363c7d915a8...`; and its 2012 copy, 29,649 bytes, `49f06518b5af0dea...`): a finding aid whose
  linked results PDF was never captured (checks B9 and B10).
- PIB releases read live but never captured, whose live pages change per request: relid=85462 and relid=85503 (2012),
  relid=29331 and relid=29379 (2007), relid=85501, relid=85464, relid=29367, relid=29342 and relid=29343 (farewells of
  2007 and 2012). Each repeats a Gazette notice or a Secretariat page recorded here.
- Duplicates of recorded pages: PIB relid=168957 and relid=168956 (Kovind's 2017 address), relid=168868 (announcement of
  the 2017 farewell), PRID=1844557 (Murmu's 2022 address), the President's website profile of 27 July 2017 (capture
  20170727221652), Murmu's 2022 profile (capture 20220920170715, not retrievable), the 2012 address PDF (sp250712.pdf) and
  the 2022 album capture of 15 August 2022.
- Ram Nath Kovind's archived website (ramnathkovind.nic.in: sp240722.html, last modified 26 November 2022, and
  sp250717.html): later copies of the 2022 farewell and the 2017 address; PIB's same-day release is recorded instead
  (check C6).
- PIB's day indexes of 18 and 25 July 2002 (r18072002.html, r25072002.html) and the Secretariat's July 2002 index
  (pressrel_july02.htm, with pr409 of 21 July 2002): finding aids and a minister's message.
- The Supreme Court (Number of Judges) Amendment Ordinance, 2026 of 16 May 2026 (Internet Archive item 272639): a second
  presidential ordinance; the later one is recorded.
- Current presidentofindia.gov.in pages (for example presidentofindia.gov.in/shri-pranab-mukherjee and /former-presidents):
  per-request Drupal pages, and 2026 captures are mostly 403 challenge pages; retrospective in any case.
- The wikipedia article on the 2012 presidential election, with news reports of the result and the ceremony: leads only,
  not consulted as evidence.

## Sources attempted

- Election Commission of India (eci.gov.in): HTTP 406 ("Request was blocked due to suspicious behavior") to scripted
  requests; not bypassed. eci.nic.in no longer resolves. The Returning Officers' declarations were read in the Gazette.
- Parliament Digital Library (eparlib.nic.in, eparlib.sansad.in): DNS failure or connection timeout. Lok Sabha records are
  taken from Internet Archive item copies, disclosed as mirror-only.
- PIB archive (archive.pib.gov.in): connections refused. The Part A check found that the web archive does list its
  President's Secretariat folders, but they hold only RPB-1990-12-14_109.pdf for 1990, nothing for 1992 and only
  RPB-1997-03-09_026.pdf for 1997, so no July 1992 or July 1997 release is captured (check A6). PIB's old English release
  archive in the web archive starts in 1998.
- The Journal of Parliamentary Information: no 1997 or 1998 issue is on the Internet Archive.
- Home Ministry notification S.O. 788(E) of 25 July 2002: not in the eGazette or either Internet Archive Gazette
  collection.
- The President's website speech archive for Narayanan and Venkataraman (404); the Secretariat's release 2190 of July 2022
  (the capture redirects to a 503 page); Murmu's 2022 profile (repeated failures); PIB's 25 July 2017 swearing-in release
  (no capture in any URL form checked); PIB release IDs 1844500-1844599 of 25 July 2022 (no swearing-in release).
- eGazette files of the 2002, 2007 and 2012 Election Commission notices under their eGazette IDs (404); the 2018
  Public.Resource.Org copies in the Internet Archive are recorded.

No site terms, licences or cookie banners were accepted, no CAPTCHA was met, and no login was used.

## Checker defects

| # | Defect | Outcome |
|---|---|---|
| A1 | `in_venkataraman_signs_presidents_act_19900110` dated a statute text as the earliest attestation | **Resolved by removal**: the claim and its source are leads; the 13 January 1990 Order dates the holder |
| A2 | IN-PRES-03 missed the Lok Sabha Debates of 25 July 1997 | **Applied**: `in_ls_debates_19970725` imported mirror-only with `in_narayanan_assumption_recalled_in_lok_sabha_19970725` (`assumption_recalled_in_debate`, a member's statement, never a holder date); the translated line at column 266 is not used; the unresolved text and the 1997 programme's uncertainty are reworded; the "outgoing President" remark is noted for IN-PRES-09 |
| A3 | Holder names used source spellings | **Applied**: every row uses one name per holder ("Shankar Dayal Sharma" as in the handoff, the 1997 notice and the President's website; "K. R. Narayanan" as in the handoff and the C01-11 style); printed forms stay in the texts |
| A4 | Event kinds differed from the C01-11 vocabulary | **Applied**: `retrospective_term_span` with "no structured date is stored", `oath_of_office` and `assumption_statement`; span claims omit `attested_on` in india.json and carry null in the rows |
| A5 | The count claim said Table II was not reviewed | **Applied**: locator PDF pages 50-51 (printed 337-338), and the uncertainty says Table II was checked |
| A6 | The PIB note gave the wrong reason | **Applied**: Sources attempted records the web archive's listing and its contents |
| A7 | IN-PRES-02 uses six sources, not five | **Applied**: six sources |
| A8 | Sources lacked `source_type`, `rights_note`, `scope_note` and snapshots | **Applied**: every source carries them in the C01-11 form |
| A9 | Missing record: Lok Sabha Debates, 25 July 1997 | **Applied**: imported (see A2); re-downloaded by this packet 50 minutes after the check with the same bytes |
| A10 | Missing record (optional): the 1987 Gazette of Venkataraman's assumption | **Declined**: before the period; a `from` of 1987 would precede the packet's coverage period, and CLAUDE-C01-11 left pre-1990 starts unimported; recorded as a lead with its identity |
| B1 | `in_pranab_ceases_ls_member_on_assuming_office_20120725` used as a start | **Applied**: renamed `in_pranab_ls_membership_ceased_wef_20120725`, kind `lok_sabha_membership_ceased_on_assumption`, off the holder; the note cites S.O. 1724(E) of 30 July 2012 (Gazette 31 July) as corroboration only |
| B2 | The vote figures' paraphrase misplaced the percentage | **Applied**: 89.58 per cent in the announcement, the figures in reply to a member, called "votes" |
| B3 | Event kinds `oath` and `assumption_of_office*` | **Applied**: the 2002 release and the 2012 caption are `oath_of_office`; the Patil caption `swearing_in_recalled_retrospective`; the reference in Mukherjee's address `oath_referred_in_address` (never a holder); resolutions and same-day addresses `assumption_statement`; proclamations `assumption_proclaimed`; the Patil profile `assumption_recalled_retrospective` |
| B4 | `holder_name` used printed forms | **Applied**: one name per holder in every row; the printed forms, including "Abul" [sic], stay in the texts |
| B5 | Row fields | **Applied**: `observation_id` `in_presidency`, `review_observation` IN-PRES-nn, `role_id` and `role_title` |
| B6 | Kalam's start rested only on a corrigendum's recital | **Applied**: his same-day speech is the `from` claim; the recital is renamed `in_kalam_assumption_recited_in_corrigendum_20020725`, dated 25 July 2002 as it states, and kept off the holder; the fallback is dropped |
| B7 | The Patil profile's publisher | **Applied**: the office of the former President, NIC-hosted |
| B8 | The Kalam speech locator and the 2002 salutation | **Applied**: second paragraph of the speech body (fourth `<p>` element); the uncertainty and the institution coverage record the unnamed Chief Justice in the salutation |
| B9 | Missing record: the Election Commission's archived 2002 page | **Declined**: a finding aid with no claim to extract; its linked results PDF was never captured; recorded as a lead with its identity |
| B10 | Missing record: the same page's 2012 copy | **Declined**: as B9 |
| C1 | `in_rb_states_murmu_assumed_office_20220725` supported Murmu's start | **Applied in part**: moved off the holder and renamed `in_rb_murmu_assumption_recalled_20220725`, kind `assumption_recalled_retrospective`, never a holder date; its date is kept as the day it recalls, following the stacked packet's convention for recollections (`in_pmo_vajpayee_took_charge_recalled_19991013`; "A retrospective statement is dated by the day it recalls"), which the check misread: only retrospective spans carry no date |
| C2 | IN-PRES-07's oath was said to be attested only as scheduled | **Applied**: PIB's photo gallery imported with the oath by J.S. Khehar (`oath_of_office`) and the register signing; IN-PRES-07 accepted |
| C3 | IN-PRES-08 said no record names who administered the oath | **Applied**: the Secretariat's photo gallery imported with the oath by N.V. Ramana; IN-PRES-08 accepted |
| C4 | The corrigendum to Kovind's name was missing | **Applied**: `in_kovind_name_corrigendum_20170804`, cited in the resolution claims' uncertainty and the holder note; no date changes |
| C5 | The 1992 and 1997 notices were said not to exist | **Applied**: O.N. 65(E) of 1997 imported (IN-PRES-09 and the 1997 poll dates); O.N. 49(E) of 1992 was already recorded by Part A; the unresolved text and the failed-search note are corrected |
| C6 | Kovind's 2022 farewell was recorded from a later copy | **Applied**: PIB PRID 1844440 of 24 July 2022 is the recorded source; the later copy is a lead |
| C7 | `oath_scheduled` folded the ceremony into the oath | **Applied**: all six programmes are `assumption_ceremony_scheduled` |
| C8 | Missing record: corrigendum S.O. 2487(E) | **Applied**: imported (see C4) |
| C9 | Missing record: PIB photo gallery, 2017 | **Applied**: imported (see C2), with Mukherjee's same-day styling as former President as an IN-PRES-09 claim |
| C10 | Missing record: the Secretariat's photo gallery, 2022 | **Applied**: imported (see C3), with Kovind's same-day styling as former President as an IN-PRES-09 claim |
| C11 | Missing record: O.N. 49(E) of 1992 (Internet Archive copy) | **Already recorded**: `in_gazette_ext_eci_on49_19920610`, the official eGazette file with the same bytes and SHA-256 |
| C12 | Missing record: O.N. 65(E) of 1997 | **Applied**: `in_gazette_ext_eci_on65_19970609` |
| C13 | Missing record: PIB PRID 1844440 of 2022 | **Applied**: `in_pib_kovind_farewell_address_20220724` (see C6) |

The Part B check's note that the 25 July 2002 release calls Narayanan "ex-President" is applied as the IN-PRES-09 claim
`in_narayanan_styled_ex_president_20020725`.

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-India-PRES-002`: the day each President's office ended, 1992-2022, if any record states it (the President's
  Secretariat's communiques in the PIB archive, from a network that can reach archive.pib.gov.in).
- `C01-India-PRES-003`: the text of the Ministry of Home Affairs notification S.O. 788(E) of 25 July 2002 and the Chief
  Justices who administered the 1997, 2002, 2007 and 2012 oaths (Lok Sabha and Rajya Sabha records of those days).
- `C01-India-PRES-004`: the Election Commission's own result records and vote values for 1992-2022 (eci.gov.in).
- `C01-India-VP-001`: the vice-presidency, 1990-2026, including any acting service (outside this packet).

## Integration notes (outside this packet's file boundary)

- **Stack, base and claim:** this branch is stacked on `claude/c01-in-11` at `538920f1` (CLAUDE-C01-11, ready for review),
  which is based on `ffe54b02` (`codex/campaign-certification`); the claim commit `f755794c` holds only the handoff. Merge
  CLAUDE-C01-11 first: both packets edit `india.json`, and this packet only appends to it (a second institution after
  `in_prime_minister`, 56 sources after the C01-11 sources, and one packet coverage note after the C01-11 note).
  `origin/claude/c01-in-11` had no commits beyond `538920f1` when this packet was written, so no merge was needed.
- `research-index.json` is the only file this packet shares with other pending packets. It is regenerated in a
  **separate commit**. New totals against `538920f1`: 287 sources and 1,958 claims (previously 231 and 1,874), 29
  institution observations (previously 28). India now has two institutions, two role observations and 84 entries pending
  mapping; its nine discovery batches become eight of 10 members and one of 4, so the total of 92 batches is unchanged.
  The index must be regenerated after any other pending packet merges.
- Pinned tests in `test_india_research_s10e.py`, none loosened and no assertion removed:
  - counts (entries, sources, claims, roles) are now (84, 128, 278, 2), from (83, 72, 194, 1);
  - exactly two institutions, `in_prime_minister` with role `in_pm` and then `in_presidency` with role `in_president`;
  - the 56 new sources are pinned to their five hosts (the web archive, archive.org, egazette.gov.in,
    cms.rajyasabha.nic.in and www.presidentofindia.gov.in) and the 2026-09-24 access date, and their rows to the
    presidency; the C01-11 sources keep their own hosts, access dates and rows;
  - the source list is the two original sources, then the C01-11 sources, then the C01-15 sources;
  - `mapping_pending` is 84 (from 83) and the India work orders are eight of 10 members and one of 4 (from 3), with the
    last batch pinned.
- Pinned values in `test_india_prime_ministers_c01_11.py`, none loosened: the institution list is `in_prime_minister`
  followed only by `in_presidency`; entries and roles (84, 2); the source list and the rows beyond C01-11's are pinned to
  the presidency's sources and claims (56 and 84); the packet coverage has nine notes with C01-11's at index 7; the index
  has two institution and two role observations; and the guard that no presidentofindia, pmindia or PIB page is a direct
  identity now allows exactly one pinned static PDF attachment (`in_rb_murmu_independence_day_address_20260814`). Its
  assertions on its own records are unchanged, and its barred event kinds and lead markers apply to the new rows too.
- `test_campaign_census` needs `spheres-sim/data`, which is absent from this sparse worktree; it was not run here, and the
  sparse checkout was not widened.
- The atlas (`tools/ui/leadership-research-review.js`) shows "Observed on" only for a holder with `attested_on`; no
  president holder has both `attested_on` and `until`. No UI code changed.
- The presidency's event-kind vocabulary is pinned in `test_india_presidents_c01_15.py`.
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
